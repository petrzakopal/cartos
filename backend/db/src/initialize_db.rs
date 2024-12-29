use std::{
    env,
    path::{Path, PathBuf},
};

use sqlx::{migrate::Migrator, Error, SqlitePool};
use tracing::{debug, error, info};

pub async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        cardSerialNumber TEXT NOT NULL,
        email TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn initialize_db() {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    let mut pool_res = connect_db_sqlite().await;

    let pool = match pool_res {
        Ok(pool) => pool,
        Err(e) => {
            error!("Error creating a pool");
            return;
        }
    };

    //let res = create_table(&pool).await;

    //debug!("Result of creating a table {:#?}", res);
}

pub async fn connect_db_sqlite() -> Result<SqlitePool, Error> {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return Err(sqlx::Error::from(e));
        }
    };

    // Construct the full path for the SQLite database file
    let db_path: PathBuf = current_dir.join("cartos.db");

    // Convert PathBuf to a string and use it in the SQLite connection string
    let database_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

    // Connect to the SQLite database
    let pool = match SqlitePool::connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return Err(e);
        }
    };

    return Ok(pool);
}

pub async fn run_migrations_sqlite() {

    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
            return;
        }
    };

    // Construct the full path for the SQLite database file
    let migrations_path: PathBuf = current_dir.join("migrations");

    debug!("Searching for migrations in the path {:#?}", migrations_path);

    let pool_prep = connect_db_sqlite().await;



    let pool = match pool_prep {
        Ok(pool) => pool,
        Err(e) => {
            error!("Cannot connect to db sqlite");
            return ;
        }
        
    };

    let migrator_prep = Migrator::new(migrations_path).await;

    let migrator = match migrator_prep {
        Ok(migrator) => {
            let migrator_res = match migrator.run(&pool).await {
                Ok(_) => {
                    info!("Successfully performed the migrations.");
                }
                Err(e) => {
                    error!("Did not perform successful migrations. {:#?}", e);
                }

            };
        }
        Err(e) => {
            error!("Error initialization of the mirgator {:#?}", e);
        }
    };
}
