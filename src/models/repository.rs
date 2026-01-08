use crate::models::user::MLSClientId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Repository {
    owner_id: Uuid,
    name: String,
}

impl Repository {
    pub fn identifier(&self) -> String {
        format!("{}/{}", self.owner_id, self.name)
    }
}

pub enum RepositoryPermission {
    Viewer = 250,
    Contributor = 500,
    Editor = 750,
    Admin = 1000,
}

pub struct RepositoryAccess {
    pub repository: Repository,
    pub permission: RepositoryPermission,
    pub delegation_level: Option<RepositoryPermission>,
}

impl RepositoryAccess {
    pub fn can_delegate(&self) -> bool {
        return self.delegation_level.is_some();
    }
}

#[derive(Debug, Clone)]
pub struct CommitHash(pub String);

#[derive(Debug, Clone)]
pub struct EncryptedChangeSet(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct EncryptedCommitMessage(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: CommitHash, 
    pub repo: Repository,
    pub parents: Vec<CommitHash>,
    pub author: MLSClientId,
    pub changes: EncryptedChangeSet, // encrypted changes
    pub message: EncryptedCommitMessage, // encrypted commit message
    pub created_at: chrono::NaiveDateTime,
}

