use std::str::FromStr;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "get" => Ok(Method::GET),
            "put" => Ok(Method::PUT),
            "post" => Ok(Method::POST),
            "patch" => Ok(Method::PATCH),
            "delete" => Ok(Method::DELETE),
            _ => Err(())
        }
    }
}
