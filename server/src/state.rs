use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Per-session bookkeeping.
#[derive(Default, Debug)]
pub struct SessionState {
    pub last_seq: u64,
    pub received: u64,
    pub dropped: u64,
}

#[derive(Default, Debug)]
pub struct AppState {
    pub sessions: HashMap<String, SessionState>,
}

pub type Shared = Arc<RwLock<AppState>>;
