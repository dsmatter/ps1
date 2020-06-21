use git2::Status as S;
use git2::{Branch, Repository, RepositoryOpenFlags};
use lazy_static::lazy_static;
use std::ffi::OsString;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub ref_name: RefName,
    pub files: FilesStatus,
    pub upstream: Option<UpstreamStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilesStatus {
    pub uncommitted_files: u32,
    pub untracked_files: u32,
}

impl Default for FilesStatus {
    fn default() -> Self {
        FilesStatus {
            uncommitted_files: 0,
            untracked_files: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpstreamStatus {
    pub commits_ahead: u32,
    pub commits_behind: u32,
}

impl Default for UpstreamStatus {
    fn default() -> Self {
        UpstreamStatus {
            commits_ahead: 0,
            commits_behind: 0,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum RefName {
    Branch(String),
    Hash(String),
}

pub fn status(cwd: &Path) -> Option<Status> {
    let repo = Repository::open_ext(
        &cwd,
        RepositoryOpenFlags::empty(),
        std::iter::empty::<OsString>(),
    )
    .ok()?;
    let ref_name = ref_name(&repo)?;
    let files = files_status(&repo)?;
    let upstream = upstream_status(&repo);

    Some(Status {
        ref_name,
        files,
        upstream,
    })
}

fn ref_name(repo: &Repository) -> Option<RefName> {
    let reference = repo.head().ok()?;
    let branch_name = if reference.is_branch() {
        reference
            .shorthand()
            .map(|r| RefName::Branch(r.to_string()))
    } else {
        None
    };
    branch_name.or_else(|| {
        reference
            .peel_to_commit()
            .ok()
            .map(|c| RefName::Hash(c.id().to_string()))
    })
}

fn files_status(repo: &Repository) -> Option<FilesStatus> {
    let mut files_status = FilesStatus::default();
    for status in repo.statuses(None).ok()?.iter().map(|s| s.status()) {
        if status == S::WT_NEW {
            files_status.untracked_files += 1;
        } else if status.intersects(*UNCOMMITTED_FLAGS) {
            files_status.uncommitted_files += 1;
        }
    }
    Some(files_status)
}

fn upstream_status(repo: &Repository) -> Option<UpstreamStatus> {
    let head = repo.head().ok()?;
    let local_oid = head.target()?;
    let upstream = Branch::wrap(head).upstream().ok()?;
    let remote_oid = upstream.get().target()?;
    let (ahead, behind) = repo.graph_ahead_behind(local_oid, remote_oid).ok()?;
    Some(UpstreamStatus {
        commits_ahead: ahead as u32,
        commits_behind: behind as u32,
    })
}

lazy_static! {
    static ref UNCOMMITTED_FLAGS: S = S::WT_MODIFIED
        | S::WT_DELETED
        | S::WT_RENAMED
        | S::WT_TYPECHANGE
        | S::INDEX_NEW
        | S::INDEX_MODIFIED
        | S::INDEX_DELETED
        | S::INDEX_RENAMED
        | S::INDEX_TYPECHANGE;
}
