use actix_web::*;
use std::fmt::{self, Debug, Display};
pub type ARes<T> = Result<T, AErr>;

#[derive(Debug)]
pub struct AErr(anyhow::Error);

impl Display for AErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E: Into<anyhow::Error>> From<E> for AErr {
    fn from(e: E) -> AErr {
        AErr(e.into())
    }
}

impl actix_web::ResponseError for AErr {}
