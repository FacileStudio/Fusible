//! The Rust toolchain each firmware pins in its `rust-toolchain.toml`.
//!
//! rustup installs an official channel and its targets on the first build. A custom
//! toolchain such as `esp` comes from its own installer, and `RUSTUP_TOOLCHAIN` overrides
//! the file altogether, so both are worth a line before `fusible build` fails on them.

use std::path::Path;
use std::process::Command;

use crate::config::FusibleConfig;

/// The `channel` of the `rust-toolchain.toml` in `dir`, if any.
fn channel(dir: &Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join("rust-toolchain.toml")).ok()?;
    text.lines().find_map(|line| {
        let (key, value) = line.split_once('=')?;
        (key.trim() == "channel").then(|| value.trim().trim_matches('"').to_string())
    })
}

/// True when `rustup toolchain list` names `channel`; `None` without rustup.
fn installed(channel: &str) -> Option<bool> {
    let out = Command::new("rustup")
        .args(["toolchain", "list"])
        .env_remove("RUSTUP_TOOLCHAIN")
        .output()
        .ok()
        .filter(|out| out.status.success())?;
    let list = String::from_utf8_lossy(&out.stdout);
    let prefix = format!("{channel}-");
    Some(list.lines().any(|line| {
        let name = line.split_whitespace().next().unwrap_or_default();
        name == channel || name.starts_with(&prefix)
    }))
}

/// The status line of a firmware pinned to `channel`.
fn status(name: &str, channel: &str) -> String {
    match (installed(channel), channel) {
        (Some(true), _) => format!("  ✅ firmware {name}: toolchain {channel}"),
        (_, "esp") => format!("  ⚠️  firmware {name}: toolchain esp missing, run `espup install`"),
        (Some(false), _) => {
            format!("  ⚪ firmware {name}: rustup installs {channel} on first build")
        }
        (None, _) => {
            format!("  ⚠️  firmware {name}: needs {channel}; install rustup (https://rustup.rs)")
        }
    }
}

/// Prints the toolchain of every firmware that pins one.
pub fn print(cfg: &FusibleConfig) {
    let pinned: Vec<(&String, String)> = cfg
        .firmware
        .iter()
        .filter_map(|(name, fw)| Some((name, channel(fw.path.as_deref()?)?)))
        .collect();
    if pinned.is_empty() {
        return;
    }
    println!("\nFirmware toolchains:");
    for (name, channel) in &pinned {
        println!("{}", status(name, channel));
    }
    if let Ok(forced) = std::env::var("RUSTUP_TOOLCHAIN") {
        println!("  ⚠️  RUSTUP_TOOLCHAIN={forced} overrides every rust-toolchain.toml; unset it to build firmware");
    }
}

#[cfg(test)]
mod tests {
    use super::channel;

    #[test]
    fn the_channel_comes_from_rust_toolchain_toml() {
        let dir = std::env::temp_dir().join(format!("fusible-channel-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("rust-toolchain.toml"),
            "[toolchain]\nchannel = \"esp\"\n",
        )
        .unwrap();

        assert_eq!(channel(&dir).as_deref(), Some("esp"));
        assert_eq!(channel(&dir.join("missing")), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
