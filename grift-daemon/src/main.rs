mod db;
mod errors;
mod grifter;

use clap::Parser;
use grift_core::prelude::*;
use grift_core::rpc::GriftServer;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::*;
use tracing_subscriber::fmt::format::FmtSpan;

use crate::db::GriftDB;
use crate::grifter::Grifter;

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

#[instrument(ret, err)]
async fn run(args: Args) -> Empty {
    let db = GriftDB::open()?;
    let server = GriftServer::build(Grifter::new(db))?;
    let cancel = CancellationToken::new();

    tokio::select! {
        res = server.run(cancel.clone()) => res?,
        _ = signal::ctrl_c() => cancel.cancel(),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Empty {
    color_eyre::install()?;
    let args = Args::parse();
    setup_logging(&args.verbosity);
    run(args).await
}
