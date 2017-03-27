use iron::prelude::*;
use iron::status;
use router::Router;
use hbs::Template;

pub fn route(req: &mut Request) -> IronResult<Response> {
    let mut router = Router::new();
    //    router.get("new", index, "user_new");
    let mut resp = Response::new();
    resp.set_mut(Template::new("some/path/hello", 0)).set_mut(status::Ok);
    Ok(resp)
}
