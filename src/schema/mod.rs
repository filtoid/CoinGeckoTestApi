use async_graphql::{MergedObject, SchemaBuilder, EmptyMutation, EmptySubscription, Schema};

mod health;

#[derive(MergedObject, Default)]
pub struct Query(
    health::HealthQuery,
);

/// Build the GraphQL Schema
pub fn build_schema() -> SchemaBuilder<Query,EmptyMutation,EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
}