use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::{
    app::AppState,
    errors::{AppResult, AuthError},
    modules::graphql::schema::build_schema,
    utils::jwt::get_uuid_from_token,
};

fn current_user_id(jar: &CookieJar, state: &AppState) -> AppResult<Uuid> {
    let token = jar
        .get("token")
        .map(|c| c.value().to_string())
        .ok_or(AuthError::Unauthorized)?;

    Ok(get_uuid_from_token(&token, &state.config.jwt.secret)?.parse()?)
}

pub async fn graphql_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    request: GraphQLRequest,
) -> AppResult<GraphQLResponse> {
    let adventurer_id = current_user_id(&jar, &state)?;
    let schema = build_schema(state);
    let request = request.into_inner().data(adventurer_id);

    Ok(schema.execute(request).await.into())
}
