fn main() {
    // Install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_env("KVS_LOG"))
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc3339())
        .with_thread_ids(true)
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .on_thread_start(|| tracing::trace!("thread start"))
        .on_thread_stop(|| tracing::trace!("thread stop"))
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            run().await;
        })
}

async fn run() {
    if let Err(err) = run_inner().await {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}

async fn run_inner() -> kvs::Result<()> {
    use kvs::cli;

    let m = cli::new().get_matches();
    match m.subcommand() {
        (cli::ECHO, Some(sm)) => cli::echo::run(sm).await,
        (cli::SERVER, Some(sm)) => cli::server::run(sm).await,
        (_, _) => unreachable!(),
    }
}