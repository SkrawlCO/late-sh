use crate::session_bootstrap::SessionOrigin;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DoorContext {
    pub user_id: Uuid,
    pub player_name: String,
    pub session_origin: SessionOrigin,
}

impl DoorContext {
    pub fn bbs_mode(&self) -> bool {
        matches!(
            self.session_origin,
            SessionOrigin::Bbs(_)
        )
    }
}
