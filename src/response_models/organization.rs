/// `Organisation` contains all info on the organisation related to the event.
#[derive(Debug, RustcDecodable)]
pub struct Organisation {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}
