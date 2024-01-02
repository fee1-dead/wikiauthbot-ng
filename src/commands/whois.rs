use color_eyre::eyre::{Context as _, eyre};
use serenity::all::UserId;
use wikiauthbot_db::WhoisResult;

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


pub struct WikiInfo {
    
}

pub struct WhoisInfo {
    medal_url: &'static str,
    registered_date: String,
    home: String,
    edit_count: u64,
    global_groups: Vec<String>,
    
}

#[poise::command(slash_command, ephemeral, guild_only = true)]
/// Check account details for an authenticated member
pub async fn whois(ctx: Context<'_>, #[description = "User to check, leave blank for yourself"] user: Option<UserId>) -> Result {
    let crate::Data { client, db } = ctx.data();
    ctx.defer_ephemeral().await?;

    let user = user.unwrap_or_else(|| ctx.author().id).get();

    let whois = db
        .whois(user, ctx.guild_id().ok_or_else(|| eyre!("must be in guild"))?.get())
        .await?;

    let Some(WhoisResult { wikimedia_id }) = whois else {
        ctx.reply("no user found. either the user is not in this server or is unauthenticated")
            .await?;
        return Ok(());
    };

    let v = client.get_value(&[
        ("action", "query"),
        ("meta", "globaluserinfo"),
        ("guiprop", "groups|merged|unattached"),
        ("guiid", &wikimedia_id.to_string()),
    ]).await.wrap_err("querying API")?;

    // use the reqwest client, or else
    ctx.reply(format!(
        "user {} at guild {}",
        ctx.author().id,
        ctx.guild_id().unwrap()
    ))
    .await?;
    Ok(())
}
