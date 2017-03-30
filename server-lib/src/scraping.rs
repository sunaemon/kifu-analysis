use std::io::Read;
use std::error::Error;
use std::str;
use std::fmt::{self, Formatter};

//use regex::Regex;
use regex::bytes::Regex;

use hyper::{self, Client};
use hyper::status::StatusCode;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::Url;
use hyper::client::IntoUrl;

#[derive(Debug, Clone, PartialEq)]
pub enum ScrapingError {
    Status(StatusCode),
    General(String),
}

impl fmt::Display for ScrapingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            &ScrapingError::Status(status) => status.fmt(f),
            &ScrapingError::General(ref s) => s.fmt(f),
        }
    }
}

impl Error for ScrapingError {
    fn description(&self) -> &str {
        "invalid status"
    }
}

fn read_https(url: Url) -> Result<Vec<u8>, Box<Error>> {
    let tls = NativeTlsClient::new()?;
    let connector = HttpsConnector::new(tls);
    let client = Client::with_connector(connector);

    let mut res = client.get(url).send()?;

    if res.status != hyper::Ok {
        return Err(Box::new(ScrapingError::Status(res.status)));
    }

    let mut buf = Vec::new();
    res.read_to_end(&mut buf)?;

    Ok(buf)
}

pub fn shougiwars_history_url(user: &str, start: u32) -> Result<Url, Box<Error>> {
    Ok(format!("http://shogiwars.heroz.jp/users/history/{}?start={}",
               user,
               start).into_url()?)
}

pub fn shougiwars_game_url(game: &str) -> Result<Url, Box<Error>> {
    Ok(format!("http://kif-pona.heroz.jp/games/{}", game).into_url()?)
}

pub fn scrape_shougiwars_history(s: &[u8]) -> Result<Vec<String>, Box<Error>> {
    let mut games = Vec::new();

    let re = Regex::new(r#"//kif-pona.heroz.jp/games/([^?"]*)"#).unwrap();

    for cap in re.captures_iter(s) {
        games.push(str::from_utf8(&cap[1])?.to_string());
    }
    Ok(games)
}

pub fn scrape_shougiwars_game(s: &[u8]) -> Result<String, Box<Error>> {
    let re = Regex::new(r#"receiveMove\("([^"]*)"\)"#).unwrap();

    for cap in re.captures_iter(s) {
        return Ok(str::from_utf8(&cap[1])?.to_string());
    }
    Err(Box::new(ScrapingError::General("no match".to_string())))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn it_works() {
        assert_eq!(scrape_shougiwars_history(include_bytes!("../test/history")).unwrap(),
                   vec!["sunaemon0-kumagayaryu-20170329_174102",
                        "sunaemon0-jjmdtgjjmdtg-20170329_140528",
                        "sunaemon0-sknow-20170329_135526",
                        "sunaemon0-masaisan88-20170327_154020",
                        "kentarou31-sunaemon0-20170327_135043",
                        "kaede0926-sunaemon0-20170327_134648",
                        "sunaemon0-tutomu19640422-20170325_140834",
                        "sunaemon0-mahinute-20170325_135235",
                        "sunaemon0-Takahiro_s-20170325_123724",
                        "Rettosei-sunaemon0-20170324_111157"]);

        assert_eq!(scrape_shougiwars_game(include_bytes!("../test/game")).unwrap(),
                   "+5756FU,L600\t-3334FU,L599\t+2858HI,L598\t-6152KI,L597\t+7776FU,\
                    L596\t-5142OU,L594\t+8877KA,L594\t-4232OU,L592\t+6766FU,L585\t-7162GI,\
                    L590\t+7786KA,L584\t-4344FU,L588\t+7978GI,L582\t-2233KA,L587\t+7877GI,\
                    L582\t-3222OU,L586\t+5948OU,L580\t-1112KY,L585\t+3938GI,L579\t-2211OU,\
                    L584\t+4839OU,L578\t-3122GI,L584\t+5655FU,L576\t-5243KI,L583\t+7675FU,\
                    L573\t-4132KI,L582\t+7776GI,L572\t-8384FU,L578\t+7665GI,L566\t-8485FU,\
                    L577\t+8668KA,L558\t-8284HI,L574\t+8977KE,L553\t-9394FU,L573\t+3928OU,\
                    L543\t-8586FU,L568\t+8786FU,L540\t-8486HI,L567\t+5554FU,L531\t-5354FU,\
                    L562\t+6554GI,L530\t-4354KI,L561\t+5854HI,L528\t-0053FU,L560\t+5457HI,\
                    L521\t-8689RY,L555\t+6958KI,L516\t-8999RY,L548\t+7765KE,L515\t-0064GI,\
                    L507\t+6886KA,L492\t-4445FU,L498\t+7574FU,L480\t-7374FU,L495\t+0052KI,\
                    L465\t-3366KA,L488\t+5756HI,L452\t-6455GI,L485\t+5636HI,L439\t-4546FU,\
                    L453\t+5262KI,L429\t-4647TO,L449\t+5847KI,L419\t-0046FU,L443\t+6553NK,\
                    L405\t-4647TO,L433\t+3847GI,L395\t-9949RY,L426\t+1716FU,L390\t-6639UM,\
                    L423\t+2818OU,L376\t-0017KY,L418\t+2917KE,L373\t-0028KI,\
                    L417\tGOTE_WIN_CHECKMATE");
    }
}
