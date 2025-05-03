// mod blockchain;
// use std::sync::Mutex;

// use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// use blockchain::Blockchain;

// struct AppState {
//     blockchain: Mutex<Blockchain>,
// }

// async fn test_api() -> impl Responder {
//     HttpResponse::Ok().body(String::from("Hello World"))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_state = web::Data::new(AppState {
//         blockchain: Mutex::new(Blockchain::init()),
//     });
//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_state.clone())
//             .route("/test_api", web::get().to(test_api))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

mod api;
mod blockchain;

fn main() {
    api::main();
}
