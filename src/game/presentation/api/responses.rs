use actix_web::HttpResponse;

/// Creates an HTTP response with an image (PNG format)
pub fn create_image_response(image_content: Vec<u8>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .append_header((
            "Cache-Control",
            "private, max-age=0, no-cache, no-store, must-revalidate",
        ))
        .append_header(("Pragma", "no-cache"))
        .body(image_content)
}

/// Creates a redirect response
pub fn create_redirect_response(location: String) -> HttpResponse {
    HttpResponse::PermanentRedirect()
        .append_header(("Location", location))
        .append_header((
            "Cache-Control",
            "private, max-age=0, no-cache, no-store, must-revalidate",
        ))
        .append_header(("Pragma", "no-cache"))
        .finish()
}

/// Creates an HTML response
pub fn create_html_response(content: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content)
}

/// Creates an error response
pub fn create_error_response(message: &str, status: actix_web::http::StatusCode) -> HttpResponse {
    HttpResponse::build(status).body(message.to_string())
}
