mod args;
mod completions;
mod init;

use std::io::Write;
use std::process::{
    exit,
    Command,
};

use clap::{
    crate_version,
    CommandFactory,
    Parser,
    Subcommand,
};
use git2::Repository;
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

    #[command(external_subcommand)]
    GitCommand(Vec<String>),
}

#[tokio::main]
async fn main() -> Empty {
    let args = GriftRoot::parse();
    let repo = Repository::discover(".")?;

    match args.subcommand {
        GriftSubcommand::Completions(args) => completions::cmd(args, GriftRoot::command()),
        GriftSubcommand::Init => init::cmd(),
        GriftSubcommand::Version => {
            println!("git-grift {}", crate_version!());
            Ok(())
        },

        GriftSubcommand::GitCommand(mut args) => {
            args = args::preprocess(args, &repo).await?;

            // TODO we should be able to detect the color setting from the gitconfig and make it do
            // the right thing here, but meh, that sounds like work.
            let output = Command::new("git").arg("-c").arg("color.ui=always").args(&args).output()?;
            println!("Executing `git {}`", args.join(" "));
            std::io::stdout().write_all(&output.stdout)?;
            std::io::stderr().write_all(&output.stderr)?;
            exit(output.status.code().expect("subprocess terminated by signal"));
        },
    }
}
