//! Logging information in discord channels

/// All sorts of events that we want to log
pub enum LogEventKind {
    /// Welcome message when someone joins
    Welcome,
    /// Message when someone authenticates
    Auth,
    /// Message when someone deauthenticates
    Deauth,
}
