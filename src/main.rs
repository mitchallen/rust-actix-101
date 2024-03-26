use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use actix_web::{test, App};

    #[actix_rt::test]
    async fn test_example() {
        // Your async test code here
    }
}
