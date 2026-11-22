use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),
    #[error("Asset dimensions exceed maximum allowed limit ({0}x{1})")]
    ImageTooLarge(u32, u32),
    #[error("File size exceeds limit: {0} bytes")]
    FileTooLarge(u64),
}

pub const MAX_IMAGE_DIMENSION: u32 = 4096;
pub const MAX_FILE_SIZE_BYTES: u64 = 50 * 1024 * 1024; // 50MB

pub fn validate_pack_path(base_dir: &Path, rel_path: &Path) -> Result<(), SecurityError> {
    for component in rel_path.components() {
        if let std::path::Component::ParentDir = component {
            return Err(SecurityError::PathTraversal(
                rel_path.to_string_lossy().to_string(),
            ));
        }
    }
    let full = base_dir.join(rel_path);
    if !full.starts_with(base_dir) {
        return Err(SecurityError::PathTraversal(
            rel_path.to_string_lossy().to_string(),
        ));
    }
    Ok(())
}
