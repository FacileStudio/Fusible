//! Creating one Opus task through the CLI.

use std::process::Command;

use serde_json::Value;

use super::dedup::Store;
use crate::cli::OpusArgs;

/// Strips inline-syntax markers from a title and caps it at 120 characters.
pub fn clean_title(text: &str) -> String {
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|w| w.trim_start_matches(['+', '*', '@', '!']))
        .collect();
    words.join(" ").chars().take(120).collect()
}

fn task_id(output: &[u8]) -> Option<String> {
    let json: Value = serde_json::from_slice(output).ok()?;
    let id = json.get("id").or_else(|| json.pointer("/task/id"))?;
    Some(id.as_str().map_or_else(|| id.to_string(), str::to_string))
}

fn add_task(args: &[String]) -> Option<String> {
    let output = match Command::new("opus")
        .arg("task")
        .arg("add")
        .args(args)
        .output()
    {
        Ok(output) if output.status.success() => output,
        Ok(output) => return warn(&String::from_utf8_lossy(&output.stderr)),
        Err(err) => return warn(&err.to_string()),
    };
    task_id(&output.stdout).or_else(|| warn("could not read the task id from opus output"))
}

fn warn(message: &str) -> Option<String> {
    eprintln!("warning: opus: {}", message.trim());
    None
}

/// Creates the task unless `key` is known, then remembers the returned id.
pub fn create(store: &mut Store, key: String, mut args: Vec<String>, opus: &OpusArgs) {
    if store.has(&key) {
        return;
    }
    if let Some(project) = &opus.opus_project {
        args.extend(["--project".into(), project.clone()]);
    }
    args.push("--json".into());
    if let Some(id) = add_task(&args) {
        store
            .insert(key, id)
            .unwrap_or_else(|err| eprintln!("warning: opus: cannot save dedup file: {err}"));
    }
}
