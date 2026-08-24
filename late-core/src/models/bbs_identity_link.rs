use anyhow::Result;
use tokio_postgres::Client;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BbsIdentityLink {
    pub id: Uuid,
    pub provider: String,
    pub external_user_id: String,
    pub user_id: Uuid,
}

impl BbsIdentityLink {
    pub async fn find_user_id(
        client: &Client,
        provider: &str,
        external_user_id: &str,
    ) -> Result<Option<Uuid>> {
        let row = client
            .query_opt(
                "SELECT user_id
                   FROM bbs_identity_links
                  WHERE provider = $1
                    AND external_user_id = $2",
                &[&provider, &external_user_id],
            )
            .await?;

        Ok(row.map(|row| row.get("user_id")))
    }

    pub async fn create(
        client: &Client,
        provider: &str,
        external_user_id: &str,
        user_id: Uuid,
    ) -> Result<Self> {
        let row = client
            .query_one(
                "INSERT INTO bbs_identity_links (
                     provider,
                     external_user_id,
                     user_id
                 )
                 VALUES ($1, $2, $3)
                 RETURNING id, provider, external_user_id, user_id",
                &[&provider, &external_user_id, &user_id],
            )
            .await?;

        Ok(Self {
            id: row.get("id"),
            provider: row.get("provider"),
            external_user_id: row.get("external_user_id"),
            user_id: row.get("user_id"),
        })
    }
}
