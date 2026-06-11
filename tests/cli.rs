use std::path::PathBuf;

use assert_cmd::Command;
use assert_cmd::assert::Assert;
use chrono::{Local, TimeZone};
use funpou::memo::Memo;
use funpou::storage;
use tempfile::TempDir;

/// Build an `fnp` command with HOME and XDG dirs isolated under the temp dir.
fn fnp(dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("fnp").unwrap();
    cmd.env("HOME", dir.path());
    cmd.env("XDG_DATA_HOME", dir.path().join("data"));
    cmd.env("XDG_CONFIG_HOME", dir.path().join("config"));
    cmd
}

fn data_file(dir: &TempDir) -> PathBuf {
    dir.path().join("data").join("funpou").join("memos.jsonl")
}

fn stdout_of(assert: &Assert) -> String {
    String::from_utf8(assert.get_output().stdout.clone()).unwrap()
}

fn stderr_of(assert: &Assert) -> String {
    String::from_utf8(assert.get_output().stderr.clone()).unwrap()
}

#[test]
fn add_then_list_round_trips_memo() {
    let dir = TempDir::new().unwrap();

    let add = fnp(&dir).args(["add", "hello", "world"]).assert().success();
    assert!(stderr_of(&add).contains("hello world"));

    let list = fnp(&dir).arg("list").assert().success();
    assert!(stdout_of(&list).contains("hello world"));
}

#[test]
fn list_json_emits_parseable_jsonl() {
    let dir = TempDir::new().unwrap();
    fnp(&dir).args(["add", "json", "test"]).assert().success();

    let list = fnp(&dir).args(["list", "--json"]).assert().success();
    let stdout = stdout_of(&list);
    let value: serde_json::Value = serde_json::from_str(stdout.lines().next().unwrap()).unwrap();
    assert_eq!(value["body"], "json test");
}

#[test]
fn list_today_excludes_memos_from_other_days() {
    let dir = TempDir::new().unwrap();

    let old = Memo::at(
        "old memo".into(),
        Local.with_ymd_and_hms(2020, 1, 1, 9, 0, 0).unwrap(),
    );
    storage::append_memo(&data_file(&dir), &old).unwrap();

    fnp(&dir).args(["add", "fresh", "memo"]).assert().success();

    let list = fnp(&dir).args(["list", "--today"]).assert().success();
    let stdout = stdout_of(&list);
    assert!(stdout.contains("fresh memo"));
    assert!(!stdout.contains("old memo"));
}

#[test]
fn clear_with_yes_removes_all_memos() {
    let dir = TempDir::new().unwrap();
    fnp(&dir)
        .args(["add", "to", "be", "cleared"])
        .assert()
        .success();

    let clear = fnp(&dir).args(["clear", "--yes"]).assert().success();
    assert!(stderr_of(&clear).contains("Cleared 1 memo(s)."));
    assert!(!data_file(&dir).exists());
}

#[test]
fn add_syncs_to_obsidian_vault_when_configured() {
    let dir = TempDir::new().unwrap();
    let vault = dir.path().join("vault");

    let config_path = dir.path().join("config").join("funpou").join("config.toml");
    std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
    std::fs::write(
        &config_path,
        format!("[obsidian]\nvault_path = \"{}\"\n", vault.display()),
    )
    .unwrap();

    fnp(&dir)
        .args(["add", "obsidian", "e2e"])
        .assert()
        .success();

    let note_path = vault
        .join("daily")
        .join(format!("{}.md", Local::now().format("%Y-%m-%d")));
    let content = std::fs::read_to_string(&note_path).unwrap();
    assert!(content.contains("## Memos"));
    assert!(content.contains("obsidian e2e"));
}

#[test]
fn config_prints_resolved_toml_with_defaults() {
    let dir = TempDir::new().unwrap();
    let config = fnp(&dir).arg("config").assert().success();
    assert!(stdout_of(&config).contains("timestamp_format = \"%Y-%m-%d %H:%M\""));
}
