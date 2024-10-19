use std::{error::Error, sync::Arc, time::Duration};

use tokio::{sync::Semaphore, task::JoinSet};
use tracing_subscriber::fmt::format::FmtSpan;

type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

const CONCURRENCY: usize = 2;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        // We want to get a summary of the time taken for interesting blocks of code (in spans)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .init();

    // We want ot run all of these tasks. But they could overload the "system" (eg: remote API)
    let tasks_with_sleep_seconds = vec![3, 3, 3, 1, 5, 1, 2, 4];

    // So, we create a semaphore to limit the concurrency of the tasks to 2 at a time.
    // As each permit expires, the next task waiting inline can take a permit and continue running
    let semaphore = Arc::new(Semaphore::new(CONCURRENCY));

    // Store the JoinHandles in a JoinSet (see also JoinMap)
    let mut tasks = JoinSet::new();

    // TODO (@NickLarsenNZ): Move this to the limit impl, then update the example
    // Spawn tasks and store the JoinHandles in a JoinSet
    for (id, &sleep_seconds) in tasks_with_sleep_seconds.iter().enumerate() {
        tracing::info!(id, sleep_seconds, "loading task");
        let semaphore = semaphore.clone();

        let task = async move {
            tracing::info!(id, sleep_seconds, "waiting for permit");
            let permit = semaphore.acquire().await.expect("acquire semaphore permit");
            tracing::info!(id, sleep_seconds, "starting task");
            tokio::time::sleep(Duration::from_secs(sleep_seconds as u64)).await;
            tracing::info!(id, sleep_seconds, "finished task");
            drop(permit);
            id
        };
        tasks.spawn(task);
    }

    // The tasks are already running (spawn). But now we can await each of the join handles
    let span = tracing::warn_span!("wait on jobs to finish");
    span.in_scope(|| async {
        while let Some(result) = tasks.join_next().await {
            let id = result.expect("is some");
            tracing::info!(id, "task completed")
        }
    })
    .await;

    Ok(())
}
