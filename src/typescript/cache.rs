const LAST_UPDATE_CHECK: &str = "typescript-last-update-check";

pub struct UpdateCache {
    path: std::path::PathBuf,
}

impl UpdateCache {
    pub fn new(work_dir: &std::path::Path) -> Self {
        Self {
            path: work_dir.join(LAST_UPDATE_CHECK),
        }
    }

    pub fn last_check(&self) -> Option<u64> {
        std::fs::read_to_string(&self.path)
            .ok()?
            .parse()
            .ok()
    }

    pub fn record_check(&self, now: u64) -> std::io::Result<()> {
        std::fs::write(&self.path, now.to_string())
    }
}
