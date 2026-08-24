use crate::session_bootstrap::SessionOrigin;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct DoorContext {
    user_id: Uuid,
    player_name: String,
    session_origin: SessionOrigin,
}

impl DoorContext {
    /// Create a new door session identity context.
    pub fn new(user_id: Uuid, player_name: String, session_origin: SessionOrigin) -> Self {
        Self {
            user_id,
            player_name,
            session_origin,
        }
    }

    /// True when this door session originated from an upstream BBS.
    pub fn bbs_mode(&self) -> bool {
        self.session_origin.is_bbs()
    }

    /// True when this door session is a native late.sh session.
    pub fn native_mode(&self) -> bool {
        matches!(self.session_origin, SessionOrigin::Native)
    }

    /// Identity provider that originated this session.
    pub fn provider(&self) -> &'static str {
        self.session_origin.provider()
    }

    /// Stable late.sh user id for this door session.
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Player-facing display name for this session.
    ///
    /// This is intentionally session scoped. Persistent ownership remains
    /// keyed by `user_id`.
    pub fn display_name(&self) -> &str {
        &self.player_name
    }

    /// Original player name supplied to the door session.
    pub fn player_name(&self) -> &str {
        &self.player_name
    }

    /// Upstream BBS identity when one exists.
    pub fn bbs_identity(&self) -> Option<&crate::session_bootstrap::BbsIdentity> {
        match &self.session_origin {
            SessionOrigin::Bbs(identity) => Some(identity),
            SessionOrigin::Native => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session_bootstrap::BbsIdentity;

    fn native_context() -> DoorContext {
        DoorContext::new(
            Uuid::nil(),
            "Native Player".to_string(),
            SessionOrigin::Native,
        )
    }

    fn bbs_context() -> DoorContext {
        DoorContext::new(
            Uuid::nil(),
            "BBS Player".to_string(),
            SessionOrigin::Bbs(BbsIdentity {
                user_id: Some("42".to_string()),
                username: Some("bbsuser".to_string()),
                display_name: Some("BBS Player".to_string()),
            }),
        )
    }

    #[test]
    fn native_identity_helpers_work() {
        let context = native_context();

        assert!(context.native_mode());
        assert!(!context.bbs_mode());
        assert_eq!(context.provider(), "late.sh");
        assert_eq!(context.display_name(), "Native Player");
        assert!(context.bbs_identity().is_none());
    }

    #[test]
    fn bbs_identity_helpers_work() {
        let context = bbs_context();

        assert!(!context.native_mode());
        assert!(context.bbs_mode());
        assert_eq!(context.provider(), "BinkTerm");
        assert_eq!(context.display_name(), "BBS Player");

        let identity = context.bbs_identity().expect("BBS identity missing");
        assert_eq!(identity.user_id.as_deref(), Some("42"));
    }
}
