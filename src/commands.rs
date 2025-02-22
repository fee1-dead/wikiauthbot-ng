use crate::Command;

mod auth;
mod deauth;
mod guilds;
mod misc;
mod revwhois;
pub mod whois;

pub use auth::handle_successful_auth;

pub fn all_commands() -> Vec<Command> {
    vec![
        auth::auth(),
        deauth::deauth(),
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
        revwhois::revwhois(),
        whois::whois(),
        whois::whois_menu(),
        whois::whois_bench(),
    ]
}
