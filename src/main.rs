use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let msg = format!("Hello {}!", name);
    HttpResponse::Ok().body(msg)
}

// async fn greet() -> impl Responder {
//     HttpResponse::Ok().body("Hello, Actix!")
// }

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(ping).service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::web::ServiceConfig;
    use actix_web::{test, App};

    // Function to configure services for the test app
    // It mirrors the configuration of the actual app
    fn test_config(cfg: &mut ServiceConfig) {
        cfg.service(ping).service(greet);
    }

    #[actix_rt::test]
    async fn test_ping_route() {
        // Setup test server with the ping service
        let mut app = test::init_service(App::new().configure(test_config)).await;
        // Create a request to the "/ping" endpoint
        let req = test::TestRequest::get().uri("/ping").to_request();
        // Execute request
        let resp = test::call_service(&mut app, req).await;

        // Check if the response status is 200
        assert!(resp.status().is_success());
        // Extract response body
        let body = test::read_body(resp).await;
        // Convert body to string and compare
        assert_eq!(body, "pong");
    }

    #[actix_rt::test]
    async fn test_greet_route() {
        let mut app = test::init_service(App::new().configure(test_config)).await;
        // Replace `{name}` with an actual name, e.g., "Alice"
        let req = test::TestRequest::get().uri("/hello/Alice").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello Alice!");
    }
}
