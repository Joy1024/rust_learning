use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/**
 * 配置路由
 */
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


fn files() -> actix_files::Files {
    actix_files::Files::new("/static", ".").show_files_listing()
}
/**
 * 运行服务器
 */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(files())
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
