use axum::response::Html;
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::Read;
use tracing::debug;

#[derive(Debug, Deserialize)]
struct Stratum {
    hashrate_15m: f32,
    hashrate_1h: f32,
    hashrate_24h: f32,
    shares_found: usize,
    shares_failed: usize,
    connections: usize,
    incoming_connections: usize,
}

async fn read_stratum() -> Stratum {
    // read file
    let file_path =
        env::current_dir().unwrap().to_str().unwrap().to_owned() + "/" + "src/stratum.json";

    debug!("Reading stratum file {}", file_path);
    let mut stratum_str = String::new();
    File::open(&file_path)
        .unwrap()
        .read_to_string(&mut stratum_str)
        .unwrap();

    serde_json::from_str(&stratum_str).unwrap()
}

/// Populates HTML table with stratum JSON
pub async fn get_stratum_table() -> Html<String> {
    let page_title = "Local Monero P2Pool stratum";
    let stratum = read_stratum().await;
    let hashrate_15m = stratum.hashrate_15m / 1000.0;
    let hashrate_1h = stratum.hashrate_1h / 1000.0;
    let hashrate_24h = stratum.hashrate_24h / 1000.0;
    let shares_found = stratum.shares_found;
    let shares_failed = stratum.shares_failed;
    let connections = stratum.connections;
    let incoming_connections = stratum.incoming_connections;

    let html_table = format!(
        r#"<html lang="en">

    <head>
            <title>Monero P2Pool metrics</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css">
            <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.0/js/bootstrap.min.js"></script>
        </head>
        <div class="container-fluid">
            <div class="row">
                <div class="col-md-12">
                    <h1>{page_title}</h1>
                </div>

            </div>

            <div class="row">
                <div class="col-md-6">
                    <table class="table table-striped">
                        <thead>
                            <tr>
                                <th scope="col">Hashrate [KH/s]</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>15m</td>
                                <td>{hashrate_15m}</td>
                            </tr>
                            <tr>
                                <td>1h</td>
                                <td>{hashrate_1h}</td>
                            </tr>
                            <tr>
                                <td>24h</td>
                                <td>{hashrate_24h}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="row">
                <div class="col-md-6">
                    <table class="table table-striped">
                        <thead>
                            <tr>
                                <th scope="col">Shares [blocks]</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>found</td>
                                <td>{shares_found}</td>
                            </tr>
                            <tr>
                                <td>failed</td>
                                <td>{shares_failed}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

            <div class="row">
                <div class="col-md-6">
                    <table class="table table-striped">
                        <thead>
                            <tr>
                                <th scope="col">Connections</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>Outgoing</td>
                                <td>{connections}</td>
                            </tr>
                            <tr>
                                <td>Incoming</td>
                                <td>{incoming_connections}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

        </html>"#,
        page_title = page_title,
        hashrate_15m = hashrate_15m,
        hashrate_1h = hashrate_1h,
        hashrate_24h = hashrate_24h,
        shares_found = shares_found,
        shares_failed = shares_failed,
        connections = connections,
        incoming_connections = incoming_connections
    );
    Html(html_table)
}
