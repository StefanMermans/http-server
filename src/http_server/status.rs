// See: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status
pub enum Status {
    // Successful responses
    Ok200,
    Created201,
    Accepted202,

    // Client error responses
    BadRequest400,
    Unauthorized401,
    NotFound404,

    // Server error responses
    InternalServerError500,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Ok200 => "200 Ok",
            Status::Created201 => "201 Created",
            Status::Accepted202 => "202 Accepted",
            Status::BadRequest400 => "400 Bad Request",
            Status::Unauthorized401 => "401 Unauthorized",
            Status::NotFound404 => "404 Not Found",
            Status::InternalServerError500 => "500 Internal Server Error",
        }
    }
}