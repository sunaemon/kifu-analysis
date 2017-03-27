use iron::prelude::*;
use iron::status;
use iron::Handler;
use router::Router;
use hbs::Template;

fn signup(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("users/new", 0)).set_mut(status::Ok);
    Ok(resp)
}

fn logoff(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("some/path/hello", 0)).set_mut(status::Ok);
    Ok(resp)
}

fn login(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("some/path/hello", 0)).set_mut(status::Ok);
    Ok(resp)
}

pub fn route() -> Box<Handler> {
    let mut router = Router::new();
    router.get("signup", signup, "signup");
    router.get("login", login, "login");
    router.get("logoff", logoff, "logoff");
    return Box::new(router);
}
