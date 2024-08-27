use clap::Parser;
use grift_core::prelude::*;
use grift_core::rpc::GriftServer;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tonic::{
    Request,
    Response,
    Status,
};
use tracing::*;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    #[arg(short, long, default_value = "info")]
    verbosity: String,
}

fn setup_logging(env_filter: &str) {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::NEW)
        .with_target(false)
        .with_env_filter(env_filter)
        .compact()
        .init();
}

#[derive(Clone)]
struct Grifter {}

#[tonic::async_trait]
impl GriftService for Grifter {
    async fn init_repo(&self, req: Request<InitRepoRequest>) -> Result<Response<InitRepoResponse>, Status> {
        let resp = InitRepoResponse {
            success: true,
            msg: format!("repo at {} initialized", req.into_inner().repo_path),
        };
        Ok(Response::new(resp))
    }
}

#[instrument(ret, err)]
async fn run(args: Args) -> Empty {
    let server = GriftServer::build(Grifter {})?;
    let cancel = CancellationToken::new();

    tokio::select! {
        res = server.run(cancel.clone()) => res?,
        _ = signal::ctrl_c() => cancel.cancel(),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Empty {
    let args = Args::parse();
    setup_logging(&args.verbosity);
    run(args).await
}
