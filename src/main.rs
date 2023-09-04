use axum::{routing::get, Extension, Router};
use clap::Parser;
use std::net::SocketAddr;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utils::get_stratum_table;

mod utils;

/// Simple program to display the stratum table
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data directory containing P2pool data
    #[arg(short, long, value_name = "DIR")]
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    info!("Reading from directory: {}", args.data_dir.display());

    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::INFO)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // build app with routes
    let app = Router::new()
        .route("/", get(get_stratum_table))
        .layer(Extension(args.data_dir));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
