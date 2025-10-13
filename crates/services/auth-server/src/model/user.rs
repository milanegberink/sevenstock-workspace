use lib_auth::pwd;
use modql::field::Fields;
use modql::field::HasSeaFields;
use modql::field::SeaField;
use modql::field::SeaFields;
use sea_query::Expr;
use sea_query::Iden;
use sea_query::PostgresQueryBuilder;
use sea_query::Query;
use sea_query_binder::SqlxBinder;
use secrecy::SecretString;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use uuid::Uuid;

use lib_core::model::Error;
use lib_core::model::Result;
use lib_core::model::base;
use lib_core::model::base::DbBmc;
use lib_core::model::base::prep_fields_for_update;
use lib_core::model::permission::Permission;
use lib_core::{ctx::Ctx, model::ModelManager};

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub email: String,
    pub pwd_clear: SecretString,
}

#[derive(Iden)]
enum UserIden {
    Id,
    Email,
    Username,
    Pwd,
}

#[derive(Fields)]
pub struct UserForInsert {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Fields, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

pub trait UserBy: HasSeaFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}

pub struct UserBmc {}

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: Uuid,
    pub username: String,
    pub pwd: Option<String>,
}

impl UserBmc {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<Uuid> {
        let UserForCreate {
            username,
            pwd_clear,
            email,
        } = user_c;

        let user_fi = UserForInsert {
            username,
            email: email.to_string(),
        };

        let mm = mm.new_with_txn()?;

        mm.dbx().begin_txn().await?;

        let user_id = base::create::<Self, _>(&ctx, &mm, user_fi)
            .await
            .map_err(|model_error| {
                Error::resolve_unique_violation(
                    model_error,
                    Some(|table: &str, constraint: &str| {
                        if table == "user" && constraint.contains("email") {
                            Some(Error::UserAlreadyExists { email })
                        } else {
                            None // Error::UniqueViolation will be created by resolve_unique_violation
                        }
                    }),
                )
            })?;

        Self::update_pwd(ctx, &mm, user_id, pwd_clear).await?;

        mm.dbx().commit_txn().await?;

        Ok(user_id)
    }
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn update_pwd(
        ctx: &Ctx,
        mm: &ModelManager,
        id: Uuid,
        pwd_clear: SecretString,
    ) -> Result<()> {
        let pwd = pwd::hash_password(pwd_clear)?;

        let mut fields = SeaFields::new(vec![SeaField::new(UserIden::Pwd, pwd)]);

        prep_fields_for_update::<Self>(&mut fields, ctx.user_id());

        let fields = fields.for_sea_update();

        let mut query = Query::update();

        query
            .table(Self::table_ref())
            .values(fields)
            .and_where(Expr::col(UserIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_with(&sql, values);
        let _count = mm.dbx().execute(sqlx_query).await?;

        Ok(())
    }
    pub async fn first_by_email<E>(_ctx: &Ctx, mm: &ModelManager, email: &str) -> Result<E>
    where
        E: UserBy,
    {
        let mut query = Query::select();

        query
            .from(Self::table_ref())
            .and_where(Expr::col(UserIden::Email).eq(email));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);
        let user = mm.dbx().fetch_one(sqlx_query).await?;

        Ok(user)
    }
    pub async fn list_permissions(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Permission>> {
        let sql = r#"
            SELECT DISTINCT
            p.id,
            p.resource,
            p.action
            FROM "user" u
            JOIN organisation_membership om ON u.id = om.user_id
            JOIN role r ON om.role_id = r.id
            JOIN role_permission rp ON r.id = rp.role_id
            JOIN permission p ON rp.permission_id = p.id
            WHERE u.id = $1
            AND r.organisation_id = $2
            ORDER BY p.resource, p.action;
            "#;

        let sqlx_query = sqlx::query_as::<_, Permission>(&sql)
            .bind(ctx.user_id())
            .bind(ctx.org_id());

        let permissions = mm.dbx().fetch_all(sqlx_query).await?;

        Ok(permissions)
    }
}
