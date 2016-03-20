use response_models::actor::Actor;
/// `Repository` contains all info regarding a git repository.
#[derive(Debug, RustcDecodable)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub url: String,
}
