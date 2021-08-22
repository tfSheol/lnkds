use serde::{Deserialize};
use async_trait::async_trait;
use rand::{thread_rng, Rng};
use std::iter;

pub struct Profile {
  pub li_at: String,
  pub user_identity: String,
}

// #[derive(Serialize, Deserialize)]
// struct Obj {
//     items: Vec<Issue>,
// }

// #[derive(Deserialize, Debug)]
// struct Issue {
//     title: String,
// }


#[async_trait]
pub trait Voyager {
  fn new(&self) -> String;
  fn csrf(&self) -> String;
  async fn request(&self) -> Result<String, reqwest::Error>;
}

#[async_trait]
impl Voyager for Profile {
  fn new(&self) -> String {
    format!("{} / ({})", self.li_at, self.user_identity)
  }

  fn csrf(&self) -> String {
    let mut rng = thread_rng();
    let rand_string: String = iter::repeat(())
      .map(|_| {
        let idx = rng.gen_range(0..10);
        b"0123456789"[idx] as char
      })
      .map(char::from)
      .take(19)
      .collect();
    rand_string
  }

  // https://www.linkedin.com/voyager/api/identity/dash/profiles?q=memberIdentity&memberIdentity={}
  // &decorationId=com.linkedin.voyager.dash.deco.identity.profile.FullProfileWithEntities-84
  async fn request(&self) -> Result<String, reqwest::Error> {
    let csrf = self.csrf();
    let client = reqwest::Client::builder().gzip(true).build()?;
    let result = client
      .get(format!("https://www.linkedin.com/voyager/api/identity/dash/profiles?q=memberIdentity&memberIdentity={}&decorationId=com.linkedin.voyager.dash.deco.identity.profile.FullProfileWithEntities-84", self.user_identity))
      .header("accept", "application/vnd.linkedin.normalized+json+2.1")
      .header("accept-encoding", "gzip, deflate, br")
      .header("cache-control", "no-cache")
      .header("content-type", "application/json; charset=utf-8")
      .header("cookie", format!("li_at={}; JSESSIONID=\"ajax:{}\"", self.li_at, csrf))
      .header("csrf-token", format!("ajax:{}", csrf))
      .header("pragma", "no-cache")
      .send()
      .await?;
    if !result.status().is_success() {
      Ok(format!("error! bad li_at"))
    } else {
      let t = result.text_with_charset("utf-8").await?;
      Ok(serde_json::from_str(&*t).unwrap())
    }
  }
}
