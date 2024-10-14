use config::Config;
use matsya_common::MatsyaRelayResult;

#[allow(dead_code)]
#[async_trait::async_trait]
pub trait App {
    async fn run(&self) -> MatsyaRelayResult<()>;

    fn config(&self) -> &Config;
}

#[allow(dead_code)]
pub fn run_app<A: App>(app: A) {
    let config = app.config();
    let worker_threads = config.get_int("tokio_worker_threads").unwrap_or(4);

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(worker_threads as usize)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { app.run().await.unwrap() })
}
