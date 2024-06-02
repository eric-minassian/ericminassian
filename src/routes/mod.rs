use crate::Response;

pub mod index;
pub mod resume;

const NOT_FOUND: &[u8] = include_bytes!("../../templates/404.html");

pub const fn not_found() -> Response<'static> {
    Response::new("HTTP/1.1 404 NOT FOUND", "text/html", NOT_FOUND)
}
