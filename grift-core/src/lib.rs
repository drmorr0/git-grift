pub mod git_config;
pub mod github;
pub mod rpc;

pub mod prelude {
    use std::sync::LazyLock;

    pub type Empty = eyre::Result<()>;
    pub use super::rpc::grift_service_server::GriftService;
    pub use super::rpc::{
        InitRepoRequest,
        InitRepoResponse,
    };

    pub static XDG_DIRS: LazyLock<xdg::BaseDirectories> =
        LazyLock::new(|| xdg::BaseDirectories::with_prefix("grift").unwrap());
}
