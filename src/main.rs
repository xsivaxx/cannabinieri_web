// Reduce code !

#[macro_use]
extern crate lazy_static;

use actix_web::{ web, App, HttpServer };
use actix_files as fs;

mod build_actix;

use crate::build_actix::route;


#[actix_web::main]
// return Result or io::Error
async fn main() -> std::io::Result<()> {
    HttpServer::new( 
        || {
            App::new()
            .service( fs::Files::new("/assets", "./templates/assets/").index_file("index.html"))
            .route("/", web::get().to(route::index))
            .route("/cbd", web::get().to(route::hemp))
            .route("/tekla", web::get().to(route::spider))
            .route("/waspwork", web::get().to(route::rootwork))
            .route("/kaoscube", web::get().to(route::cube))
            .route("/cyberpreneur", web::get().to(route::cyber))
            .route("/greenmachine", web::get().to(route::off))

            .route("/info/{info}", web::get().to(route::info))

            // info windows index
            .route("/robot", web::get().to(route::robot))
            .route("/app", web::get().to(route::app))
            .route("/remote", web::get().to(route::business))
            .route("/network", web::get().to(route::network))
            .route("/cannabinieri", web::get().to(route::cannabinieri))
            .route("/greenhome", web::get().to(route::greenhome))
            .route("/energy", web::get().to(route::get_box_energy))
            .route("/miner", web::get().to(route::get_box_miner))

            // Timelines
            .route("/timeline/{name}", web::get().to(route::timeline))
            // Footer
            .route("/fund", web::get().to(route::pool))
            .route("/contact", web::get().to(route::contact))
            .route("/contact/mail", web::post().to(route::send_form))
            .route("/privacy", web::get().to(route::privacy))
            
        })
        .bind("0.0.0.0:5000")?
        .run()
        .await
}
