//! Logging information in discord channels

use serenity::all::UserId;

/// All sorts of events that we want to log
#[derive(Clone, Copy, Hash)]
pub enum LogEventKind {
    /// Welcome message when someone joins
    Welcome,
    /// Message when someone authenticates
    Auth,
    /// Message when someone deauthenticates
    Deauth,
}

pub enum FullLogEvent {
    Auth {
        user_id: UserId,
        wikimedia_username: String,
    },
}
