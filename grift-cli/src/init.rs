use std::io::{
    stdout,
    Write,
};

use colored::*;
use git2::Repository;
use grift_core::config::store_default_branch_for;
use grift_core::github::lookup_default_branch;
use grift_core::prelude::*;
use grift_core::rpc::GriftClient;
use tonic::Request;

pub async fn cmd(repo: &Repository) -> Empty {
    let repo_path = repo.path().to_string_lossy();

    let req = Request::new(InitRepoRequest { repo_path: repo_path.to_string() });
    let mut client = GriftClient::build().await?;
    let resp = client.init_repo(req).await?;
    println!("Got response from grift-daemon: {resp:?}");

    println!("{}", format!("Initializing grift in {repo_path}").blue());

    print!("{}", "  🔎 Detecting default branch for 'origin'... ".green());
    stdout().flush()?;

    let branch = lookup_default_branch(repo, "origin").await?;
    store_default_branch_for(repo, "origin", &branch)?;
    println!("{}", format!("found {branch}").green());

    println!("{}", format!("Writing config to {repo_path}config").blue());

    Ok(())
}
