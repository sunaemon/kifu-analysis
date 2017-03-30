use iron::prelude::*;
use iron::{status, AfterMiddleware};

use handlebars_iron::Template;
use std::error::Error;

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
