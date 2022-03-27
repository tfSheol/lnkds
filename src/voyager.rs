use async_trait::async_trait;
use rand::{thread_rng, Rng};
use std::iter;

pub struct Profile {
  pub li_at: String,
  pub user_identity: String,
}

#[async_trait]
pub trait Voyager {
  fn new(&self) -> String;
  fn csrf(&self) -> String;
  async fn request(&self) -> Result<serde_json::Value, reqwest::Error>;
  async fn me(&self) -> Result<serde_json::Value, reqwest::Error>;
  async fn experiences(&self, entity: String) -> Result<serde_json::Value, reqwest::Error>;
  // fn getExperiences() -> Vec<_>;
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

  async fn me(&self) -> Result<serde_json::Value, reqwest::Error> {
    let csrf = self.csrf();
    let client = reqwest::Client::builder().gzip(true).build()?;
    let result = client
    .get("https://www.linkedin.com/voyager/api/me")
    .header("accept", "application/vnd.linkedin.normalized+json+2.1")
    .header("accept-encoding", "gzip, deflate, br")
    .header("accept-language", "en-FR,en;q=0.9,fr-FR;q=0.8,fr;q=0.7,en-US;q=0.6")
    .header("cookie", format!("li_at={}; JSESSIONID=\"ajax:{}\"", self.li_at, csrf))
    .header("csrf-token", format!("ajax:{}", csrf))
    .header("pragma", "no-cache")
    .header("x-restli-protocol-version", "2.0.0")
    .send()
    .await?;
    match result.error_for_status() {
      Ok(res) => Ok(res.json::<serde_json::Value>().await?),
      Err(e) => Err(e),
    }
  }

  async fn experiences(&self, entity: String) -> Result<serde_json::Value, reqwest::Error> {
    let csrf = self.csrf();
    let client = reqwest::Client::builder().gzip(true).build()?;
    println!("{}", entity);
    let result = client
      .get(format!("https://www.linkedin.com/voyager/api/voyagerIdentityGraphQL?variables=(profileUrn:urn%3Ali%3Afsd_profile%3A{},sectionType:experience)&&queryId=voyagerIdentityDashProfileComponents.1b81c8fd4bc5d26c55be2e1ce3e11a68", entity.to_string().replace("\"", "")))
      .header("accept", "application/vnd.linkedin.normalized+json+2.1")
      .header("accept-encoding", "gzip, deflate, br")
      .header("accept-language", "en-FR,en;q=0.9,fr-FR;q=0.8,fr;q=0.7,en-US;q=0.6")
      .header("cookie", format!("li_at={}; JSESSIONID=\"ajax:{}\"", self.li_at, csrf))
      .header("csrf-token", format!("ajax:{}", csrf))
      .header("pragma", "no-cache")
      .header("x-restli-protocol-version", "2.0.0")
      .send()
      .await?;
    match result.error_for_status() {
      Ok(res) => Ok(res.json::<serde_json::Value>().await?),
      Err(e) => Err(e),
    }
  }

  // https://www.linkedin.com/voyager/api/identity/dash/profiles?q=memberIdentity&memberIdentity={}
  // &decorationId=com.linkedin.voyager.dash.deco.identity.profile.FullProfileWithEntities-84
  // &decorationId=com.linkedin.voyager.dash.deco.identity.profile.FullProfileWithEntities-47
  // com.linkedin.voyager.dash.deco.identity.profile.FullProfilePosition-51
  async fn request(&self) -> Result<serde_json::Value, reqwest::Error> {
    let csrf = self.csrf();
    let client = reqwest::Client::builder().gzip(true).build()?;
    let result = client
      .get(format!("https://www.linkedin.com/voyager/api/identity/dash/profiles?q=memberIdentity&memberIdentity={}&decorationId=com.linkedin.voyager.dash.deco.identity.profile.FullProfileWithEntities-84&start=0&count=50", self.user_identity))
      .header("accept", "application/vnd.linkedin.normalized+json+2.1")
      .header("accept-encoding", "gzip, deflate, br")
      .header("cache-control", "no-cache")
      .header("content-type", "application/json; charset=utf-8")
      .header("cookie", format!("li_at={}; JSESSIONID=\"ajax:{}\"", self.li_at, csrf))
      .header("csrf-token", format!("ajax:{}", csrf))
      .header("pragma", "no-cache")
      .send()
      .await?;
    match result.error_for_status() {
      Ok(res) => Ok(res.json::<serde_json::Value>().await?),
      Err(e) => Err(e),
    }
  }
}
