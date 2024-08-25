mod args;
mod completions;
mod init;

use std::io::{
    stderr,
    stdout,
    Write,
};
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
use colored::*;
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

async fn execute() -> Empty {
    let args = GriftRoot::parse();
    let repo = Repository::discover(".")?;

    match args.subcommand {
        GriftSubcommand::Completions(args) => completions::cmd(args, GriftRoot::command()),
        GriftSubcommand::Init => init::cmd(&repo).await,
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
            stdout().write_all(&output.stdout)?;
            stderr().write_all(&output.stderr)?;
            exit(output.status.code().expect("subprocess terminated by signal"));
        },
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = execute().await {
        println!("{}\n\n  {e}\n\n{}", "The following error occurred:".red(), "Have you run `grift init`?".red());
    }
}
