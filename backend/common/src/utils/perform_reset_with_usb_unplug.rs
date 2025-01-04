use std::{default, env, path::PathBuf};

use sqlx::{Error, Pool, Sqlite, SqlitePool};
use tracing::{debug, error, info, warn};

use crate::{
    hw::gpio::gpio_unplug_and_plug_nfc_usb,
    types::{database::ApplicationState, general::ApplicationRunMode},
};

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
pub async fn perform_reset_with_nfc_usb_unplug() {
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

                    let query = sqlx::query(r#"UPDATE application_state SET do_reset = 1"#)
                        .execute(&db_pool)
                        .await;
                }
                1 => {
                    info!("Set do_reset to FALSE for next reboot, replug USB and restart the service.");
                    let query = sqlx::query(r#"UPDATE application_state SET do_reset = 0"#)
                        .execute(&db_pool)
                        .await;

                    gpio_unplug_and_plug_nfc_usb().await;
                    let mut application_run_mode: ApplicationRunMode;

                    // Check if the app is running as a service or a standalone app
                    if env::var("STANDALONE").is_ok() {
                        debug!("The app runs in a standalone mode, will restart the app.");
                        application_run_mode = ApplicationRunMode::Standalone;
                    } else {
                        debug!("The app runs as a service, will restart the service.");
                        application_run_mode = ApplicationRunMode::Service;
                    }

                    info!("The applications runs as {:#?}", application_run_mode);
                    match application_run_mode {
                        ApplicationRunMode::Standalone => {
                            // searches for the process running wirth port of 4000 - this is the
                            // server api port
                            // in the future the port will be passed using argument
                            let output = std::process::Command::new("lsof")
                                .arg("-t") // Only output PIDs
                                .arg("-i")
                                .arg(format!("tcp:{}", 4000))
                                .output();

                            // will make to separete function witht the results
                            // checking if some process has been found
                            match output {
                                Ok(output) if !output.stdout.is_empty() => {
                                    // Get the PID of the process using the port
                                    let pid_str = String::from_utf8_lossy(&output.stdout);
                                    let pid = pid_str.trim().parse::<u32>().ok();

                                    // if so
                                    if let Some(pid) = pid {
                                        // Kill the process using the found PID
                                        let kill_output = tokio::process::Command::new("kill")
                                            .arg("-TERM")
                                            .arg(pid.to_string())
                                            .output()
                                            .await
                                            .unwrap();

                                        if !kill_output.status.success() {
                                            error!("Failed to send kill signal to the process");
                                            //return Err(std::io::Error::new(
                                            //    std::io::ErrorKind::Other,
                                            //    "Failed to kill process",
                                            //));
                                            return;
                                        }

                                        info!("Successfully killed the process with PID {}", pid);
                                    } else {
                                        // Err(std::io::Error::new(
                                        //     std::io::ErrorKind::Other,
                                        //     "Invalid PID",
                                        // ));
                                    }
                                }
                                _ => {
                                    info!("No process is using port {}", 4000);
                                }
                            }
                            // getting the executable path of the application
                            let current_exe =
                                env::current_exe().expect("Failed to get current executable path");
                            debug!(
                                "The current executable path of the application is {:#?}",
                                current_exe
                            );
                            // spawning the new app
                            tokio::process::Command::new(current_exe.clone())
                                .spawn()
                                .expect("Failed to spawn new instance");

                            warn!("Terminating the current standalone app instance.");

                            // exit the process
                            std::process::exit(0);
                        }
                        ApplicationRunMode::Service => {
                            warn!("Terminating the current service instance.");
                            let service_name = "cartos-backend.service"; // Replace with your actual service name
                            let status: std::process::ExitStatus =
                                std::process::Command::new("systemctl")
                                    .arg("restart")
                                    .arg(service_name)
                                    .status()
                                    .expect("Failed to execute command");
                        }
                    };
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
