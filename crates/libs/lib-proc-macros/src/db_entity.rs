use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitStr, parse_macro_input};

pub fn database_entity_derive_impl(input: TokenStream) -> TokenStream {
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
        }
    };

    TokenStream::from(expanded)
}
