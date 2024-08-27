pub mod config;
pub mod github;
pub mod rpc;

pub mod prelude {
    pub type Empty = eyre::Result<()>;
    pub use super::rpc::grift_service_server::GriftService;
    pub use super::rpc::{
        InitRepoRequest,
        InitRepoResponse,
    };
}
