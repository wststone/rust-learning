use actix_web::{App, HttpServer};

fn main() {
    HttpServer::new(|| App::new().route("/"));
}
