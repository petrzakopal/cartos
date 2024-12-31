use std::path::Path;

use dotenv::from_path;
use tracing::debug;
use std::env;

pub fn load_env() {
    //    let project_root = Path::new(env!("CARGO_MANIFEST_DIR")); // Project root directory
    //    let env_path = project_root.join(".env"); // Pointing to .env file in project root
    //
    //    // Load the .env file from the specified path when development used
    //    from_path(env_path).ok();

    // Check if running in a Docker environment
    if env::var("DOCKER_ENV").is_err() {
        let project_root = Path::new(env!("CARGO_MANIFEST_DIR")); // Crate root directory
        // please note that it is only a crate root direcotry,
        // when using cargo workspaces this points to the folder
        // where the toml for the currently build crate is
        // debug!("Cargo path {}", env!("CARGO_MANIFEST_DIR"));
        let env_path = project_root.join(".env"); // Pointing to .env file in project root

        // Load the .env file from the specified path if not running in Docker
        if let Err(e) = from_path(env_path) {
            eprintln!("Error loading .env file: {}", e);
        }
    }
}
