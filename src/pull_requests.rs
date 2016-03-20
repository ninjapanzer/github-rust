use error::*;
use Client;
use response::Response;

use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use response_models::actor::Actor;
use response_models::repository::Repository;
use response_models::organisation::Organisation;

pub fn list_pulls(client: &Client, user: &str, repo: &str) -> PullsReturnType {
    ::http::get(
          &client.user_agent,
          &format!("{}repos/{}/{}/pulls", client.base_url, user, repo),
          None)
}

#[derive(Debug, RustcDecodable)]
pub struct PullsResponse {
    pub id: u64,
}

pub type PullsReturnType = Result<(Vec<PullsResponse>, Response), ClientError>;
