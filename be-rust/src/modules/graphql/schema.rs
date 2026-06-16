use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::{app::AppState, modules::graphql::query::QueryRoot};

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(state: AppState) -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state)
        .finish()
}
