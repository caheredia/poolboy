use axum::response::Html;
use axum::Extension;
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_with::serde_as;
use serde_with::TimestampSeconds;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;
use tracing::debug;

#[derive(Debug, Deserialize, PartialEq)]
struct Stratum {
    hashrate_15m: f32,
    hashrate_1h: f32,
    hashrate_24h: f32,
    shares_found: usize,
    shares_failed: usize,
    connections: usize,
    incoming_connections: usize,
}

#[serde_as]
#[derive(Debug, Deserialize)]
struct NetworkStats {
    #[serde_as(as = "TimestampSeconds<i64>")]
    timestamp: SystemTime,
}

///read file and return String
async fn get_file_str(file_path: PathBuf) -> String {
    debug!("Reading file {}", file_path.display());
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Reads the stratum file contents and returns Stratum struct
async fn get_stratum(stratum_str: String) -> Stratum {
    serde_json::from_str(&stratum_str).unwrap()
}

/// Reads the network stats contents and returns timestamp String
async fn get_network_timestamp(network_str: String) -> NetworkStats {
    let network_stats: NetworkStats = serde_json::from_str(&network_str).unwrap();
    network_stats
}

/// Converts timestamp EPOCH String to DateTime String
async fn convert_to_network_timestamp(network_str: String) -> String {
    let network_stats: NetworkStats = get_network_timestamp(network_str).await;
    let timestamp =
        <std::time::SystemTime as Into<DateTime<Utc>>>::into(network_stats.timestamp).to_string();
    timestamp
}

/// Populates HTML table with stratum JSON
pub async fn get_stratum_table(Extension(data_dir): Extension<PathBuf>) -> Html<String> {
    let page_title = "Local Monero P2Pool stratum";

    let stratum_path = data_dir.join("local").join("stratum");
    let stratum_str = get_file_str(stratum_path).await;
    let stratum = get_stratum(stratum_str).await;
    let hashrate_15m = stratum.hashrate_15m / 1000.0;
    let hashrate_1h = stratum.hashrate_1h / 1000.0;
    let hashrate_24h = stratum.hashrate_24h / 1000.0;
    let shares_found = stratum.shares_found;
    let shares_failed = stratum.shares_failed;
    let connections = stratum.connections;
    let incoming_connections = stratum.incoming_connections;

    // read network file
    let network_path = data_dir.join("network").join("stats");
    let network_str = get_file_str(network_path).await;
    let timestamp = convert_to_network_timestamp(network_str).await;

    let html_table = format!(
        r#"<html lang="en">

    <head>
            <title>Monero P2Pool stats</title>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.0/css/bootstrap.min.css">
            <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.5.0/js/bootstrap.min.js"></script>
        </head>
        <div class="container-fluid">
            <div class="row">
                <div class="col-md-12">
                    <h1>{page_title}</h1> <h2> {timestamp}</h2>
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
        incoming_connections = incoming_connections,
        timestamp = timestamp
    );
    Html(html_table)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_network_timestamp() {
        let network_str = r#"{
                  "difficulty": 326180875193,
                  "hash": "5cc9cc40404608a866c16f4114a396355b82f8148c4285a21cd0937e8b84e776",
                  "height": 2870723,
                  "reward": 605959900000,
                  "timestamp": 1682270152
                  }"#;
        let ts = convert_to_network_timestamp(network_str.to_string()).await;
        assert_eq!(ts, "2023-04-23 17:15:52 UTC");
    }
    #[tokio::test]
    async fn test_get_stratumm() {
        let network_str = r#"{
              "hashrate_15m": 10505,
              "hashrate_1h": 13794,
              "hashrate_24h": 24049,
              "total_hashes": 6021562332,
              "shares_found": 18,
              "shares_failed": 1,
              "average_effort": 122.298,
              "current_effort": 108.724,
              "connections": 2,
              "incoming_connections": 1
              }"#;
        let stratum = get_stratum(network_str.to_string()).await;
        assert_eq!(
            stratum,
            Stratum {
                hashrate_1h: 13794.0,
                hashrate_15m: 10505.0,
                hashrate_24h: 24049.0,
                shares_found: 18,
                shares_failed: 1,
                connections: 2,
                incoming_connections: 1
            }
        );
    }
}
