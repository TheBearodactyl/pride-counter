pub async fn with_loading_screen<F>(loading_fn: fn(), task_fn: F)
where
    F: FnOnce() + Send + 'static,
{
    let loading_handle = tokio::spawn(async move {
        loading_fn();
    });

    let task_handle = tokio::spawn(async move {
        task_fn();
    });

    task_handle.await.unwrap();
    loading_handle.abort();
}
