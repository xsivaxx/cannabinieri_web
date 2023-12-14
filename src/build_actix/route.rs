use actix_web::{ 
    HttpRequest,
    HttpResponse,
    web
};

use actix_web::{error::Error};


use crate::build_actix::template::{self};
use crate::build_actix::error;

use askama::Template;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;

#[derive(Deserialize, Serialize)]
pub struct Data {
    mail: String,
    subject: String,
    comment: String,
}

// This writes formdata to stdout as csv --> needs to be written to file

pub async fn send_form(req: HttpRequest, form: web::Form<Data>) -> Result<HttpResponse,Error> {
    // format data as csv and write to stdout
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let form = form.into_inner();
    wtr.serialize(form).unwrap();
    wtr.flush().unwrap();

    // let mut f = File::create("data.csv").unwrap();

    // write! (f, "{:#?}", wtr.serialize(form).unwrap());


    HttpResponse::Ok()
        .content_type("text/html")
        .body(
         template::TplS {
         title: "",
         content: "",
         lang: &template::get_lang(&req)

         }.render().unwrap()
            
            ).await
    
}


pub async fn pool (req: HttpRequest) -> Result<HttpResponse, Error> {
    HttpResponse::Ok().content_type("text/html")
        .body(
            template::TplPools {
                title: "fund",
                content: "",
                subtitle: "Invest in a node and decide which of our projects comes first. Follow our timelines to see what your contribution made possible.",
                lang: &template::get_lang(&req),
            }.render().unwrap()
        ).await
}


pub async fn info (path: web::Path<String>, req: HttpRequest) -> Result<HttpResponse, Error> {
let info = path.into_inner();
println!("path is {}", info);
let list = ["solar-miner","energy"];

HttpResponse::Ok()
    .content_type("text/html")

    // set teplate context
    .body(
        if info == list[0] {
            template::InSm {
                title: "Solar Miner",
                content: "Info about Solar Miner",
                lang: &template::get_lang(&req),
            }.render().unwrap()
            //load context for info page
            } else if info == list [1] {
                
                template::InSm {
                    title: "Energy",
                    content: "Info about Energy",
                    lang: &template::get_lang(&req),
                }.render().unwrap()
            }
            else {
                format! ("Error {}", info)
                
            }).await

}


pub async fn timeline(path: web::Path<String>, req: HttpRequest) -> Result<HttpResponse, Error> {
    let name = path.into_inner();
    let list = ["waspwork","cyberpreneur", "tekla", "green-machine", "solar-miner", "energy", "kaos-cube", "cbd"];
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            if name == list[0] {
                template::TlWw {
                    content: "content for waspwork",
                    lang: &template::get_lang(&req),
                    title: "Waspwork| Timeline",
                }.render().unwrap()

            } else if name == list [1] {
        
                template::TlCp{
                    content: "timeline cyberpreneur",
                    lang: &template::get_lang(&req),
                    title: "Cyberpreneur| Timeline",
                }.render().unwrap()

            } else if name == list [2] {
                
                template::TlTk{
                    content: "timeline tekla",
                    lang: &template::get_lang(&req),
                    title: "Tekla| Timeline",
                }.render().unwrap()

            } else if name == list [3] {

                template::TlGm{
                    content: "timeline green machine",
                    lang: &template::get_lang(&req),
                    title: "Green Machine| Timeline",
                }.render().unwrap()

            } else if name == list [4] {
            
                template::TlSm{
                    content: "timeline solar miner",
                    lang: &template::get_lang(&req),
                    title: "Solar Miner| Timeline",

                }.render().unwrap()


            } else if name == list [5] {
                
                template::TlEn {
                    content: "timeline energy",
                    lang: &template::get_lang(&req),
                    title: "Energy| Timeline",
                }.render().unwrap()

            } else if name == list [6] {
               
                template::TlKc{
                    content: "timeline kaos cube",
                    lang: &template::get_lang(&req),
                    title: "Kaos Cube| Timeline",
                }.render().unwrap()


            } else if name == list [7] {
            
                template::TlCb{
                    content: "timeline cbd",
                    lang: &template::get_lang(&req),
                    title: "CBD| Timeline",
                }.render().unwrap()


            }
            else {

                format! ("Whatever {}", name)
            }
        ).await
    }


pub async fn index( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render index template
        template::TplIndex {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}


pub async fn rootwork( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplRootWork {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn hemp( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplHemp {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn spider( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplSpider {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn cube( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplCube {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn cyber( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplCyber {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn off( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplOff {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}


pub async fn robot( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplRobot {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn app( req: HttpRequest ) -> Result<HttpResponse, Error> {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        template::TplApp {
            lang : &template::get_lang(&req),
        }
        
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn business( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplModel {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn network( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplNetwork {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn cannabinieri( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplCanna {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}

pub async fn greenhome( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplHome {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}


pub async fn get_box_energy ( req: HttpRequest ) -> Result<HttpResponse, Error> {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        template::TplBoxEnergy{
            lang : &template::get_lang(&req),
        }
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}


pub async fn get_box_miner ( req: HttpRequest ) -> Result<HttpResponse, Error> {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        template::TplBoxMiner {
            lang : &template::get_lang(&req),
        }
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}


// pub async fn timelines( req: HttpRequest ) -> Result<HttpResponse, Error> {
    
    // let uri : &Uri = req.uri();
    // print!(" uri is {} ", uri);

        // let body = match uri.path() {

            // "/cyberpreneur-timeline" => {
                // println!("path match : {}", uri.path());
                // template::TplCyber{lang: &template::get_lang(&req),}.render()


            // "/waspwork-timeline" => {
                // println!("path match : {}", uri.path());
                // template::TplRootWork{lang: &template::get_lang(&req),}.render()
        
        // },

    // };

    // HttpResponse::Ok()
        // returns HttpResponseBuilder
    // .content_type("text/html")
        // set body of HttpResponse
    // .body(
        // set body to template
        // type Result<String,askama::Error>
        // body.map_err( |e| {
        // apply funtion to Result<Error>, leave Result<Ok> untouched
        // print error to stdout
        // eprint!("{}", e );
        // use sncf custom error
        // error::crash(
            // template::get_lang((&req), "error rendering template"),
        // })

    // )?, // return Ok value
    // .await
// }






// footer Routes

pub async fn contact( req: HttpRequest ) -> Result<HttpResponse, Error> {
    // if response Ok return HttpResponseBuilder
    HttpResponse::Ok()
    // set response content type html
    .content_type("text/html")
    // set response body to template context
    .body(
        // render template context
        template::TplContact {
            // lang to value of Accept-Language header
            lang : &template::get_lang(&req),
        }
        
        // render template context into String
        .render()
        .map_err( |e| {
            eprintln!("error_tplrender : {}", e );
            error::crash( template::get_lang(&req), "error_tplrender" )
        })?,
    ).await
}
// add errot handling
pub async fn privacy (req: HttpRequest) -> Result<HttpResponse, Error> {
    HttpResponse::Ok()
    .content_type("text/html")
    .body(
        template::TplPr {
            lang: &template::get_lang(&req),
        }.render().unwrap()
        ).await
}



