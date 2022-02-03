use super::lazy_static_git_info::GIT_INFO;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn get_git_commit_string() -> String {
    format!(
        "{} branch'{}' {} {}",
        GIT_INFO.repo_link, GIT_INFO.branch, GIT_INFO.commit_id, GIT_INFO.commit_message,
    )
}