use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base;
use crate::{generate_common_bmc_fns, model::base::DbBmc};
use lib_auth::secret::generate_secret_key_b64u;
use lib_auth::secret::hash_secret_key;
use modql::field::{Fields, HasSeaFields};
use sea_query::Expr;
use sea_query::Iden;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::prelude::FromRow;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

pub trait RefreshTokenBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RefreshTokenBy for RefreshToken {}

#[derive(Debug, Fields, FromRow)]
pub struct RefreshToken {
    pub user_id: Uuid,
    id: Uuid,
    token_hash: Vec<u8>,
    expires_at: OffsetDateTime,
}

impl RefreshToken {
    pub fn is_expired(&self) -> bool {
        let now = OffsetDateTime::now_utc();

        self.expires_at < now
    }
}

#[derive(Deserialize, Fields)]
pub struct RefreshTokenForCreate {
    pub token_hash: Vec<u8>,
    pub user_id: Uuid,
}

impl DbBmc for RefreshTokenBmc {
    const TABLE: &'static str = "refresh_token";
}

pub struct RefreshTokenBmc;

#[derive(Iden)]
enum RefreshIden {
    UserId,
    TokenHash,
}

impl RefreshTokenBmc {
    pub async fn generate_new(ctx: &Ctx, mm: &ModelManager) -> Result<SecretString> {
        let refresh_token_raw = generate_secret_key_b64u();

        let refresh_token_c = RefreshTokenForCreate {
            token_hash: hash_secret_key(&refresh_token_raw.expose_secret()).to_vec(),
            user_id: ctx.user_id(),
        };

        let id = Self::create(&ctx, &mm, refresh_token_c).await?;

        Ok(refresh_token_raw)
    }
    pub async fn get_by_hash<E>(ctx: &Ctx, mm: &ModelManager, token_hash: &[u8]) -> Result<E>
    where
        E: RefreshTokenBy,
    {
        let mut query = Query::select();

        query
            .from(Self::table_ref())
            .and_where(Expr::col(RefreshIden::UserId).eq(ctx.user_id()))
            .and_where(Expr::col(RefreshIden::TokenHash).eq(token_hash));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let user = mm.dbx().fetch_one(sqlx_query).await?;

        Ok(user)
    }
}

generate_common_bmc_fns!(
    Bmc: RefreshTokenBmc,
    Entity: RefreshToken,
    ForCreate: RefreshTokenForCreate,
);
