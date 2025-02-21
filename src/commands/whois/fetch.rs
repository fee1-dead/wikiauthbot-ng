use std::cmp::Reverse;

use color_eyre::eyre::{Context as _, OptionExt};
use serenity::all::{Mention, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use wikiauthbot_common::mwclient_with_url;
use wikiauthbot_db::{DatabaseConnectionInGuild, msg};

use crate::Result;

pub struct MedalInfo {
    days: u64,
    edits: u64,
    /// starts with `https://upload.wikimedia.org/wikipedia/commons/`,
    /// prefix not included
    url: &'static str,
}

#[rustfmt::skip]
const MEDALS : &[MedalInfo] = &[
    MedalInfo { days: 6570, edits: 150000, url: "1/12/Editor_-_lapis_matter_iv.jpg" },
    MedalInfo { days: 5840, edits: 132000, url: "9/91/Editor_-_lapis_matter_iii.jpg" },
    MedalInfo { days: 5110, edits: 11400, url: "6/6e/Editor_-_lapis_matter_ii.jpg" },
    MedalInfo { days: 4380, edits: 96000, url: "8/85/Editor_-_lapis_philosophorum_superstar.jpg" },
    MedalInfo { days: 3650, edits: 78000, url: "0/00/Editor_-_orichalcum_star.jpg" },
    MedalInfo { days: 2920, edits: 60000, url: "7/7a/Editor_-_bufonite_star.jpg" },
    MedalInfo { days: 2555, edits: 51000, url: "d/dd/Editor_-_platinum_star_II.jpg" },
    MedalInfo { days: 2190, edits: 42000, url: "8/86/Editor_-_platinum_star_I.jpg" },
    MedalInfo { days: 1825, edits: 33000, url: "4/4a/Editor_-_rhodium_star_III.jpg" },
    MedalInfo { days: 1642, edits: 28500, url: "1/1f/Editor_-_rhodium_star_II.jpg" },
    MedalInfo { days: 1460, edits: 24000, url: "9/9a/Editor_-_rhodium_star_I.jpg" },
    MedalInfo { days: 1277, edits: 20000, url: "0/0f/Editor_-_gold_star.jpg" },
    MedalInfo { days: 1095, edits: 16000, url: "0/06/Editor_-_silver_star.jpg" },
    MedalInfo { days: 912, edits: 12000, url: "7/7b/Editor_-_bronze_star.jpg" },
    MedalInfo { days: 730, edits: 8000, url: "5/53/Editor_-_iron_star.jpg" },
    MedalInfo { days: 547, edits: 6000, url: "c/cd/Editor_-_gold_ribbon_-_3_pips.jpg" },
    MedalInfo { days: 365, edits: 4000, url: "c/c2/Editor_-_silver_ribbon_-_2_pips.jpg" },
    MedalInfo { days: 182, edits: 2000, url: "6/67/Editor_-_bronze_ribbon_-_1_pip.jpg" },
    MedalInfo { days: 91, edits: 1000, url: "f/f3/Editor_-_blue_ribbon_-_0_pips.jpg" },
    MedalInfo { days: 30, edits: 200, url: "e/e7/Editor_-_white_ribbon_-_0_pips.jpg" },
    MedalInfo { days: 23, edits: 150, url: "7/74/Registered_editor_badge_with_tildes.jpg" },
    MedalInfo { days: 15, edits: 100, url: "thumb/e/ef/Registered_Editor_lv2.svg/222px-Registered_Editor_lv2.svg.png" },
    MedalInfo { days: 8, edits: 50, url: "thumb/1/11/Registered_Editor_lv3.svg/222px-Registered_Editor_lv3.svg.png" },
    MedalInfo { days: 0, edits: 0, url: "thumb/8/86/Registered_Editor_lv4.svg/222px-Registered_Editor_lv4.svg.png" },
];

#[derive(serde::Deserialize)]
pub struct BlockInfo {
    reason: String,
    expiry: String,
}

#[derive(serde::Deserialize)]
pub struct WikiInfo {
    wiki: String,
    url: String,
    blocked: Option<BlockInfo>,
    editcount: u64,
    #[serde(default)]
    groups: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct WhoisInfo {
    pub name: String,
    // medal_url: &'static str,
    registration: String,
    home: String,
    // editcount: u64,
    groups: Vec<String>,
    // indeffed: bool,
    #[serde(default)]
    locked: bool,
    merged: Vec<WikiInfo>,
}

#[derive(serde::Deserialize)]
pub struct BlockFlags {
    partial: bool,
}

pub struct EmbeddableWikiInfo {
    wiki: String,
    editcount: u64,
    groups: Vec<String>,
}

pub struct EmbeddableBlockInfo {
    wiki: String,
    reason: String,
    expiry: String,
    partial: bool,
}

#[derive(Clone, Copy)]
pub enum EmbeddableBlockKind {
    NotBlocked,
    PartiallyBlocked,
    FullyBlocked,
}

pub struct EmbeddableWhois {
    discord_user_id: UserId,
    name: String,
    registration: String,
    groups: Vec<String>,
    /// sorted from most edits to least edits; must have at least one edit; must not be more than 10
    wikis: Vec<EmbeddableWikiInfo>,
    blocked: EmbeddableBlockKind,
    locked: bool,
    blocks: Vec<EmbeddableBlockInfo>,
    home: String,
    edits: u64,
    overflowed: bool,
}

impl EmbeddableWhois {
    pub fn create_embed(self, db: DatabaseConnectionInGuild<'_>) -> Result<CreateEmbed> {
        let EmbeddableWhois {
            discord_user_id,
            name,
            registration,
            groups,
            wikis,
            blocked,
            locked,
            blocks,
            home,
            edits,
            overflowed,
        } = self;
        let mention = Mention::User(discord_user_id).to_string();
        let registration = registration.split_once("T").ok_or_eyre("invalid date")?.0;
        let global_groups = if !groups.is_empty() {
            let mut msg = msg!(db, "whois_global_groups", groupslist = groups.join(", "))?;
            msg.to_mut().push('\n');
            msg
        } else {
            "".into()
        };

        let fields = wikis.into_iter().map(
            |EmbeddableWikiInfo {
                 wiki,
                 editcount,
                 groups,
             }| {
                // we're just YOLO'ing here because I like it to be an iterator
                let mut content = msg!(db, "whois_edits", edits = editcount).unwrap();
                let mut inline = true;
                if !groups.is_empty() {
                    let content = content.to_mut();
                    content.push('\n');
                    content.push_str(
                        &msg!(db, "whois_groups", groupslist = groups.join(", ")).unwrap(),
                    );
                    inline = false;
                }
                (wiki, content, inline)
            },
        );

        let mut whois = msg!(
            db,
            "whois",
            mention = mention,
            registration = registration,
            home = home,
            global_groups = global_groups,
            edits = edits,
        )?;
        let mb = whois.to_mut();

        let icon = match (blocked, locked) {
            (_, true) | (EmbeddableBlockKind::FullyBlocked, false) => {
                "<:declined:359850777453264906>"
            }
            (EmbeddableBlockKind::PartiallyBlocked, false) => "<:possilikely:936065888237547541>",
            (EmbeddableBlockKind::NotBlocked, false) => "",
        };

        let pblocked = msg!(db, "whois_pblocked")?;

        if !icon.is_empty() {
            let mut text = match blocked {
                EmbeddableBlockKind::FullyBlocked => pblocked.clone(),
                EmbeddableBlockKind::PartiallyBlocked => msg!(db, "whois_blocked")?,
                EmbeddableBlockKind::NotBlocked => "".into(),
            };

            if locked {
                if text.is_empty() {
                    text.to_mut().push(' ');
                }
                text.to_mut().push_str(&msg!(db, "whois_locked")?);
            }
            mb.push_str(&format!("\n\n{icon} {text}\n",));
        }

        for EmbeddableBlockInfo {
            wiki,
            reason,
            expiry,
            partial,
        } in blocks.into_iter()
        {
            let reason = if reason.is_empty() {
                &*db.get_message("whois_no_block_reason")?
            } else {
                &reason
            };
            mb.push_str(&format!(
                "**{wiki}** ({}){}\n",
                expiry,
                partial
                    .then(|| format!(" ({pblocked})"))
                    .unwrap_or_default()
            ));
            mb.push_str(&format!("__{reason}__\n"));
        }

        let date = chrono::DateTime::parse_from_rfc3339(&registration).context("invalid date")?;

        let days: u64 = chrono::offset::Utc::now()
            .signed_duration_since(date)
            .num_days()
            .try_into()?;

        let medal = MEDALS
            .iter()
            .find(|m| m.days < days && m.edits < edits)
            .unwrap_or(MEDALS.last().unwrap())
            .url;

        let mut fields = fields.into_iter();

        let user_link = db.user_link(&name)?;
        let mut embed = CreateEmbed::new()
            .colour(0xCCCCCC)
            .title(name)
            .description(whois)
            .url(user_link)
            .thumbnail(format!(
                "https://upload.wikimedia.org/wikipedia/commons/{medal}"
            ))
            .fields(fields.by_ref().take(10));

        if overflowed {
            embed = embed.footer(CreateEmbedFooter::new(db.get_message("whois_overflow")?));
        }
        Ok(embed)
    }
}

impl WhoisInfo {
    pub async fn into_embeddable(mut self, discord_user_id: UserId) -> Result<EmbeddableWhois> {
        let mut edits = 0;
        self.merged.sort_by_key(|w| Reverse(w.editcount));
        self.merged.retain(|x| x.editcount > 0);
        let mut wikis = Vec::new();
        let mut blocks = Vec::new();
        let mut blocked = EmbeddableBlockKind::NotBlocked;
        let overflowed = self.merged.len() > 10;
        for wiki in self.merged {
            edits += wiki.editcount;

            wikis.push(EmbeddableWikiInfo {
                editcount: wiki.editcount,
                groups: wiki.groups,
                wiki: wiki.wiki.clone(),
            });
            if let Some(info) = wiki.blocked {
                let blockflags = fetch_block(&wiki.url, &self.name).await?;
                let partial = blockflags.into_iter().all(|flags| flags.partial);
                match (blocked, partial) {
                    (EmbeddableBlockKind::NotBlocked, true) => {
                        blocked = EmbeddableBlockKind::PartiallyBlocked
                    }
                    (_, true) => {}
                    (_, false) => blocked = EmbeddableBlockKind::FullyBlocked,
                };
                blocks.push(EmbeddableBlockInfo {
                    expiry: info.expiry,
                    partial,
                    reason: info.reason,
                    wiki: wiki.wiki,
                });
            }
        }

        let whois = EmbeddableWhois {
            discord_user_id,
            name: self.name,
            registration: self.registration,
            groups: self.groups,
            blocked,
            blocks,
            wikis,
            locked: self.locked,
            home: self.home,
            edits,
            overflowed,
        };
        Ok(whois)
    }
}

pub async fn fetch_whois(client: &mwapi::Client, wikimedia_id: u32) -> Result<WhoisInfo> {
    let v = client
        .get_value(&[
            ("action", "query"),
            ("meta", "globaluserinfo"),
            ("guiprop", "groups|merged|unattached"),
            ("guiid", &wikimedia_id.to_string()),
        ])
        .await
        .wrap_err("querying API")?["query"]["globaluserinfo"]
        .take();

    serde_json::from_value(v).map_err(Into::into)
}

/// url in the form of `"https://en.wikipedia.org"`
pub async fn fetch_block(url: &str, name: &str) -> Result<Vec<BlockFlags>> {
    let url = format!("{url}/w/api.php");
    let v = mwclient_with_url(&url)
        .await?
        .get_value(&[
            ("action", "query"),
            ("list", "blocks"),
            ("bkusers", name),
            ("bkprop", "flags"),
        ])
        .await
        .wrap_err("querying API")?["query"]["blocks"]
        .take();

    serde_json::from_value(v).map_err(Into::into)
}
