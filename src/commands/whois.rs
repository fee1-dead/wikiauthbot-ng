use std::cmp::Reverse;

use color_eyre::eyre::{Context as _, OptionExt};
use poise::CreateReply;
use serenity::all::{Mention, User, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedFooter};
use wikiauthbot_db::{msg, DatabaseConnectionInGuild, WhoisResult};

use crate::{Context, Result};

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

impl WhoisInfo {
    pub async fn create_embed(
        mut self,
        discord_user_id: UserId,
        db: DatabaseConnectionInGuild<'_>,
    ) -> Result<CreateEmbed> {
        let mention = Mention::User(discord_user_id).to_string();
        let registration = self
            .registration
            .split_once("T")
            .ok_or_eyre("invalid date")?
            .0;

        let global_groups = if !self.groups.is_empty() {
            let mut msg = msg!(
                db,
                "whois_global_groups",
                groupslist = self.groups.join(", ")
            )?;
            msg.to_mut().push('\n');
            msg
        } else {
            "".into()
        };

        let mut edits = 0;

        // TODO introduce if servers want to remove users who are indeffed
        // let mut indeffed = false;
        self.merged.sort_by_key(|w| Reverse(w.editcount));
        let mut fields = Vec::new();
        let mut blocks = Vec::new();
        for wiki in self.merged.into_iter().filter(|w| w.editcount > 0) {
            let mut content = msg!(db, "whois_edits", edits = wiki.editcount)?;
            let mut inline = true;
            edits += wiki.editcount;
            if !wiki.groups.is_empty() {
                let content = content.to_mut();
                content.push('\n');
                content.push_str(&msg!(
                    db,
                    "whois_groups",
                    groupslist = wiki.groups.join(", ")
                )?);
                inline = false;
            }

            fields.push((wiki.wiki.clone(), content, inline));
            blocks.push(wiki.blocked.zip(Some(wiki.wiki)));
        }

        let mut whois = msg!(
            db,
            "whois",
            mention = mention,
            registration = registration,
            home = self.home,
            global_groups = global_groups,
            edits = edits,
        )?;

        let mut blocked = false;
        let btext = msg!(db, "whois_blocked")?;
        let ltext = msg!(db, "whois_locked")?;
        let mb = whois.to_mut();
        for (block, wiki) in blocks.into_iter().flatten() {
            if !blocked {
                mb.push_str(&format!(
                    "\n\n<:declined:359850777453264906> {btext}{}\n",
                    if self.locked {
                        format!(" {ltext}")
                    } else {
                        String::new()
                    }
                ));
                blocked = true;
            }
            let reason = if block.reason.is_empty() {
                &*db.get_message("whois_no_block_reason").await?
            } else {
                &block.reason
            };
            mb.push_str(&format!("**{wiki}** ({})\n", block.expiry));
            mb.push_str(&format!("__{reason}__\n"));
        }

        if !blocked && self.locked {
            mb.push_str(&format!("\n\n<:declined:359850777453264906> {ltext}\n"));
        }

        let date =
            chrono::DateTime::parse_from_rfc3339(&self.registration).context("invalid date")?;

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

        let user_link = db.user_link(&self.name).await?;
        let mut embed = CreateEmbed::new()
            .colour(0xCCCCCC)
            .title(self.name)
            .description(whois)
            .url(user_link)
            .thumbnail(format!(
                "https://upload.wikimedia.org/wikipedia/commons/{medal}"
            ))
            .fields(fields.by_ref().take(10));

        if fields.next().is_some() {
            embed = embed.footer(CreateEmbedFooter::new(
                db.get_message("whois_overflow").await?,
            ));
        }

        Ok(embed)
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

pub async fn whois_impl(ctx: Context<'_>, user_id: UserId) -> Result {
    let crate::Data { client, .. } = ctx.data();
    let db = ctx.data().db_guild(&ctx);

    if !db.has_server_settings() {
        ctx.reply("this server has not been setup. Please contact dbeef for setup assistance.")
            .await?;
    }

    if db.whois_is_ephemeral() {
        ctx.defer_ephemeral().await?;
    } else {
        ctx.defer().await?;
    }

    let user = user_id.get();
    let whois = db.whois(user).await?;

    let Some(WhoisResult { wikimedia_id }) = whois else {
        ctx.reply(db.get_message("whois_no_user_found").await?)
            .await?;
        return Ok(());
    };

    let whois: WhoisInfo = fetch_whois(client, wikimedia_id).await?;

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .embed(whois.create_embed(user_id, db).await?),
    )
    .await?;

    Ok(())
}

#[poise::command(context_menu_command = "Get whois", ephemeral, guild_only = true)]
pub async fn whois_menu(ctx: Context<'_>, user: User) -> Result {
    whois_impl(ctx, user.id).await
}

#[poise::command(slash_command, ephemeral, guild_only = true)]
/// Check account details for an authenticated member
pub async fn whois(
    ctx: Context<'_>,
    // TODO i18n description of commands
    #[description = "User to check, leave blank for yourself"] user: Option<UserId>,
) -> Result {
    whois_impl(ctx, user.unwrap_or_else(|| ctx.author().id)).await
}
