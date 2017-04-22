use std::error::Error;

use iron;
use iron::prelude::*;
use iron::status;
use iron::modifiers;

use router::Router;
use handlebars_iron::Template;

use iron_sessionstorage;
use iron_sessionstorage::traits::*;

use database_lib;
use url;
use bodyparser;

use super::error::make_it_ironerror;

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
    if let Ok(l) = req.session().get::<Login>() {
        if let Some(ll) = l {
            return if ll.email != "" { Some(ll) } else { None };
            //l
        }
    }

    None
}

pub fn login_user(d: &database_lib::Database,
                  req: &mut Request)
                  -> Result<database_lib::models::User, Box<Error>> {
    let login = login_username(req).ok_or("No Session".to_string())?;

    Ok(d.get_user(&login.email)?)
}

fn root(url: &iron::Url) -> Result<iron::Url, Box<Error>> {
    let mut url = <iron::Url as Into<url::Url>>::into(url.clone());
    url.set_path("/");
    Ok(iron::Url::from_generic_url(url)?)
}

fn render_signup(_req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    resp.set_mut(Template::new("users/signup", ())).set_mut(status::Ok);
    Ok(resp)
}

fn render_login(req: &mut Request) -> IronResult<Response> {
    if login_username(req).is_some() {
        // Already logged in
        let root = root(&req.url).map_err(make_it_ironerror)?;
        return Ok(Response::with((status::Found, modifiers::Redirect(root))));
    }

    let mut resp = Response::new();
    resp.set_mut(Template::new("users/login", ())).set_mut(status::Ok);
    Ok(resp)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailPassword {
    pub email: String,
    pub password: String,
}

fn signup(req: &mut Request) -> IronResult<Response> {
    {
        let formdata = iexpect!(itry!(req.get::<bodyparser::Struct<EmailPassword>>()));

        let d = database_lib::Database::new();

        d.create_user(&formdata.email, &formdata.password).map_err(make_it_ironerror)?;
    }

    let root = root(&req.url).map_err(make_it_ironerror)?;
    Ok(Response::with((status::Found, modifiers::Redirect(root))))
}

fn login(req: &mut Request) -> IronResult<Response> {
    let email = {
        let formdata = iexpect!(itry!(req.get::<bodyparser::Struct<EmailPassword>>()));

        let d = database_lib::Database::new();

        d.assume_user(&formdata.email, &formdata.password).map_err(make_it_ironerror)?;

        formdata.email
    };

    req.session().set(Login { email: email })?;
    let root = root(&req.url).map_err(make_it_ironerror)?;
    Ok(Response::with((status::Found, modifiers::Redirect(root))))
}

fn logout(req: &mut Request) -> IronResult<Response> {
    req.session().set(Login { email: "".to_string() })?;
    //req.session().clear()?;
    let root = root(&req.url).map_err(make_it_ironerror)?;
    Ok(Response::with((status::Found, modifiers::Redirect(root))))
}
