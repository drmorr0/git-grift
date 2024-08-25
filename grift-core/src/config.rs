use git2::Repository;

use crate::prelude::*;

const DEFAULT_BRANCH_KEY: &str = "griftDefaultBranch";

pub fn expand_alias(mut args: Vec<String>, repo: &Repository) -> eyre::Result<Vec<String>> {
    let maybe_alias = &args[0];
    if let Ok(alias_value) = repo.config()?.snapshot()?.get_str(&format!("alias.{maybe_alias}")) {
        let mut expanded_alias: Vec<String> = alias_value.split(" ").map(|s| s.into()).collect();
        expanded_alias.extend_from_slice(&args[1..]);
        args = expanded_alias;
    }
    Ok(args)
}

pub fn store_default_branch_for(repo: &Repository, remote: &str, branch_name: &str) -> Empty {
    repo.config()?.set_str(&default_branch_path(remote), branch_name)?;
    Ok(())
}

pub fn fetch_default_branch_for(repo: &Repository, remote: &str) -> eyre::Result<String> {
    Ok(repo.config()?.snapshot()?.get_str(&default_branch_path(remote))?.into())
}

fn default_branch_path(remote: &str) -> String {
    format!("remote.{remote}.{DEFAULT_BRANCH_KEY}")
}
