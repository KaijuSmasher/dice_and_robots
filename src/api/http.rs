use crate::{
    api::{self},
    model::{
        player::Player,
        positions,
        roll::{self, DiceType, Roll},
    },
};
use actix_files::Files;
use actix_web::{
    body::BoxBody, http::header, post, web, App, HttpRequest, HttpResponse, HttpResponseBuilder,
    HttpServer, Responder,
};
use tokio::time::Duration;

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    tokio::spawn(async {
        println!("Starting Tcp Connection on 192.168.2.40:30002");
        let mut _tcp = api::tcp::RobotArm::start_tcp("192.168.2.40:30002")
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        //let _ = api::tcp::RobotArm::send_script(&mut _tcp).await;
    });

    println!("Starting Webserver on Localhost(http://127.0.0.1:8080)");
    HttpServer::new(move || {
        App::new()
            .service(request)
            .service(Files::new("/", "src/webcontent").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/roll")]
async fn request(player: web::Json<Player>) -> impl Responder {
    let _position = positions::Position::create_position(player.dice_to_roll.dice_used);

    Player {
        name: player.name.clone(),
        dice_to_roll: Roll {
            dice_used: player.dice_to_roll.dice_used,
            bonus: player.dice_to_roll.bonus,
            result_rolled: 4,
        },
        history: player.history.clone(),
    }
}

impl Responder for Player {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(body)
    }
}

impl Responder for Roll {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(body)
    }
}

impl Responder for DiceType {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(header::ContentType::json())
            .body(body)
    }
}
