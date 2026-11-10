//! A unified diff of two small texts, line by line, with three lines of context.

use std::ops::Range;

const CONTEXT: usize = 3;

#[derive(Clone, Copy, PartialEq)]
enum Op {
    Keep(usize, usize),
    Delete(usize),
    Insert(usize),
}

/// The changes from `old` to `new` as a unified diff labelled `label`; empty when they match.
pub(super) fn unified(old: &str, new: &str, label: &str) -> String {
    let (a, b): (Vec<&str>, Vec<&str>) = (old.lines().collect(), new.lines().collect());
    let ops = edit_script(&a, &b);
    let hunks = hunk_ranges(&ops);
    if hunks.is_empty() {
        return String::new();
    }
    let mut out = format!("--- {label}\n+++ {label}\n");
    for range in hunks {
        out.push_str(&render(&ops[range], &a, &b));
    }
    out
}

fn lcs_table(a: &[&str], b: &[&str]) -> Vec<Vec<u32>> {
    let mut lcs = vec![vec![0u32; b.len() + 1]; a.len() + 1];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            lcs[i][j] = match a[i] == b[j] {
                true => lcs[i + 1][j + 1] + 1,
                false => lcs[i + 1][j].max(lcs[i][j + 1]),
            };
        }
    }
    lcs
}

fn edit_script(a: &[&str], b: &[&str]) -> Vec<Op> {
    let lcs = lcs_table(a, b);
    let (mut i, mut j, mut ops) = (0, 0, Vec::new());
    while i < a.len() || j < b.len() {
        let op = match (i < a.len(), j < b.len()) {
            (true, true) if a[i] == b[j] => Op::Keep(i, j),
            (true, true) if lcs[i + 1][j] >= lcs[i][j + 1] => Op::Delete(i),
            (true, false) => Op::Delete(i),
            _ => Op::Insert(j),
        };
        (i, j) = match op {
            Op::Keep(..) => (i + 1, j + 1),
            Op::Delete(_) => (i + 1, j),
            Op::Insert(_) => (i, j + 1),
        };
        ops.push(op);
    }
    ops
}

fn hunk_ranges(ops: &[Op]) -> Vec<Range<usize>> {
    let changed: Vec<usize> = (0..ops.len())
        .filter(|&k| !matches!(ops[k], Op::Keep(..)))
        .collect();
    let mut hunks: Vec<Range<usize>> = Vec::new();
    for k in changed {
        let range = k.saturating_sub(CONTEXT)..(k + CONTEXT + 1).min(ops.len());
        match hunks.last_mut() {
            Some(last) if range.start <= last.end => last.end = range.end,
            _ => hunks.push(range),
        }
    }
    hunks
}

fn render(ops: &[Op], a: &[&str], b: &[&str]) -> String {
    let start = |old: bool| {
        ops.iter().find_map(|op| match (op, old) {
            (Op::Keep(i, _) | Op::Delete(i), true) => Some(*i),
            (Op::Keep(_, j) | Op::Insert(j), false) => Some(*j),
            _ => None,
        })
    };
    let count = |old: bool| {
        ops.iter()
            .filter(|op| !matches!((op, old), (Op::Insert(_), true) | (Op::Delete(_), false)))
            .count()
    };
    let first = |old: bool, len: usize| start(old).map_or(0, |s| s + usize::from(len > 0));
    let (old_len, new_len) = (count(true), count(false));
    let mut out = format!(
        "@@ -{},{old_len} +{},{new_len} @@\n",
        first(true, old_len),
        first(false, new_len)
    );
    for op in ops {
        let line = match *op {
            Op::Keep(i, _) => format!(" {}", a[i]),
            Op::Delete(i) => format!("-{}", a[i]),
            Op::Insert(j) => format!("+{}", b[j]),
        };
        out.push_str(&line);
        out.push('\n');
    }
    out
}
