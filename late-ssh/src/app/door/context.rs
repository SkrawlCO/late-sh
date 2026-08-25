use crate::session_bootstrap::SessionOrigin;
use uuid::Uuid;

/// Identity presented to a door for the current session.
///
/// `user_id` is always the authoritative late.sh account identity used for
/// ownership and persistence. Provider and external identity describe where
/// the session originated without exposing transport-specific details.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DoorIdentity<'a> {
    pub user_id: Uuid,
    pub display_name: &'a str,
    pub provider: &'static str,
    pub external_identity: Option<&'a str>,
}

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

    /// Unified identity view for this door session.
    ///
    /// Doors consume identity through this API without needing to know how the
    /// session was authenticated or which transport supplied external identity.
    pub fn identity(&self) -> DoorIdentity<'_> {
        DoorIdentity {
            user_id: self.user_id,
            display_name: &self.player_name,
            provider: self.session_origin.provider(),
            external_identity: match &self.session_origin {
                SessionOrigin::Native => None,
                SessionOrigin::Bbs(identity) => identity.stable_key(),
            },
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
    fn native_identity_view_is_transport_neutral() {
        let context = native_context();
        let identity = context.identity();

        assert_eq!(identity.user_id, Uuid::nil());
        assert_eq!(identity.display_name, "Native Player");
        assert_eq!(identity.provider, "late.sh");
        assert_eq!(identity.external_identity, None);
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

    #[test]
    fn bbs_identity_view_exposes_stable_external_identity() {
        let context = bbs_context();
        let identity = context.identity();

        assert_eq!(identity.user_id, Uuid::nil());
        assert_eq!(identity.display_name, "BBS Player");
        assert_eq!(identity.provider, "BinkTerm");
        assert_eq!(identity.external_identity, Some("42"));
    }
}
