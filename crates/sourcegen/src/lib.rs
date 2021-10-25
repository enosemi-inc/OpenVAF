/*
 *  ******************************************************************************************
 *  Copyright (c) 2021 Pascal Kuthe. This file is part of the frontend project.
 *  It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/DSPOM/OpenVAF/blob/master/LICENSE.
 *  No part of frontend, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 *  *****************************************************************************************
 */

#[cfg(test)]
mod ast_src;
#[cfg(test)]
mod sourcegen_ast;
#[cfg(test)]
mod integration_tests;

use std::{
    fmt, fs, mem,
    path::{Path, PathBuf},
};
use xshell::{cmd, pushenv};

pub fn list_rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = list_files(dir);
    res.retain(|it| {
        it.file_name().unwrap_or_default().to_str().unwrap_or_default().ends_with(".rs")
    });
    res
}

pub fn list_files(dir: &Path) -> Vec<PathBuf> {
    let mut res = Vec::new();
    let mut work = vec![dir.to_path_buf()];
    while let Some(dir) = work.pop() {
        for entry in dir.read_dir().unwrap() {
            let entry = entry.unwrap();
            let file_type = entry.file_type().unwrap();
            let path = entry.path();
            let is_hidden =
                path.file_name().unwrap_or_default().to_str().unwrap_or_default().starts_with('.');
            if !is_hidden {
                if file_type.is_dir() {
                    work.push(path)
                } else if file_type.is_file() {
                    res.push(path)
                }
            }
        }
    }
    res
}

pub struct CommentBlock {
    pub id: String,
    pub line: usize,
    pub contents: Vec<String>,
}

impl CommentBlock {
    pub fn extract(tag: &str, text: &str) -> Vec<CommentBlock> {
        assert!(tag.starts_with(char::is_uppercase));

        let tag = format!("{}:", tag);
        let mut res = Vec::new();
        for (line, mut block) in do_extract_comment_blocks(text, true) {
            let first = block.remove(0);
            if let Some(id) = first.strip_prefix(&tag) {
                let id = id.trim().to_string();
                let block = CommentBlock { id, line, contents: block };
                res.push(block);
            }
        }
        res
    }

    pub fn extract_untagged(text: &str) -> Vec<CommentBlock> {
        let mut res = Vec::new();
        for (line, block) in do_extract_comment_blocks(text, false) {
            let id = String::new();
            let block = CommentBlock { id, line, contents: block };
            res.push(block);
        }
        res
    }
}

fn do_extract_comment_blocks(
    text: &str,
    allow_blocks_with_empty_lines: bool,
) -> Vec<(usize, Vec<String>)> {
    let mut res = Vec::new();

    let prefix = "// ";
    let lines = text.lines().map(str::trim_start);

    let mut block = (0, vec![]);
    for (line_num, line) in lines.enumerate() {
        if line == "//" && allow_blocks_with_empty_lines {
            block.1.push(String::new());
            continue;
        }

        let is_comment = line.starts_with(prefix);
        if is_comment {
            block.1.push(line[prefix.len()..].to_string());
        } else {
            if !block.1.is_empty() {
                res.push(mem::take(&mut block));
            }
            block.0 = line_num + 2;
        }
    }
    if !block.1.is_empty() {
        res.push(block)
    }
    res
}

#[derive(Debug)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let path = self.file.strip_prefix(&project_root()).unwrap().display().to_string();
        let path = path.replace('\\', "/");
        let name = self.file.file_name().unwrap();
        write!(
            f,
            "https://github.com/rust-analyzer/rust-analyzer/blob/master/{}#L{}[{}]",
            path,
            self.line,
            name.to_str().unwrap()
        )
    }
}

fn ensure_rustfmt() {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    let version = cmd!("rustfmt --version").read().unwrap_or_default();
    if !version.contains("stable") {
        panic!(
            "Failed to run rustfmt from toolchain 'stable'. \
                 Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
}

pub fn reformat(text: String) -> String {
    ensure_rustfmt();
    let rustfmt_toml = project_root().join("rustfmt.toml");
    let mut stdout = cmd!("rustfmt --config-path {rustfmt_toml} --config fn_single_line=true")
        .stdin(text)
        .read()
        .unwrap();
    if !stdout.ends_with('\n') {
        stdout.push('\n');
    }
    stdout
}

pub fn add_preamble(generator: &'static str, mut text: String) -> String {
    let preamble = format!("//! Generated by `{}`, do not edit by hand.\n\n", generator);
    text.insert_str(0, &preamble);
    text
}

/// Checks that the `file` has the specified `contents`. If that is not the
/// case, updates the file and then fails the test.
pub fn ensure_file_contents(file: &Path, contents: &str) {
    if let Ok(old_contents) = fs::read_to_string(file) {
        if normalize_newlines(&old_contents) == normalize_newlines(contents) {
            // File is already up to date.
            return;
        }
    }

    let display_path = file.strip_prefix(&project_root()).unwrap_or(file);
    eprintln!(
        "\n\x1b[31;1merror\x1b[0m: {} was not up-to-date, updating\n",
        display_path.display()
    );
    if std::env::var("CI").is_ok() {
        eprintln!("    NOTE: run `cargo test` locally and commit the updated files\n");
    }
    if let Some(parent) = file.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(file, contents).unwrap();
    panic!("some file was not up to date and has been updated, simply re-run the tests")
}

fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

pub fn project_root() -> PathBuf {
    let dir = env!("CARGO_MANIFEST_DIR");
    let mut res = PathBuf::from(dir);
    while !res.join("README.md").exists() {
        res = res.parent().expect("reached fs root without finding project root").to_owned()
    }
    res
}

pub fn to_upper_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_uppercase());
    }
    buf
}

pub fn to_lower_snake_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c.is_ascii_uppercase() && prev {
            buf.push('_')
        }
        prev = true;

        buf.push(c.to_ascii_lowercase());
    }
    buf
}

pub fn to_pascal_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev_is_underscore = true;
    for c in s.chars() {
        if c == '_' {
            prev_is_underscore = true;
        } else if prev_is_underscore {
            buf.push(c.to_ascii_uppercase());
            prev_is_underscore = false;
        } else {
            buf.push(c.to_ascii_lowercase());
        }
    }
    buf
}

pub fn pluralize(s: &str) -> String {
    format!("{}s", s)
}
/// Returns `false` if slow tests should not run, otherwise returns `true` and
/// also creates a file at `./target/.slow_tests_cookie` which serves as a flag
/// that slow tests did run.
pub fn skip_slow_tests() -> bool {
    let should_skip = std::env::var("CI").is_err() && std::env::var("RUN_SLOW_TESTS").is_err();
    if should_skip {
        eprintln!("ignoring slow test");
    } else {
        let path = project_root().join("./target/.slow_tests_cookie");
        fs::write(&path, ".").unwrap();
    }
    should_skip
}
