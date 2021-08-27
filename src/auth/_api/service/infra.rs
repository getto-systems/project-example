pub trait AuthAuthorizer {
    pub async fn fetch_token(&self) -> Result<Option<String>, AuthServiceError>;
}
