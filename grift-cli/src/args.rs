use git2::Repository;
use grift_core::git_config::{
    expand_alias,
    fetch_default_branch_for,
};

const DEFAULT_BRANCH_KEY: &str = "@default";

pub(crate) async fn preprocess(mut args: Vec<String>, repo: &Repository) -> eyre::Result<Vec<String>> {
    args = expand_alias(args, repo)?;

    for arg in args.iter_mut() {
        #[allow(clippy::single_match)]
        match arg.as_ref() {
            DEFAULT_BRANCH_KEY => *arg = fetch_default_branch_for(repo, "origin")?,
            _ => (),
        }
    }

    Ok(args)
}
