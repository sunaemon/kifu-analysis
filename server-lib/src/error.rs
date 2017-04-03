use iron::prelude::*;
use iron::{status, AfterMiddleware};

use handlebars_iron::Template;
use std::error::Error;
use std::fmt;

pub struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        info!("error handler: {:?}", err);

        use rustc_serialize::json::{ToJson, Object};
        let mut data = Object::new();
        data.insert("description".to_string(), err.description().to_json());

        let mut resp = Response::new();
        resp.set_mut(Template::new("error", data)).set_mut(status::BadRequest);

        Ok(resp)
    }
}

pub struct IronErrorWrapper {
    message: String,
    debug_message: String,
    description: String,
}

impl IronErrorWrapper {
    pub fn new(b: Box<Error>) -> IronErrorWrapper {
        IronErrorWrapper {
            message: format!("{}", b),
            debug_message: format!("{:?}", b),
            description: b.description().to_string(),
        }
    }
}

impl fmt::Display for IronErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for IronErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.debug_message)
    }
}

impl Error for IronErrorWrapper {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[macro_export]
macro_rules! iwtry {
    ($result:expr) => (iwtry!($result, $crate::status::InternalServerError));

    ($result:expr, $modifier:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => return ::std::result::Result::Err(
            IronError::new($crate::error::IronErrorWrapper::new(::std::convert::From::from(err)), $modifier))
    })
}
