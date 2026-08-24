use crate::session_bootstrap::SessionOrigin;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DoorContext {
    pub user_id: Uuid,
    pub player_name: String,
    pub session_origin: SessionOrigin,
}

impl DoorContext {
    /// True when this door session originated from an upstream BBS.
    pub fn bbs_mode(&self) -> bool {
        self.session_origin.is_bbs()
    }

    /// Identity provider that originated this session.
    pub fn provider(&self) -> &'static str {
        self.session_origin.provider()
    }
}
