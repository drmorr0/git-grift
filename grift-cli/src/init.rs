use std::io::{
    stdout,
    Write,
};

use colored::*;
use git2::Repository;
use grift_core::config::store_default_branch_for;
use grift_core::github::lookup_default_branch;
use grift_core::prelude::*;

pub async fn cmd(repo: &Repository) -> Empty {
    let git_path = repo.path().to_string_lossy();
    println!("{}", format!("Initializing grift in {git_path}").blue());

    print!("{}", "  🔎 Detecting default branch for 'origin'... ".green());
    stdout().flush()?;

    let branch = lookup_default_branch(repo, "origin").await?;
    store_default_branch_for(repo, "origin", &branch)?;
    println!("{}", format!("found {branch}").green());

    println!("{}", format!("Writing config to {git_path}config").blue());

    Ok(())
}
