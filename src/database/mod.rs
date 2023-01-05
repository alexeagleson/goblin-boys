use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Database(pub SqlitePool);

pub type DatabaseLock = Arc<RwLock<Database>>;
