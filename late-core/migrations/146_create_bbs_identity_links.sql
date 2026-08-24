CREATE TABLE bbs_identity_links (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    created TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,
    updated TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    provider TEXT NOT NULL,
    external_user_id TEXT NOT NULL,

    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,

    UNIQUE (provider, external_user_id),
    UNIQUE (provider, user_id)
);

CREATE INDEX idx_bbs_identity_links_user_id
    ON bbs_identity_links(user_id);
