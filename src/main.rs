mod api;
mod model;

use api::http;

fn main() {
    let _server = http::start_server();
}
