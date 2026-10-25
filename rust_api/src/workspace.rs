use std::{fs, io, path::{Path, PathBuf}};
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkspaceManager {
    root: PathBuf,
}

impl WorkspaceManager {
    pub fn new(root: impl Into<PathBuf>) -> io::Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    pub fn create(&self) -> io::Result<PathBuf> {
        let path = self.root.join(Uuid::new_v4().to_string());
        fs::create_dir_all(&path)?;
        Ok(path)
    }

    pub fn safe_join(&self, workspace: &Path, relative: &Path) -> io::Result<PathBuf> {
        if relative.is_absolute() || relative.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "unsafe path"));
        }
        let base = workspace.canonicalize()?;
        let candidate = base.join(relative);
        if let Some(parent) = candidate.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(candidate)
    }
}
