mod api;
mod model;

fn main() {
    let _server = api::http::start_server();
}
