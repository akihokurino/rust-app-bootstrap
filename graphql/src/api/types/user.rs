use async_graphql::{Object, ID};

#[derive(Debug, Clone)]
pub struct Me(domain::types::user::User);
#[Object]
impl Me {
    async fn id(&self) -> ID {
        ID::from(self.0.id.clone().as_str())
    }

    async fn name(&self) -> String {
        self.0.name.clone().to_string()
    }
}
