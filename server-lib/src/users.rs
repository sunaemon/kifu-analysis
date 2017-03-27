use iron::prelude::*;
use iron::status;
use iron::Handler;
use iron::modifiers::Redirect;

use hyper;
use iron;

use router::Router;
use handlebars_iron::Template;

use iron_sessionstorage;
use iron_sessionstorage::traits::*;

use urlencoded::UrlEncodedBody;

use database_lib;
use url;

pub struct Login {
    email: String,
}

pub struct UserRoute {
    router: Router,
}

impl UserRoute {
    pub fn new() -> UserRoute {
        let mut router = Router::new();
        router.get("signup", signup, "signup");
        router.post("signup", signup_post, "signup_post");
        router.get("login", login, "login");
        router.post("login", login_post, "login_post");
        router.get("logout", logout, "logout");
        UserRoute { router: router }
    }
}

impl Handler for UserRoute {
    fn handle(self: &Self, req: &mut Request) -> IronResult<Response> {
        self.router.handle(req)
    }
}

impl iron_sessionstorage::Value for Login {
    fn get_key() -> &'static str {
        "logged_in_user"
    }
    fn into_raw(self) -> String {
        self.email
    }
    fn from_raw(value: String) -> Option<Self> {
        if value.is_empty() {
            None
        } else {
            Some(Login { email: value })
        }
    }
}

fn root(url: &iron::Url) -> iron::Url {
    let mut url = <iron::Url as Into<url::Url>>::into(url.clone());
    url.set_path("/");
    iron::Url::from_generic_url(url).unwrap()
}

fn signup(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    //resp.set_mut(Template::new("users/signup", 0)).set_mut(status::Ok);
    resp.set_mut(Template::with(include_str!("../templates/users/signup.hbs"), 0))
        .set_mut(status::Ok);
    Ok(resp)
}

fn signup_post(req: &mut Request) -> IronResult<Response> {
    {
        let formdata = itry!(req.get_ref::<UrlEncodedBody>());

        let email = iexpect!(formdata.get("email"))[0].to_owned();
        let password = iexpect!(formdata.get("password"))[0].to_owned();

        let d = database_lib::Database::new();

        itry!(d.create_user(&email, &password));
    }

    Ok(Response::with((status::Found, Redirect(root(&req.url)))))
}

fn login(req: &mut Request) -> IronResult<Response> {
    if try!(req.session().get::<Login>()).is_some() {
        // Already logged in
        return Ok(Response::with((status::Found, Redirect(root(&req.url)))));
    }

    let mut resp = Response::new();
    //    resp.set_mut(Template::new("users/login", 0)).set_mut(status::Ok);
    resp.set_mut(Template::with(include_str!("../templates/users/login.hbs"), 0))
        .set_mut(status::Ok);
    Ok(resp)
}

fn login_post(req: &mut Request) -> IronResult<Response> {
    let email = {
        let formdata = itry!(req.get_ref::<UrlEncodedBody>());

        let email = iexpect!(formdata.get("email"))[0].to_owned();
        let password = iexpect!(formdata.get("password"))[0].to_owned();

        let d = database_lib::Database::new();

        itry!(d.verify_user(&email, &password));

        email
    };

    try!(req.session().set(Login { email: email }));
    Ok(Response::with((status::Found, Redirect(root(&req.url)))))
}

fn logout(req: &mut Request) -> IronResult<Response> {
    try!(req.session().clear());
    Ok(Response::with((status::Found, Redirect(root(&req.url)))))
}