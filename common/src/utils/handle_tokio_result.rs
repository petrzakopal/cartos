/// Use to gracefuly handle tokio join of spawned threads
pub fn handle_task_result<T>(task_result: Result<T, impl std::fmt::Debug>, task_name: &str) {
    if let Err(e) = task_result {
        tracing::error!("Error in {}: {:?}", task_name, e);
    }
}
