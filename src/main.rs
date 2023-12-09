use actix_files as fs;
use actix_web::{App, HttpServer};

mod index;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(move || {
        App::new()
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(index::index)
            .service(api::get_distro_list)
            .service(api::get_distro_update_date)
            .service(api::get_distro_status)
    })
    .workers(4)
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
