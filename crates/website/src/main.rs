use std::env;
use std::net::SocketAddr;
use std::time::Duration;

use color_eyre::eyre;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use gardyn_management::router::build_router;
use gardyn_management::server::setup_server;
use gardyn_management::state::ApplicationState;
use gardyn_management::states::config::Config;
use gardyn_management::utils;
use tokio::signal;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;
use tracing::{event, Level};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[expect(clippy::unnecessary_wraps)]
fn build_configs() -> Result<Config, eyre::Report> {
    let config = Config {
        bind_to: SocketAddr::from(([0, 0, 0, 0], 3000)),
    };

    Ok(config)
}

/// starts all the tasks, such as the web server, the key refresh, ...
/// ensures all tasks are gracefully shutdown in case of error, ctrl+c or sigterm
async fn start_tasks() -> Result<(), eyre::Report> {
    let config = build_configs()?;

    // this channel is used to communicate between
    // tasks and this function, in the case that a task fails, they'll send a message on the shutdown channel
    // after which we'll gracefully terminate other services
    let token = CancellationToken::new();

    let manager = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL")?,
    );

    let pool: Pool<AsyncPgConnection> = Pool::builder(manager).build()?;

    let application_state = ApplicationState::new(config, pool);

    let tasks = TaskTracker::new();

    {
        let token = token.clone();

        let bind_to = application_state.config.bind_to;
        let router = build_router(application_state);

        tasks.spawn(async move {
            let _guard = token.clone().drop_guard();

            let server = setup_server(bind_to, router, token.clone()).await;

            match server {
                Err(e) => {
                    event!(Level::ERROR, message = "Server shutting down", ?e);
                },
                Ok(()) => {
                    event!(Level::INFO, "Server shutting down gracefully");
                },
            }
        });
    }

    // now we wait forever for either
    // * sigterm
    // * ctrl + c
    // * a message on the shutdown channel, sent either by the server task or
    // another task when they complete (which means they failed)
    tokio::select! {
        r = utils::wait_for_sigterm() => {
            if let Err(e) = r {
                event!(Level::ERROR, message = "Failed to register SIGERM handler, aborting", ?e);
            } else {
                // we completed because ...
                event!(Level::WARN, message = "Sigterm detected, stopping all tasks");
            }
        },
        r = signal::ctrl_c() => {
            if let Err(e) = r {
                event!(Level::ERROR, message = "Failed to register CTRL+C handler, aborting", ?e);
            } else {
                // we completed because ...
                event!(Level::WARN, message = "CTRL+C detected, stopping all tasks");
            }
        },
        () = token.cancelled() => {
            event!(Level::ERROR, message = "Underlying task stopped, stopping all others tasks");
        },
    };

    // announce cancel
    token.cancel();

    // close the tracker, otherwise wait doesn't work
    tasks.close();

    // wait for the task that holds the server to exit gracefully
    // it listens to shutdown_send
    if timeout(Duration::from_millis(10000), tasks.wait())
        .await
        .is_err()
    {
        event!(
            Level::ERROR,
            message = "Tasks didn't stop within allotted time!"
        );
    }

    event!(Level::INFO, message = "Goodbye");

    Ok(())
}

fn init_tracing() -> Result<(), eyre::Report> {
    let main_filter = EnvFilter::builder()
        .parse(env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| {
            format!("INFO,{}=TRACE", env!("CARGO_PKG_NAME").replace('-', "_"))
        }))?;

    let layers = vec![
        #[cfg(feature = "tokio-console")]
        console_subscriber::ConsoleLayer::builder()
            .with_default_env()
            .spawn()
            .boxed(),
        tracing_subscriber::fmt::layer()
            .with_filter(main_filter)
            .boxed(),
        tracing_error::ErrorLayer::default().boxed(),
    ];

    Ok(tracing_subscriber::registry().with(layers).try_init()?)
}

fn main() -> Result<(), eyre::Report> {
    color_eyre::config::HookBuilder::default().install()?;

    dotenvy::dotenv()?;

    init_tracing()?;

    // initialize the runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // start service
    let result: Result<(), eyre::Report> = rt.block_on(start_tasks());

    result
}
