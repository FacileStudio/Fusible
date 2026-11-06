//! Calls to the `nuage` CLI.

use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

use super::ShareError;

fn nuage(args: &[&str]) -> Result<std::process::Output, ShareError> {
    Command::new("nuage")
        .args(args)
        .output()
        .map_err(|e| ShareError::Nuage(format!("cannot run nuage: {e}")))
}

fn expand(path: &str) -> PathBuf {
    match (path.strip_prefix("~/"), std::env::var_os("HOME")) {
        (Some(rest), Some(home)) => PathBuf::from(home).join(rest),
        _ => PathBuf::from(path),
    }
}

/// The local sync folder of the space called `name`.
pub fn sync_dir(name: &str) -> Result<PathBuf, ShareError> {
    let output = nuage(&["spaces", "list", "--json"])?;
    if !output.status.success() {
        return Err(ShareError::Nuage(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }
    let json: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| ShareError::Nuage(format!("unreadable nuage spaces output: {e}")))?;
    let space = json["spaces"]
        .as_array()
        .and_then(|spaces| spaces.iter().find(|s| s["name"] == name))
        .ok_or_else(|| ShareError::Nuage(format!("no Nuage space named \"{name}\"")))?;
    let dir = space["sync_dir"]
        .as_str()
        .ok_or_else(|| ShareError::Unsynced(name.to_string()))?;
    Ok(expand(dir))
}

/// Runs a one-shot sync of every mapped space.
pub fn sync() -> Result<(), ShareError> {
    let output = nuage(&["sync", "--json"])?;
    match output.status.success() {
        true => Ok(()),
        false => Err(ShareError::Nuage(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        )),
    }
}

fn url_of(stdout: &[u8]) -> Option<String> {
    let json: Value = serde_json::from_slice(stdout).ok()?;
    json.get("url")
        .or_else(|| json.pointer("/share/url"))?
        .as_str()
        .map(str::to_string)
}

/// Creates a share link; a failure only prints a hint because the files already arrived.
pub fn link(remote: &str, expires: &str) {
    let output = nuage(&["share", remote, "--json", "--expires", expires]);
    match output {
        Ok(out) if out.status.success() => match url_of(&out.stdout) {
            Some(url) => println!("Share link: {url}"),
            None => hint(""),
        },
        Ok(out) => hint(&String::from_utf8_lossy(&out.stderr)),
        Err(err) => hint(&err.to_string()),
    }
}

fn hint(stderr: &str) {
    eprintln!("{}", stderr.trim());
    eprintln!("files are synced; create the link from the Nuage app");
}
