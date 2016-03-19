use error::*;
use response::Response;

use response_models::actor::Actor;
use response_models::repository::Repository;
use response_models::organization::Organisation;

use rustc_serialize::Decoder;
use rustc_serialize::Decodable;

use std::fmt;

/// Documentation References:
/// https://developer.github.com/v3/activity/

/// All Activity::Events have the same response format.
/// The following structs represent this info found as a json response.

/// `EventResponse` represents the response for almost
/// all event requests found in `activity::events` with issues as an exception.
#[derive(Debug, RustcDecodable)]
pub struct EventResponse {
    pub public: bool,
    pub repo: Repository,
    pub actor: Actor,
    pub org: Option<Organisation>,
    // todo: replace with proper time
    pub created_at: String,
    pub id: String,
}

/// `IssueEventType` is an enumuration of
/// all the different types of Issue Events.
#[derive(Debug)]
pub enum IssueEventType {
    /// The issue was closed by the `Actor`.
    Closed,
    /// The issue was reopened by the `Actor`.
    Reopened,
    /// The `Actor` subscribed to receive notifications for an issue.
    Subscribed,
    /// The issue was merged by the `Actor`.
    Merged,
    /// The issue was referenced from a commit message.
    Referenced,
    /// The `Actor` was @mentioned in an issue body.
    Mentioned,
    /// The issue was assigned to the `Actor`.
    Assigned,
    /// The issue was unassigned to the `Actor`.
    Unassigned,
    /// A label was added to the issue.
    Labeled,
    /// A label was removed from the issue.
    Unlabeled,
    /// The issue was added to a milestone.
    Milestoned,
    /// The issue was removed from a milestone.
    Demilestoned,
    /// The issue title was changed.
    Renamed,
    /// The issue was locked by the `Actor`.
    Locked,
    /// The issue was unlocked by the `Actor`.
    Unlocked,
    /// The pull request’s branch was deleted.
    HeadRefDeleted,
    /// The pull request’s branch was restored.
    HeadRefRestored,
    /// `Unknown(String)` is used as a last resort when an event is unknown.
    /// This should never happen, please report/resolve the issue when it does happen.
    Unknown(String),
}

/// Allowing `IssueEventType` to be printed via `{}`.
impl fmt::Display for IssueEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg: &str = match *self {
            IssueEventType::Closed => "closed",
            IssueEventType::Reopened => "reopened",
            IssueEventType::Subscribed => "subscribed",
            IssueEventType::Merged => "merged",
            IssueEventType::Referenced => "referenced",
            IssueEventType::Mentioned => "mentioned",
            IssueEventType::Assigned => "assigned",
            IssueEventType::Unassigned => "unassigned",
            IssueEventType::Labeled => "labeled",
            IssueEventType::Unlabeled => "unlabeled",
            IssueEventType::Milestoned => "milestoned",
            IssueEventType::Demilestoned => "demilestoned",
            IssueEventType::Renamed => "renamed",
            IssueEventType::Locked => "locked",
            IssueEventType::Unlocked => "unlocked",
            IssueEventType::HeadRefDeleted => "head reference deleted",
            IssueEventType::HeadRefRestored => "head reference restored",
            IssueEventType::Unknown(ref s) => &s,
        };

        write!(f, "{}", msg)
    }
}

/// Allowing `IssueEventType` to be decoded from json values.
/// Linked to the `event` key to the `IssueEventType` enumeration.
impl Decodable for IssueEventType {
    fn decode<D: Decoder>(d: &mut D) -> Result<IssueEventType, D::Error> {
        match d.read_str() {
            Ok(code) => Ok(match &*code {
                "closed" => IssueEventType::Closed,
                "reopened" => IssueEventType::Reopened,
                "subscribed" => IssueEventType::Subscribed,
                "merged" => IssueEventType::Merged,
                "referenced" => IssueEventType::Referenced,
                "mentioned" => IssueEventType::Mentioned,
                "assigned" => IssueEventType::Assigned,
                "unassigned" => IssueEventType::Unassigned,
                "labeled" => IssueEventType::Labeled,
                "unlabeled" => IssueEventType::Unlabeled,
                "milestoned" => IssueEventType::Milestoned,
                "demilestoned" => IssueEventType::Demilestoned,
                "renamed" => IssueEventType::Renamed,
                "locked" => IssueEventType::Locked,
                "unlocked" => IssueEventType::Unlocked,
                "head_ref_deleted" => IssueEventType::HeadRefDeleted,
                "head_ref_restored" => IssueEventType::HeadRefRestored,
                unknown => IssueEventType::Unknown(unknown.to_string()),
            }),
            Err(err) => Err(err),
        }
    }
}

/// `EventResponse` represents the response for
/// all issue event requests found in `activity::events`.
#[derive(Debug, RustcDecodable)]
pub struct IssueEventResponse {
    pub public: bool,
    pub repo: Repository,
    pub actor: Actor,
    pub org: Option<Organisation>,
    pub event: IssueEventType,
    pub created_at: String,
    pub commit_id: String,
    pub id: String,
}

/// `EventReturnType` is the return type for most event-requests.
pub type EventReturnType = Result<(Vec<EventResponse>, Response), ClientError>;
/// `EventReturnType` is the return type for issue-event-requests.
pub type IssueEventReturnType = Result<(Vec<IssueEventResponse>, Response), ClientError>;

pub mod events;
