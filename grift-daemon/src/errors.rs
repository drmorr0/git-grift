use thiserror::Error;
use tonic::Status;

#[derive(Debug, Error)]
pub enum GriftdError {
    #[error("repo {0} has already been initialized")]
    AlreadyInitialized(String),

    #[error("unknown error")]
    Unknown,
}

impl From<GriftdError> for Status {
    fn from(err: GriftdError) -> Self {
        match err {
            GriftdError::AlreadyInitialized(repo) => {
                Status::already_exists(format!("repo {repo} has already been initialized"))
            },
            _ => Status::unknown("an unknown error occurred"),
        }
    }
}
