use iron::prelude::*;
use iron::{status, AfterMiddleware};

use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;

use handlebars_iron::Template;
use std::error::Error;

pub struct ErrorReporter;

impl AfterMiddleware for ErrorReporter {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        info!("after handler");

        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        info!("error handler");

        /*
        let mut data: BTreeMap<String, Json> = BTreeMap::new();

        data.insert("description".to_string(), err.description().to_json());

        let mut resp = Response::new();
        resp.set_mut(Template::with(include_str!("../templates/error.hbs"), data))
            .set_mut(status::BadRequest);

        Ok(resp)
        */
        Ok(Response::with((status::BadRequest, err.description())))
    }
}
