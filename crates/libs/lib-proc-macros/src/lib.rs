use proc_macro::TokenStream;
use quote::quote;
use sqlx::{Execute, Postgres, QueryBuilder};
use syn::{DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(DbEntity, attributes(table_name))]
pub fn database_entity_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let table_name_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table_name"))
        .expect("A #[table_name(\"...\")] attribute is required.");

    let table_name: LitStr = table_name_attr
           .parse_args()
           .expect("The table_name attribute must contain a single string literal, like #[table_name(\"users\")].");

    let query_string = format!("SELECT * FROM {} WHERE id = $1", table_name.value());

    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(&format!(
        "INSERT INTO {} (id, username) VALUES ($1, $2)",
        table_name.value()
    ));

    let insert_query = query_builder.build();
    let sql = insert_query.sql();

    let expanded = quote! {
        impl #name {
            pub async fn get(mm: &ModelManager, ctx: &Ctx, id: uuid::Uuid) -> Result<Option<#name>, sqlx::Error> {
                sqlx::query_as!(#name, #query_string, id)
                    .fetch_optional(mm.db())
                    .await
            }
            pub async fn get_with_redis(mm: &ModelManager, ctx: &Ctx, id: uuid::Uuid) -> Result<Option<#name>, sqlx::Error> {
                let key = format!("{}:{}", #table_name, id);

                let redis_result: Result<String, RedisError> = mm.redis().get(&key).await;

                match redis_result {
                    Ok(cached) => {
                        println!("Got entity from redis");
                        match serde_json::from_str::<#name>(&cached) {
                            Ok(parsed) => return Ok(Some(parsed)),
                            Err(e) => {
                                println!("Failed to parse cached data for key {}: {}", key, e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Redis error for key {}: {}", key, e);
                    }
                }

                let entity = #name::get(&mm, &ctx, id).await?;

                if let Some(ref entity_data) = entity {
                    if let Ok(serialized) = serde_json::to_string(entity_data) {
                        if let Err(e) = {
                            let _: () = mm.redis().set(&key, serialized).await.expect("meow");
                            Ok::<(), redis::RedisError>(())
                        } {
                            println!("Failed to cache entity for key {}: {}", key, e);
                        }
                    }
                }


                Ok(entity)
            }
            pub async fn create(mm: &ModelManager, ctx: &Ctx) -> Result<(), sqlx::Error> {
                println!(#sql);

                let uuid = Uuid::now_v7();

                sqlx::query!(#sql, uuid, "Joost").execute(mm.db()).await?;

                println!("Created user with uuid {}", uuid);

                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(DbEntityForCreate, attributes(table_name))]
pub fn database_entity_for_create(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names: Vec<String> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string())
        .collect();

    let field_names_str = format!("{:?}", field_names);

    let expanded = quote! {
        impl #name {
            pub async fn test() {
                println!("{}", #field_names_str);
            }
        }
    };

    TokenStream::from(expanded)
}
