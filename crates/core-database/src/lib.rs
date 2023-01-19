use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Database(pub SqlitePool);

pub type DatabaseLock = Arc<RwLock<Database>>;

pub async fn increment_db_move_count_and_get_total(db: &DatabaseLock) -> i32 {
    let db = db.read().await;

    // Log the move in the database regardless of whether it succeeds because why not
    sqlx::query!("INSERT INTO moves (direction) VALUES (?)", "up")
        .execute(&db.0)
        .await
        .unwrap();

    let count_results = sqlx::query!("SELECT COUNT(id) as count FROM moves")
        .fetch_all(&db.0)
        .await
        .unwrap();

    let move_count = count_results[0].count;

    move_count
}

pub async fn database_setup() -> DatabaseLock {
    // Database setup
    // Initiate a connection to the database file, creating the file if required.
    let database = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    // Run migrations, which updates the database's schema to the latest version.
    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Couldn't run database migrations");

    let db: DatabaseLock = Arc::new(RwLock::new(Database(database)));

    db
}
