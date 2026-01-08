use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct MLSClientId(pub Uuid);

#[derive(Debug, Clone)]
pub struct MLSClient {
    pub id: MLSClientId,
    pub assoc_user: Option<Uuid>, // associated user account, if any
}

impl MLSClient {
    pub fn has_account(&self) -> bool {
        return self.assoc_user.is_some();
    }
}

#[derive(Debug, Clone)]
pub struct KeyPackage {
    pub mls_client_id: MLSClientId,
    pub kpkg: Vec<u8>, // encrypted with user's public key
    pub expires_at: chrono::NaiveDateTime,
}
