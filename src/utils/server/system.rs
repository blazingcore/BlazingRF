use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

pub async fn start_server(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(android_request)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[post("/api/init")]
async fn android_request(req_body: String) -> impl Responder {
    println!("Android request: {}", req_body);
    HttpResponse::Ok().body(req_body)
}
