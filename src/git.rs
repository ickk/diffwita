use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json;
use std::process::Command;
use unidiff::PatchSet;

/// Set the cleanup mode on commits to "scissors" to prevent git from removing lines that start
/// with characters like `#`, which are important for us.
pub fn configure() {
  match Command::new("git")
    .arg("config")
    .arg("--local")
    .arg("commit.cleanup")
    .arg("scissors")
    .output()
  {
    Ok(output) if output.status.success() => (),
    _ => panic!("Failed to execute git config command"),
  }
}

/// Get the current HEAD
pub fn head() -> String {
  match Command::new("git")
    .arg("show")
    .arg("--no-patch")
    .arg("--format=%H")
    .output()
  {
    Ok(output) if output.status.success() => std::str::from_utf8(&output.stdout)
      .expect("Failed to interpret output as utf8")
      .trim_end()
      .to_string(),
    _ => panic!("Failed to get the HEAD"),
  }
}

/// Get the raw unsantised commit message text of a commit
pub fn message(hash: &str) -> String {
  match Command::new("git")
    .arg("show")
    .arg("--no-patch")
    .arg("--format=%B")
    .arg(hash)
    .output()
  {
    Ok(output) if output.status.success() => {
      String::from_utf8(output.stdout).expect("Failed to interpret output as utf8")
    },
    _ => panic!("Failed to get the message from commit:{hash}"),
  }
}

/// Get a sequence of commits given a starting commit hash
pub fn log(hash: &str) -> Log {
  let output = match Command::new("git")
    .arg("log")
    .arg("--reverse")
    .arg(r#"--format={"commit":"%H","author":{"name":"%aN","email":"%aE"},"date":"%aI","subject":"%f"},"#)
    .arg(hash)
    .output()
    {
      Ok(output) if output.status.success() => output,
      _ => panic!("Failed to get list of commits from commit:{hash}"),
    };

  Log(
    serde_json::from_str(&{
      let mut s = String::from("[");
      s.push_str(std::str::from_utf8(&output.stdout).expect("Failed to interpret output as utf8"));
      s.pop();
      s.pop();
      s.push(']');
      s
    })
    .expect("Failed to parse as json"),
  )
}

/// Get the patchset for a particular commit
pub fn patchset(hash: &str) -> PatchSet {
  // we use `git show` because it handles the first commit well unlike `git diff hash~ hash`
  // however couldn't find a way to totally silence the log output.
  let output = match Command::new("git")
    .arg("show")
    .arg("-p")
    .arg("--format=%n")
    .arg(hash)
    .output()
  {
    Ok(output) if output.status.success() => output,
    _ => panic!("Failed to get the patch for commit:{hash}"),
  };

  // ignore whitespace at the start from `--format`
  let output = unsafe {
    std::str::from_utf8_unchecked(&output.stdout)
      .trim_start()
      .as_bytes()
  };

  let mut p = PatchSet::new();
  p.parse_bytes(output).expect("failed to parse patchset");
  p
}

/// Metadata about a commit. Contains the basic information from `git log`
#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMeta {
  pub commit: String,
  pub author: Author,
  pub date: DateTime<Utc>,
  pub subject: String,
}

/// The author of a commit
#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
  pub name: String,
  pub email: String,
}

pub struct Log(Vec<CommitMeta>);

#[derive(Debug)]
pub struct Section {
  pub id: usize,
  pub text: String,
  pub commit: String,
  pub patchset: PatchSet,
}

impl Log {
  pub fn sections(&self) -> impl Iterator<Item = Section> + '_ {
    self.0.iter().enumerate().map(|(id, commit_meta)| Section {
      id,
      commit: commit_meta.commit.clone(),
      text: message(&commit_meta.commit),
      patchset: patchset(&commit_meta.commit),
    })
  }
}
