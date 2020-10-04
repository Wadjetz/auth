use actix_web::{http::header, web::HttpResponse};

pub fn redirect_response(url: &str) -> HttpResponse {
    HttpResponse::Found().header(header::LOCATION, url).finish()
}
