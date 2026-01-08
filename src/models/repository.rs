use crate::models::user::MLSClientId;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Repository {
    pub owner_id: Uuid,
    pub name: String,
    pub members: Vec<MLSClientId>,
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
    pub repo: Uuid,
    pub parents: Vec<CommitHash>,
    pub author: MLSClientId,
    pub changes: EncryptedChangeSet, // encrypted changes
    pub message: EncryptedCommitMessage, // encrypted commit message
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum MessageType {
    Proposal,
    Commit,
    Welcome,
    Application,
}

#[derive(Debug, Clone)]
pub struct BroadcastMessage {
    pub id: Uuid,
    pub repo: Uuid,
    pub sender: MLSClientId,
    pub message_type: MessageType,
    pub payload: Vec<u8>, // encrypted payload
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct UnicastMessage {
    pub id: Uuid,
    pub recipient: MLSClientId,
    pub sender: MLSClientId,
    pub message_type: MessageType,
    pub payload: Vec<u8>, // encrypted payload
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct BroadcastMessageReadReceipt {
    pub message_id: Uuid,
    pub readers: MLSClientId,
    pub read_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct UnicastMessageReadReceipt {
    pub message_id: Uuid,
    pub read_at: chrono::NaiveDateTime,
}

pub struct BlobServerId(pub Uuid);
