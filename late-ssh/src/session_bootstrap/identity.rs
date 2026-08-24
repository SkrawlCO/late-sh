//! Session identity primitives.

/// Identity asserted by an upstream BBS gateway for this SSH session.
///
/// This is session-scoped metadata. It does not replace late.sh's own
/// authenticated User identity.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BbsIdentity {
    pub user_id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
}

impl BbsIdentity {
    pub fn is_empty(&self) -> bool {
        self.user_id.is_none() && self.username.is_none() && self.display_name.is_none()
    }

    /// Stable upstream identity key when one exists.
    ///
    /// Prefer the BBS-provided user id because usernames/display names may
    /// change over time.
    pub fn stable_key(&self) -> Option<&str> {
        self.user_id.as_deref().or(self.username.as_deref())
    }

    /// Human-facing name for display in experiences.
    ///
    /// Prefer the explicit display name, then fall back to username.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref().or(self.username.as_deref())
    }
}

/// Where this session identity originated.
///
/// Native late.sh sessions have no upstream identity.
/// BBS sessions carry the identity asserted by the gateway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionOrigin {
    Native,
    Bbs(BbsIdentity),
}

impl SessionOrigin {
    pub fn provider(&self) -> &'static str {
        match self {
            SessionOrigin::Native => "late.sh",
            SessionOrigin::Bbs(_) => "BinkTerm",
        }
    }

    /// True when this session was launched through an upstream BBS.
    pub fn is_bbs(&self) -> bool {
        matches!(self, SessionOrigin::Bbs(_))
    }
}
