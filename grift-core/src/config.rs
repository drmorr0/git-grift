use git2::Repository;

pub fn expand_alias(mut args: Vec<String>, repo: &Repository) -> eyre::Result<Vec<String>> {
    let config = repo.config()?.snapshot()?;
    let maybe_alias = &args[0];
    if let Ok(alias_value) = config.get_str(&format!("alias.{maybe_alias}")) {
        let mut expanded_alias: Vec<String> = alias_value.split(" ").map(|s| s.into()).collect();
        expanded_alias.extend_from_slice(&args[1..]);
        args = expanded_alias;
    }
    Ok(args)
}
