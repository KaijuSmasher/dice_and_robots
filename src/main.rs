mod api;
mod model;

fn main() {
    let _server = api::http::start_server();
    let _tcp = api::tcp::RobotArm::start_tcp("192.168.2.40:30002");
}
