use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{Extension, State};
use uuid::Uuid;

use crate::{
    app::AppState,
    errors::{AppResult, UuidError},
    modules::graphql::schema::build_schema,
    utils::jwt::Claims,
};

pub async fn graphql_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    request: GraphQLRequest,
) -> AppResult<GraphQLResponse> {
    let adventurer_id = claims.sub.parse::<Uuid>().map_err(UuidError::from)?;
    let schema = build_schema(state);
    let request = request.into_inner().data(adventurer_id);

    Ok(schema.execute(request).await.into())
}
