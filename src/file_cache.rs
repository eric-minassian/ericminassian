use std::{fs::File, io::Read, path::PathBuf, sync::Arc};

use dashmap::DashMap;

pub type FileCache = Arc<DashMap<PathBuf, Arc<Vec<u8>>>>;

#[must_use]
pub fn load_file(cache: &FileCache, path: &PathBuf) -> Option<Arc<Vec<u8>>> {
    if let Some(file) = cache.get(path) {
        return Some(file.value().clone());
    }

    let mut file = File::open(path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    let buffer = Arc::new(buffer);

    cache.insert(path.clone(), buffer.clone());

    Some(buffer)
}
