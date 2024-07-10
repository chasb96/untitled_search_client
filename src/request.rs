use prost::Message;

#[derive(Message)]
pub struct CreateUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
    #[prost(string, tag = "2")]
    pub username: String,
}

#[derive(Message)]
pub struct CreateProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
    #[prost(string, tag = "2")]
    pub project_name: String,
    #[prost(string, tag = "3")]
    pub value: String,
}