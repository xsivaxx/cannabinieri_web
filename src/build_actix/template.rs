// Askama Template
use askama::Template;

use actix_web::HttpRequest;


#[derive(Template)]
#[template(path="pools.html")]
pub struct TplPools<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub lang: &'a str,
    pub subtitle: &'a str,
}

// context for info pages

#[derive(Template)]
#[template(path="info.html")]
pub struct InSm<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlWw<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlCp<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlTk<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlGm<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}



#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlSm<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlEn<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlKc<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="timeline.html")]
pub struct TlCb<'a> {
    pub content: &'a str,
    pub lang: &'a str,
    pub title: &'a str,
}


#[derive(Template)]
#[template(path="submit.html")]
pub struct TplS<'a> {
    pub lang: &'a str,
    pub content: &'a str,
    pub title: &'a str,
}







// define struct linked to template context
#[derive(Template)]
// define path to use template in
#[template(path="index.html")]
pub struct TplIndex<'a> {
   
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="app.html")]
pub struct TplRootWork<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="hemp.html")]
pub struct TplHemp<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="spider.html")]
pub struct TplSpider<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="cube.html")]
pub struct TplCube<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="cyber.html")]
pub struct TplCyber<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
// define path to use template in
#[template(path="offgrid.html")]
pub struct TplOff<'a> {
    // replace with lang
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="policy.html")]
pub struct TplPr<'a> {
    pub lang: &'a str,
}


// index info windows

#[derive(Template)]
#[template(path="tekla.html")]
pub struct TplRobot<'a> {
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="rootwork.html")]
pub struct TplApp<'a> {
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="virtual.html")]
pub struct TplModel<'a> {
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="network.html")]
pub struct TplNetwork<'a> {
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="canna.html")]
pub struct TplCanna<'a> {
    pub lang: &'a str,
}

#[derive(Template)]
#[template(path="greenhome.html")]
pub struct TplHome<'a> {
    pub lang: &'a str,
}


#[derive(Template)]
#[template(path="energy_box.html")]
pub struct TplBoxEnergy<'a> {
    pub lang: &'a str,
}


#[derive(Template)]
#[template(path="miner_box.html")]
pub struct TplBoxMiner<'a> {
    pub lang: &'a str,
}





// linked to error template
#[derive(Template)]
#[template(path="error.html")]
pub struct TplError<'a> {
    pub lang: &'a str,
    pub error_msg: &'a str,

}

// footer pages

#[derive(Template)]
#[template(path="contact.html")]
pub struct TplContact<'a> {
    pub lang: &'a str,

}

// Get Language from Client Header
pub fn get_lang( req: &HttpRequest ) -> String {
    // get Accept-Language header
    if let Some( accept_language ) = req.headers().get( "Accept-Language" ) {
        if let Ok( s ) = accept_language.to_str() {
            println!("language is {}", s);
            // take first two characters of header
            return s.to_lowercase()[..2].to_string();
        }
    }
    
    String::from("en")
}

// Define askama filter 
mod filters {
    use crate::build_actix::config::LOC;

    pub fn translate(key: &str, lang: &str) -> askama::Result<String> {

        let translation = LOC.get(key).ok_or_else(|| {

            eprintln!("no translation available for key {}", key);
            askama::Error::from(std::fmt::Error)
        })?;
        Ok( String::from(
            translation.get( lang )
            .unwrap_or( translation.get( "en" ).ok_or_else(|| {
                eprintln!("no translation available for lan {} in key {}", lang, key );
                askama::Error::from(std::fmt::Error)
            })?)
            .as_str()
            .ok_or_else( || {
                eprintln!("lang {} in key {} is not a string", lang, key );
                askama::Error::from(std::fmt::Error)
            })?,
        ))
    }

}



