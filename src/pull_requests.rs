use Client;

pub fn list_pulls(client: &Client, user: &str, repo: &str) -> PullsReturnType {
    ::http::get(
          &client.user_agent,
          &format!("/repos/{}/{}/pulls", user, repo),
          None)

}
