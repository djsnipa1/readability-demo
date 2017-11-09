extern crate iron;
#[macro_use]
extern crate router;
extern crate mount;
extern crate urlencoded;
extern crate readability;
extern crate staticfile;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod error;

use std::str::FromStr;
use std::env;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::path::Path;
use mount::Mount;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use staticfile::Static;
use urlencoded::UrlEncodedQuery;
use readability::extractor::scrape;

use error::Error;

fn main() {
    let path = Path::new("public");
    let mut mount = Mount::new();
    mount.mount("/web/", Static::new(Path::new(path)));
    let router = router!(
        web: get "/*" => mount,
        readability: get "/readability" => readability,
    );

    let port_str = match env::var("PORT") {
        Ok(n)  => n,
        Err(_) => "8080".to_string()
    };
    let port: u16 = match port_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Faild to parse port");
            return;
        }
    };
    println!("PORT {}", port_str);
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    Iron::new(router).http(SocketAddrV4::new(ip, port)).unwrap();
}

fn text_html() -> Mime {
    Mime::from_str("text/html").ok().unwrap()
}

fn readability(req: &mut Request) -> IronResult<Response> {
    fn readability2(req: &mut Request) -> Result<Response, Error> {
        let ref params = try!(req.get_ref::<UrlEncodedQuery>());
        let url        = try!(params.get("url").ok_or(Error::BadRequest));
        if let Ok(product) = scrape(&url[0]) {
            println!("handle {}", url[0]);
            return Ok(Response::with((status::Ok, text_html(), product.content)));
        }
        println!("unhandle {}", url[0]);
        Ok(Response::with((status::Ok, text_html(), "")))
    }
    readability2(req).map_err(|err| IronError::from(err))
}
