use std::{sync::Mutex};
use actix_files::{Files, NamedFile};
use serde::{Deserialize};
use actix_web::{App, HttpResponse, HttpServer, Responder, dev::{ServiceRequest, ServiceResponse}, get, middleware, web};

struct AppState {
    app_name: Mutex<String>,
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    let mut str = data.app_name.lock().unwrap();
    let counter = data.counter.lock().unwrap();
    *str = String::from("test");
    HttpResponse::Ok().body(format!("hello {}:{}!", str, counter))
}

#[derive(Deserialize)]
struct HelloPath {
    test: String,
}

#[get("/{test}")]
async fn test(data: web::Data<AppState>, test: web::Path<HelloPath>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    println!("{:?}", test.test);
    HttpResponse::Ok().body(format!("counter increased: {}", counter))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let app_state = web::Data::new(AppState {
            app_name: Mutex::new(String::from("Actix-web")),
            counter: Mutex::new(0),
        });
        let hello_scope = web::scope("/hello").service(hello).service(test);
        App::new()
            // set logger.
            .wrap(middleware::Logger::default())
            // .wrap(middleware::NormalizePath::default())
            // set app data.
            .app_data(app_state)
            .service(hello_scope)
            // assets.
            .service(Files::new("/assets", "build/assets/"))
            // main.
            .service(Files::new("/", "build/frontend/main/")
            .index_file("index.html")
            // default fallback to index.html.
            .default_handler(|req: ServiceRequest| {
                let (http_req, _payload) = req.into_parts();
                async {
                    let response = NamedFile::open("./build/frontend/main/index.html")?.into_response(&http_req);
                    Ok(ServiceResponse::new(http_req, response))
                }
            })
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
