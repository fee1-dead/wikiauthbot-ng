use core::net;

use crate::Command;

mod auth;
mod deauth;
mod guilds;
mod misc;
mod revwhois;
pub mod whois;

pub use auth::handle_successful_auth;
use wikiauthbot_common::i18n::{get_locales_map, get_message};

fn localize_command(mut c: Command) -> Command {
    let mut langs = Vec::new();
    for (lang, _) in get_locales_map() {
        // these are the only locales supported by Discord it seems
        // https://discord.com/developers/docs/reference#locales
        let discord_lang = match *lang {
            "zh-hans" => "zh-CN",
            "es" => "es-419",
            "de" | "da" | "id" | "fr" | "hr" | "it"
            | "lt" | "hu" | "nl" | "no" | "pl" | "ro"
            | "fi" | "vi" | "tr" | "cs" | "el" | "bg"
            | "ru" | "uk" | "hi" | "th" | "ja" | "ko"  => lang,
            // not covered: pt-BR, sv-SE, zh-TW, es-ES, en-GB, en-US
            "en" | _ => continue,
        };

        langs.push((*lang, discord_lang));
    }

    let name = &c.name;

    for (lang, discord_lang) in langs {
        c.name_localizations.insert(discord_lang.to_owned(), get_message(lang, &format!("cmd_{name}")).unwrap())
    }

    if c.description.is_some() {
        for (lang, discord_lang) in langs {
            c.description_localizations.insert(discord_lang.to_owned(), get_message(lang, &format!("cmd_{name}_desc")).unwrap())
        }
    }
     
    for param in &mut c.parameters {
        let param_name = &param.name;
        for (lang, discord_lang) in langs {
            param.name_localizations.insert(discord_lang.to_owned(), get_message(lang, &format!("cmd_{name}_{param_name}")).unwrap())
        }
        if param.description.is_some() {
            for (lang, discord_lang) in langs {
                param.description_localizations.insert(discord_lang.to_owned(), get_message(lang, &format!("cmd_{name}_{param_name}_desc")).unwrap())
            }
        }
    }
    c
}

pub fn all_commands() -> Vec<Command> {
    vec![
        localize_command(auth::auth()),
        localize_command(deauth::deauth()),
        guilds::add_role_rule(),
        guilds::cleanup_roles(),
        guilds::premigrate_server_check(),
        guilds::remove_role_rule(),
        guilds::setup_server(),
        guilds::unauthed_list(),
        guilds::set_server_language(),
        guilds::set_server_whois_is_ephemeral(),
        misc::register(),
        misc::debug_deauth(),
        localize_command(revwhois::revwhois()),
        localize_command(whois::whois()),
        localize_command(whois::whois_menu()),
        whois::whois_bench(),
    ]
}
