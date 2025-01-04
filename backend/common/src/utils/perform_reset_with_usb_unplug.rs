use std::{default, env, path::PathBuf};

use sqlx::{Error, Pool, Sqlite, SqlitePool};
use tracing::{debug, error, info};

use crate::{hw::gpio::gpio_unplug_and_plug_nfc_usb, types::database::ApplicationState};

/// Copied from db to hack away the cyclic dependency
async fn connect_db_sqlite() -> Result<SqlitePool, Error> {
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

/// Copied from db to hack away the cyclic dependency
async fn get_sqlite_db_pool() -> Pool<Sqlite> {
    let pool_res = connect_db_sqlite().await;

    let pool = pool_res.unwrap_or_else(|e| {
        error!("The db pool is not accessible. {:#?}", e);
        panic!("Failed to connect to the sqlite db pool.")
    });

    return pool;
}

/// Used to check if the app should unplug the NFC USB
/// after the reboot, it checks the database for the variable
/// do_reset in application_state table
/// if it is 1 (TRUE), then the USB NFC reader is unplugged for defined
/// time and then connected back, then the value of do_reset in db changed
/// to 0 (FALSE) and the service which runs the application is restarted
/// at the start of the function perform_reset_with_nfc_usb_unplug it must
/// be a match statement which when finds 0 (FALSE) for the do_reset sets the
/// value to 1 (TRUE) and normally starts the application
/// so that when the device is rebooted it is ready to make a whole USB reset
/// cycle again
pub async fn perform_reset_with_nfc_usb_unplug(db_client: Pool<Sqlite>) {
    // cannot depend on db lib because db lib depends on common
    // this would result in a cyclic dependency
    let db_pool = get_sqlite_db_pool().await;

    if db_pool.is_closed() {
        error!("The database pool is closed or not reachable.\n Cannot check the application state for performing the reset.");
        return;
    }

    debug!("Starting to test the application_state for reboot operation.");

    let mut query_mapped: Result<ApplicationState, sqlx::Error> =
        sqlx::query_as(r#"SELECT * from application_state;"#)
            .fetch_one(&db_pool)
            .await;

    match query_mapped {
        Ok(query) => {
            match query.do_reset {
                0 => {
                    info!("Set do_reset to TRUE for the next reboot.");
                    let query = sqlx::query(r#"UPDATE application_state SET do_reset = 1"#).
                    execute(&db_pool).await;

                    gpio_unplug_and_plug_nfc_usb().await;


                }
                1 => {
                    info!("Set do_reset to FALSE for next reboot, replug USB and restart the service.");
                    let query = sqlx::query(r#"UPDATE application_state SET do_reset = 0"#).
                    execute(&db_pool).await;
                }
                default => {
                    info!("In application_state the do_reset has non valid value.");
                }
            };
        }
        Err(e) => {
            error!("Could not get the application_state from the db. {:#?}", e);
        }
    };
}
