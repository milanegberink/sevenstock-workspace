use proc_macro::TokenStream;

mod db_entity;

#[proc_macro_derive(DbEntity, attributes(table_name))]
pub fn database_entity_derive(input: TokenStream) -> TokenStream {
    db_entity::database_entity_derive_impl(input)
}
