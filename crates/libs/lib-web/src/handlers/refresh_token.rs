use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use lib_auth::token::TokenBuilder;

pub async fn refresh_token(ctx: CtxW, refresh_token: TypedHeader<Authorization<Bearer>>) {
    let refresh_token = TokenBuilder::refresh().sub(ctx.0.user_id()).build();
    let access_token = TokenBuilder::access().sub(ctx.0.user_id()).build();
}
