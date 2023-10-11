use std::{net::SocketAddr, path::Path};

use axum::{routing::get, Json, Router};
use livegrep::{DateRange, LiveGrep};
use serde::Serialize;

mod livegrep;

#[derive(Serialize)]
struct ResponseData {
    lines: Vec<String>,
}

#[tokio::main]
async fn main() {
    let routes = Router::new().route("/grep", get(handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr).serve(routes.into_make_service());
    if let Err(e) = server.await {
        println!("Error starting server: {e}");
    }
}

async fn handler() -> Json<ResponseData> {
    // TODO: Date and regex and pathbuf from the parameters to the query
    let date_range = DateRange::parse_date_range("01/Jun/1990-01/Aug/2000".to_string());
    if let Ok(range) = date_range {
        let livegrep = LiveGrep {
            regex: "unicomp[0-9].unicomp.net".to_string(),
            date_range: range,
        };

        let test_data = Path::new("./NASA_access_log_test");
        if let Ok(lines) = livegrep.scan_directory(test_data.to_path_buf()) {
            let response = ResponseData { lines };

            return Json(response);
        }
    }

    let empty_response = ResponseData { lines: vec![] };

    Json(empty_response)
}
