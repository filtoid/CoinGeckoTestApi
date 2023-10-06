use async_graphql::Object;

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Returns true when GraphQL is reachable
    async fn health(&self) -> bool {
        true
    }
}