//! Backup provider interface (groundwork — not implemented in this phase).
//!
//! Phase 4 will implement at least:
//! - `GoogleDriveBackupProvider`: uploads encrypted `.chrxp` to a user's Drive
//! - `LocalFolderBackupProvider`: copies `.chrxp` to a local directory on schedule
//!
//! The provider is responsible for:
//! 1. Authenticating with the storage service (OAuth2 for Drive).
//! 2. Listing existing backups (for rotation / duplicate detection).
//! 3. Uploading a new encrypted archive.
//! 4. Downloading the latest backup for restore.
//! 5. Reporting progress via a callback.
//!
//! Encryption is always done before the provider touches the bytes —
//! the provider is not trusted with the cleartext key.

use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// Metadata about a stored backup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMeta {
    /// Provider-assigned unique identifier (Drive file ID, S3 key, etc.).
    pub id: String,
    /// ISO-8601 timestamp when the backup was created.
    pub created_at: String,
    /// Human-readable label (e.g., filename or Drive title).
    pub label: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Whether the archive is password-protected.
    pub encrypted: bool,
}

/// Progress update emitted during upload / download.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupProgress {
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub phase: BackupPhase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupPhase {
    Preparing,
    Uploading,
    Downloading,
    Verifying,
    Done,
}

/// Errors from backup operations.
#[derive(Debug, thiserror::Error)]
pub enum BackupError {
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Quota exceeded")]
    QuotaExceeded,
    #[error("Backup not found: {0}")]
    NotFound(String),
    #[error("Provider error: {0}")]
    Provider(String),
}

/// A callback type for progress updates.
pub type ProgressFn = Box<dyn Fn(BackupProgress) + Send + Sync>;

/// Trait that every backup provider must implement.
///
/// All methods are `async` and take `&self` (providers should store their
/// authenticated client in `Arc<Mutex<…>>` if they need mutable state).
#[async_trait]
pub trait BackupProvider: Send + Sync {
    /// Human-readable name (e.g., "Google Drive", "Local Folder").
    fn name(&self) -> &'static str;

    /// Return true if the provider is authenticated and ready.
    async fn is_authenticated(&self) -> bool;

    /// Authenticate the provider (e.g., open OAuth2 browser flow).
    async fn authenticate(&self) -> Result<(), BackupError>;

    /// List existing backups, newest first.
    async fn list_backups(&self) -> Result<Vec<BackupMeta>, BackupError>;

    /// Upload a new backup archive.
    ///
    /// `data` is the encrypted `.chrxp` bytes (never plaintext).
    /// `label` is a suggested filename.
    async fn upload(
        &self,
        data: Vec<u8>,
        label: String,
        progress: Option<ProgressFn>,
    ) -> Result<BackupMeta, BackupError>;

    /// Download a backup by its provider-assigned `id`.
    async fn download(
        &self,
        id: &str,
        progress: Option<ProgressFn>,
    ) -> Result<Vec<u8>, BackupError>;

    /// Delete a backup by id. Used for rotation (keep last N backups).
    async fn delete(&self, id: &str) -> Result<(), BackupError>;
}

// ── Stubs ─────────────────────────────────────────────────────────────────────

/// Stub: Google Drive backup (Phase 4).
///
/// Will use OAuth2 + Drive REST API. Requires user to grant
/// `drive.appdata` or `drive.file` scope.
pub struct GoogleDriveBackupProvider;

#[async_trait]
impl BackupProvider for GoogleDriveBackupProvider {
    fn name(&self) -> &'static str {
        "Google Drive"
    }

    async fn is_authenticated(&self) -> bool {
        false // TODO: Phase 4
    }

    async fn authenticate(&self) -> Result<(), BackupError> {
        Err(BackupError::Provider(
            "Google Drive backup not yet implemented (Phase 4)".into(),
        ))
    }

    async fn list_backups(&self) -> Result<Vec<BackupMeta>, BackupError> {
        Err(BackupError::Provider(
            "Google Drive backup not yet implemented (Phase 4)".into(),
        ))
    }

    async fn upload(
        &self,
        _data: Vec<u8>,
        _label: String,
        _progress: Option<ProgressFn>,
    ) -> Result<BackupMeta, BackupError> {
        Err(BackupError::Provider(
            "Google Drive backup not yet implemented (Phase 4)".into(),
        ))
    }

    async fn download(
        &self,
        _id: &str,
        _progress: Option<ProgressFn>,
    ) -> Result<Vec<u8>, BackupError> {
        Err(BackupError::Provider(
            "Google Drive backup not yet implemented (Phase 4)".into(),
        ))
    }

    async fn delete(&self, _id: &str) -> Result<(), BackupError> {
        Err(BackupError::Provider(
            "Google Drive backup not yet implemented (Phase 4)".into(),
        ))
    }
}
