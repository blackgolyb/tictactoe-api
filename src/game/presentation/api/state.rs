use std::sync::Arc;

use crate::game::infrastructure::di::DIContainer;

/// Application state that holds the DI container
/// This is shared across all HTTP handlers via actix-web's Data extractor
#[derive(Clone)]
pub struct AppState {
    pub container: Arc<DIContainer>,
}

impl AppState {
    pub fn new(container: DIContainer) -> Self {
        Self {
            container: Arc::new(container),
        }
    }
}
