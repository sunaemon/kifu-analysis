use std::error::Error;
use std::fmt;

use iron;
use iron::prelude::*;
use iron::status;
use iron::modifiers;

use router::Router;
use handlebars_iron::Template;

use iron_sessionstorage;
use iron_sessionstorage::traits::*;

use urlencoded::UrlEncodedBody;

use database_lib;
use url;

#[derive(Debug, Clone, PartialEq)]
struct AuthentificationError {
    message: String,
}

impl fmt::Display for AuthentificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl Error for AuthentificationError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub struct Login {
    pub email: String,
}

pub struct UserRoute;

impl UserRoute {
    pub fn new(router: &mut Router) -> UserRoute {
        let prefix = "/users".to_string();
        router.get(format!("{}/signup", prefix), render_signup, "render_signup");
        router.get(format!("{}/login", prefix), render_login, "render_login");

        router.post(format!("{}/signup", prefix), signup, "signup");
        router.post(format!("{}/login", prefix), login, "login");
        router.get(format!("{}/logout", prefix), logout, "logout");

        UserRoute
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

pub fn login_username(req: &mut Request) -> Option<Login> {
    if let Ok(Some(l)) = req.session().get::<Login>() {
        if l.email != "" { Some(l) } else { None }
    } else {
        None
    }
}

pub fn login_user(d: &database_lib::Database,
                  req: &mut Request)
                  -> Result<database_lib::models::User, Box<Error>> {
    let login =
        login_username(req).ok_or(AuthentificationError { message: "No Session".to_string() })?;

    Ok(d.get_user(&login.email)?)
}

fn root(url: &iron::Url) -> iron::Url {
    let mut url = <iron::Url as Into<url::Url>>::into(url.clone());
    url.set_path("/");
    iron::Url::from_generic_url(url).unwrap()
}

fn render_signup(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("users/signup", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_login(req: &mut Request) -> IronResult<Response> {
    if login_username(req).is_some() {
        // Already logged in
        return Ok(Response::with((status::Found, modifiers::Redirect(root(&req.url)))));
    }

    let mut resp = Response::new();
    resp.set_mut(Template::new("users/login", ())).set_mut(status::Ok);
    Ok(resp)
}

fn signup(req: &mut Request) -> IronResult<Response> {
    {
        let formdata = itry!(req.get_ref::<UrlEncodedBody>());

        let email = iexpect!(formdata.get("email"))[0].to_owned();
        let password = iexpect!(formdata.get("password"))[0].to_owned();

        let d = database_lib::Database::new();

        iwtry!(d.create_user(&email, &password));
    }

    Ok(Response::with((status::Found, modifiers::Redirect(root(&req.url)))))
}

fn login(req: &mut Request) -> IronResult<Response> {
    let email = {
        let formdata = itry!(req.get_ref::<UrlEncodedBody>());

        let email = iexpect!(formdata.get("email"))[0].to_owned();
        let password = iexpect!(formdata.get("password"))[0].to_owned();

        let d = database_lib::Database::new();

        iwtry!(d.verify_user(&email, &password));

        email
    };

    req.session().set(Login { email: email })?;
    Ok(Response::with((status::Found, modifiers::Redirect(root(&req.url)))))
}

fn logout(req: &mut Request) -> IronResult<Response> {
    req.session().set(Login { email: "".to_string() })?;
    Ok(Response::with((status::Found, modifiers::Redirect(root(&req.url)))))
}
