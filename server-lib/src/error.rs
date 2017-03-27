use iron::prelude::*;
use iron::{status, AfterMiddleware};

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

        use std::collections::BTreeMap;
        use rustc_serialize::json::{ToJson, Json};
        let mut data: BTreeMap<String, Json> = BTreeMap::new();

        data.insert("description".to_string(), err.description().to_json());

        let mut resp = Response::new();
        resp.set_mut(Template::with(include_str!("../templates/error.hbs"), data))
            .set_mut(status::BadRequest);

        Ok(resp)
    }
}
