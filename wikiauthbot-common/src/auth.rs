use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;

use dashmap::DashMap;

pub struct AuthRequest {
    /// A random generated (anonymised) id for an auth request.
    id: [u8; 28],
    /// discord user id who initiated this auth request.
    discord_user_id: u64,
}

impl AuthRequest {
    pub fn new(discord_user_id: u64) -> AuthRequest {
        AuthRequest {
            id: rand::random(),
            discord_user_id,
        }
    }

    pub fn state(&self) -> impl Display {
        struct HexFmt([u8; 28]);

        impl Display for HexFmt {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                for n in self.0 {
                    write!(f, "{n:0x}")?
                }
                Ok(())
            }
        }

        HexFmt(self.id)
    }
}

pub struct AuthRequestsMap {
    in_progress: Arc<DashMap<[u8; 28], u64>>,
}

impl AuthRequestsMap {
    pub fn new() -> Self {
        Self {
            in_progress: Arc::new(DashMap::new()),
        }
    }

    /// insert a new auth request to this map (with 10 minutes expiry)
    pub fn add_auth_req(
        &self,
        AuthRequest {
            id,
            discord_user_id,
        }: AuthRequest,
    ) {
        self.in_progress.insert(id, discord_user_id);
        let map = self.in_progress.clone();
        tokio::spawn(async move {
            // 10 minutes timeout
            tokio::time::sleep(Duration::from_secs(60 * 10)).await;
            map.remove(&id)
        });
    }

    /// optionally return the discord user id associated with the auth request.
    pub fn get_auth_req(&self, state: &str) -> Option<AuthRequest> {
        if state.len() != 28 * 2 {
            return None;
        }

        let id = state
            .as_bytes()
            .chunks_exact(2)
            .map(|x| u8::from_str_radix(&String::from_utf8_lossy(x), 16))
            .collect::<Result<Vec<_>, _>>()
            .ok()?;

        self.in_progress
            .remove(&*id)
            .map(|(id, discord_user_id)| AuthRequest {
                id,
                discord_user_id,
            })
    }
}
