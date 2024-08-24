use git2::Repository;
use grift_core::config::expand_alias;
use grift_core::github::lookup_default_branch;

const DEFAULT_BRANCH_KEY: &str = "@default";

pub(crate) async fn preprocess(mut args: Vec<String>, repo: &Repository) -> eyre::Result<Vec<String>> {
    args = expand_alias(args, repo)?;
    for arg in args.iter_mut() {
        #[allow(clippy::single_match)]
        match arg.as_ref() {
            DEFAULT_BRANCH_KEY => *arg = lookup_default_branch(repo).await?,
            _ => (),
        }
    }

    Ok(args)
}
