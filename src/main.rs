use axum::response::Json;
use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde_derive::Deserialize;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod html_utils;
use html_utils::get_stratum_table;
#[tokio::main]
async fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // build app with routes
    let app = Router::new()
        .route("/", get(handler))
        .route("/metrics", get(json))
        .route("/table", get(get_stratum_table));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
// async fn plain_text() -> &'static str {
//     "foo"
// }

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}
async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    info!("Random number: {}", random_number);
    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}
