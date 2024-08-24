use eyre::format_err;
use git2::Repository;
use regex::Regex;

const REMOTE_ORIGIN: &str = "origin";

fn parse_github_repo_owner_name(remote: &str) -> eyre::Result<(String, String)> {
    let re = Regex::new(r"(git@|https://)github.com[:/](?<owner>[\w\.-]+)/(?<repo_name>[\w\.-]+)")?;
    let Some(caps) = re.captures(remote) else {
        return Err(format_err!("{remote} does not appear to be a GitHub repo"));
    };
    Ok((caps["owner"].into(), caps["repo_name"].into()))
}

pub async fn lookup_default_branch(repo: &Repository) -> eyre::Result<String> {
    let origin = repo.find_remote(REMOTE_ORIGIN)?;
    let remote_url = origin.url().expect("invalid remote URL");
    let (owner, repo_name) = parse_github_repo_owner_name(remote_url)?;

    Ok(octocrab::instance()
        .repos(owner, repo_name)
        .get()
        .await?
        .default_branch
        .expect("no default branch found for {remote_url}"))
}
