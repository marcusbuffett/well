/* spell-checker:words revwalk */

pub fn list_commits(_arguments: &str) -> Result<String, String> {
    let repo = git2::Repository::discover(".").map_err(|err| err.to_string())?;


    let mut result = String::new();
    for rev in repo.revwalk().map_err(|err| err.to_string())?.take(11) {
        let rev = rev.map_err(|err| err.to_string())?;
        let commit = repo.find_commit(rev).map_err(|err| err.to_string())?;
        let date = commit.time().seconds();
        let hash = commit.id().to_string();
        let summary = commit.summary().unwrap_or_default();
        let author = commit.author();
        let author = author.name().unwrap_or_default();

        result.push_str(&format!("[{date}] [{hash}] | {summary} {{{author}}}\n"));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "run manually to see output"]
    fn commits_listing_format() {
        let result = list_commits(".").unwrap();
        assert_eq!(result, "Initial commit\n");
    }
}
