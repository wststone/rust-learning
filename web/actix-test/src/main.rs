use actix_web::{HttpServer, App};

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/")
    });
}
