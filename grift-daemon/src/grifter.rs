use std::fmt;

use grift_core::prelude::*;
use tonic::{
    Request,
    Response,
    Status,
};
use tracing::*;

use crate::db::GriftDB;

#[derive(Clone)]
pub struct Grifter {
    db: GriftDB,
}

impl Grifter {
    pub fn new(db: GriftDB) -> Grifter {
        Grifter { db }
    }
}

impl fmt::Debug for Grifter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Grifter({})", self.db.path())
    }
}

#[tonic::async_trait]
impl GriftService for Grifter {
    #[instrument(ret, err)]
    #[allow(clippy::blocks_in_conditions)] // TODO - remove this once we're on Rust 1.81
    async fn init_repo(&self, req: Request<InitRepoRequest>) -> Result<Response<InitRepoResponse>, Status> {
        let repo_path = req.into_inner().repo_path;
        let mut resp = InitRepoResponse {
            success: true,
            msg: format!("repo at {repo_path} initialized"),
        };

        self.db.track_repo(&repo_path).unwrap_or_else(|e| {
            resp.success = false;
            resp.msg = format!("could not initialize repo: {e}");
        });

        Ok(Response::new(resp))
    }
}
