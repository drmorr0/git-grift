mod completions;
mod init;

use clap::{
    crate_version,
    CommandFactory,
    Parser,
    Subcommand,
};
use grift_core::prelude::*;

#[derive(Parser)]
#[command(about = "handy tools for working with git", version, propagate_version = true)]

struct GriftRoot {
    #[command(subcommand)]
    subcommand: GriftSubcommand,
}

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
enum GriftSubcommand {
    #[command(about = "generate shell completions for skctl")]
    Completions(completions::Args),

    #[command(about = "initialize git grift in this repo")]
    Init,

    #[command(about = "git-grift version")]
    Version,
}

fn main() -> Empty {
    let args = GriftRoot::parse();

    match &args.subcommand {
        GriftSubcommand::Completions(args) => completions::cmd(args, GriftRoot::command()),
        GriftSubcommand::Init => init::cmd(),
        GriftSubcommand::Version => {
            println!("git-grift {}", crate_version!());
            Ok(())
        },
    }
}
