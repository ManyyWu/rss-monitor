mod config;
mod logger;
mod error;
mod feed;
mod sink;
mod rss_watcher;
mod macros;
#[cfg(feature = "features")]
mod features;

use crate::logger::init_logger;
use crate::error::{
    Error,
    Result,
};
use crate::config::{
    Args,
    Config,
};

use std::process::exit;
use std::time::Duration;
use tokio::{
    runtime::{
        Runtime,
        Builder,
    },
};
#[cfg(windows)]
use tokio::signal::windows::{ctrl_c, ctrl_break};
#[cfg(unix)]
use tokio::signal::unix::{
    signal,
    SignalKind,
};

fn main() -> Result<()> {
    let args = Args{};
    let config = Config { worker_count: 1 };
    // let args = match parse_args() {
    //     Ok(args) => args,
    //     Err(e) => {
    //         eprintln!("Parsing arguments error: {e}\nUse --help for more information");
    //         exit(1);
    //     }
    // };
    //
    // let config = match Config::load(&args) {
    //     Ok(config) => config,
    //     Err(e) => {
    //         eprintln!("Loading configuration error: {e}\nUse --help for more information");
    //         exit(1);
    //     }
    // };

    let runtime = init_runtime(&config)?;

    init_logger(&config);

    init_services(&runtime, &config)?;

    init_signal_handler(&runtime);

    exit(block(&runtime));
}

fn parse_args() -> Result<Args> {
    Args::parse_from()
}

fn init_services(runtime: &Runtime, config: &Config) -> Result<()> {
    Ok(())
}

fn init_signal_handler(runtime: &Runtime) {
    #[cfg(unix)]
    runtime.spawn(async {
        let mut sigint = signal(SignalKind::interrupt()).expect("Setup SIGINT handler failed");
        let mut sigbreak = signal(SignalKind::terminate()).expect("Setup SIGBREAK handler failed");

        tokio::select!(
            _ = sigint.recv() => eprintln!("Received SIGINT signal"),
            _ = sigbreak.recv() => eprintln!("Received SIGBREAK signal"),
        );
        exit(1);
    });

    #[cfg(windows)]
    runtime.spawn(async {
        let mut sigint = ctrl_c().expect("Setup SIGINT handler failed");
        let mut sigbreak = ctrl_break().expect("Setup SIGBREAK handler failed");

        tokio::select!(
            _ = sigint.recv() => eprintln!("Received SIGINT signal"),
            _ = sigbreak.recv() => eprintln!("Received SIGBREAK signal"),
        );
        exit(1);
    });
}

fn init_runtime(config: &Config) -> std::io::Result<Runtime> {
    Builder::new_multi_thread()
        .worker_threads(config.worker_count)
        .enable_all()
        .build()
}

fn block(runtime: &Runtime) -> i32 {
    // runtime.shutdown_timeout();
    runtime.block_on(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    0
}