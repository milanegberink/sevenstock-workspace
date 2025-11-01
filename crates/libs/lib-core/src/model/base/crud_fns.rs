use modql::{field::HasSeaFields, filter::ListOptions};
use redis::AsyncCommands;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{FromRow, postgres::PgRow};
use uuid::Uuid;

use crate::{
    ctx::Ctx,
    model::{
        Error, ModelManager, Result,
        base::{
            CommonIden, DbBmc, LIST_LIMIT_DEFAULT, LIST_LIMIT_MAX, prep_fields_for_update,
            utils::prep_fields_for_create,
        },
    },
};

pub async fn create<MC, E>(ctx: &Ctx, mm: &ModelManager, data: E) -> Result<Uuid>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let user_id = ctx.user_id();

    let mut fields = data.not_none_sea_fields();
    prep_fields_for_create::<MC>(&mut fields, user_id);

    let (columns, sea_values) = fields.for_sea_insert();
    let mut query = Query::insert();

    query
        .into_table(MC::table_ref())
        .columns(columns)
        .values(sea_values)?
        .returning(Query::returning().columns([CommonIden::Id]));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sqlx_query = sqlx::query_as_with::<_, (Uuid,), _>(&sql, values);

    let (id,) = mm.dbx().fetch_one(sqlx_query).await?;

    Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
where
    MC: DbBmc,
    E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
    E: HasSeaFields,
{
    let mut query = Query::select();

    query
        .from(MC::table_ref())
        .columns(E::sea_column_refs())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

    let sqlx_query = sqlx::query_as_with::<_, E, _>(&sql, values);

    let entity = mm
        .dbx
        .fetch_optional(sqlx_query)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

// pub async fn get_with_redis<MC, E>(ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<E>
// where
//     MC: DbBmc,
//     E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
//     E: HasSeaFields,
// {
//     let key = format!("{}:{}", MC::TABLE, id);
//     match mm.redis().get::<E>(key).await {
//         Ok(e) => Ok(e),
//         Err(_) => {
//             let e = get::<MC, E>(&ctx, &mm, id).await?;

//             mm.redis().set(key, &e).await;

//             Ok(e)
//         }
//     }
// }

pub async fn create_many<MC, E>(ctx: &Ctx, mm: &ModelManager, data: Vec<E>) -> Result<Vec<Uuid>>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    let user_id = ctx.user_id();
    let mut ids = Vec::with_capacity(data.len());

    // Prepare insert query
    let mut query = Query::insert();

    for item in data {
        let mut fields = item.not_none_sea_fields();
        prep_fields_for_create::<MC>(&mut fields, user_id);
        let (columns, sea_values) = fields.for_sea_insert();

        // Append values for each item
        query
            .into_table(MC::table_ref())
            .columns(columns.clone())
            .values(sea_values)?;
    }

    query.returning(Query::returning().columns([CommonIden::Id]));

    // Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_as_with::<_, (Uuid,), _>(&sql, values);

    let rows = mm.dbx().fetch_all(sqlx_query).await?;

    for row in rows {
        let (id,): (Uuid,) = row;
        ids.push(id);
    }

    Ok(ids)
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: Uuid) -> Result<()>
where
    MC: DbBmc,
{
    // -- Build query
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(CommonIden::Id).eq(id));

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = mm.dbx().execute(sqlx_query).await?;

    // -- Check result
    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub async fn delete_many<MC>(_ctx: &Ctx, mm: &ModelManager, ids: Vec<Uuid>) -> Result<u64>
where
    MC: DbBmc,
{
    if ids.is_empty() {
        return Ok(0);
    }

    // -- Build query
    let mut query = Query::delete();
    query
        .from_table(MC::table_ref())
        .and_where(Expr::col(CommonIden::Id).is_in(ids.clone()));

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let result = mm.dbx().execute(sqlx_query).await?;

    // -- Check result
    if result as usize != ids.len() {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id: Uuid::nil(),
        })
    } else {
        Ok(result)
    }
}

pub async fn update<MC, E>(ctx: &Ctx, mm: &ModelManager, id: Uuid, data: E) -> Result<()>
where
    MC: DbBmc,
    E: HasSeaFields,
{
    // -- Prep Fields
    let mut fields = data.not_none_sea_fields();
    prep_fields_for_update::<MC>(&mut fields, ctx.user_id());

    // -- Build query
    let fields = fields.for_sea_update();
    let mut query = Query::update();
    query
        .table(MC::table_ref())
        .values(fields)
        .and_where(Expr::col(CommonIden::Id).eq(id));

    // -- Execute query
    let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
    let sqlx_query = sqlx::query_with(&sql, values);
    let count = mm.dbx().execute(sqlx_query).await?;

    // -- Check result
    if count == 0 {
        Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })
    } else {
        Ok(())
    }
}

pub fn compute_list_options(list_options: Option<ListOptions>) -> Result<ListOptions> {
    if let Some(mut list_options) = list_options {
        // Validate the limit.
        if let Some(limit) = list_options.limit {
            if limit > LIST_LIMIT_MAX {
                return Err(Error::ListLimitOverMax {
                    max: LIST_LIMIT_MAX,
                    actual: limit,
                });
            }
        }
        // Set the default limit if no limit
        else {
            list_options.limit = Some(LIST_LIMIT_DEFAULT);
        }
        Ok(list_options)
    }
    // When None, return default
    else {
        Ok(ListOptions {
            limit: Some(LIST_LIMIT_DEFAULT),
            offset: None,
            order_bys: Some("id".into()),
        })
    }
}
