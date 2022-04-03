use crate::build_actix::template::TplError;

use actix_web::{ error, HttpResponse };
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{ header, StatusCode };

use askama::Template;
use std::fmt;

// create error message
pub fn crash( lang: String, error_msg: &'static str ) -> Crash {
    Crash { lang, error_msg }
}

// struct to store error values
#[derive(Debug)]
pub struct Crash {
    pub error_msg: &'static str,
    pub lang: String,
}

// Implement Disply to format error message
    impl fmt::Display for Crash {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self.error_msg)
    }
}

// Implement ResponseError to use Error as Response
impl error::ResponseError for Crash {
    fn error_response(&self) -> HttpResponse {
        eprintln!("Error reached: {}", self.error_msg);
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(
                TplError {
                    lang: &self.lang,
                    error_msg: self.error_msg,
                }
                .render()
                .expect("error_tplrender (TplError). Empty page sent to client."),
            )
    }

            fn status_code(&self) -> StatusCode {
                match self.error_msg {
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

    }
