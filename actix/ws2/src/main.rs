use actix_web::{get, http::header::{ContentDisposition, DispositionType}, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

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

//文件列表
fn files() -> actix_files::Files {
    actix_files::Files::new("/files", ".").show_files_listing()
}

#[get("/files/{filename:.*}")]
async fn file_open(req: HttpRequest) -> Result<actix_files::NamedFile, actix_web::Error> {
    let path: std::path::PathBuf = req.match_info().query("filename").parse().unwrap();
    let mime = mime_guess::from_path(path.clone().as_path()).first_or_octet_stream();
    print!("mime:{}", mime);
    let file = actix_files::NamedFile::open(&path)?;
    Ok(file
        .use_last_modified(true)
        .set_content_type(mime)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
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
            .app_data(file_open)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
