use std::error::Error;
use std::fmt;

use iron::prelude::*;
use iron::{status, AfterMiddleware};

use handlebars_iron::Template;

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
    pub message: String,
    pub debug_message: String,
    pub description: String,
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

pub fn make_it_ironerror<T: Into<Box<Error>>>(ee: T) -> IronError {
    let e = ee.into();
    IronError::new(IronErrorWrapper {
                       message: format!("{}", e),
                       debug_message: format!("{:?}", e),
                       description: e.description().to_string(),
                   },
                   status::InternalServerError)
}
