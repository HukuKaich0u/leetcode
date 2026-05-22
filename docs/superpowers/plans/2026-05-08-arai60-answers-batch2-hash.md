# arai60 Answers Batch 2 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `HashMap / HashSet` バッチの回答を、新 README 形式に沿って高品質化する

**Architecture:** 各問題ファイルを、厚めの解説、本命解法、比較用の別解、必要に応じた `素朴実装 / 本命実装 / 整理した実装` のコードレベル比較、`#[cfg(test)]` による自己完結テストを持つ教材へ作り替える。検証はバッチ 1 と同様に `rustc --test --crate-name ...` を使う。

**Tech Stack:** Rust, `rustc --test`, Markdown comments, git

---

## Chunk 1: sliding hash basics

### Task 1: rewrite the string-window and stack/set fundamentals

**Files:**
- Modify: `arai60/answers/3.longest-substring-without-repeating-characters.rs`
- Modify: `arai60/answers/20.valid-parentheses.rs`
- Modify: `arai60/answers/349.intersection-of-two-arrays.rs`
- Modify: `arai60/answers/387.first-unique-character-in-a-string.rs`
- Modify: `arai60/answers/929.unique-email-addresses.rs`

- [ ] **Step 1: Write failing tests first for all five files**

Per-file focus:
- `3...rs`: 基本ケース、空文字、重複が連続するケース
- `20...rs`: 正常ネスト、順序違い、空文字
- `349...rs`: 重複入力、共通部分なし
- `387...rs`: 最初の一意文字、存在しないケース
- `929...rs`: `.` と `+` を含む正規化ケース

- [ ] **Step 2: Run `rustc --test` for each file and verify RED**

- [ ] **Step 3: Implement each file in new format**

Required comparisons:
- `3...rs`: 素朴走査 / HashSet shrink / last-seen jump
- `20...rs`: 文字列置換的な素朴案 / stack / 整理版
- `349...rs`: 二重ループ / set / sort + two pointers
- `387...rs`: 二重ループ / 2-pass count / queue or map variant
- `929...rs`: 素朴比較 / 正規化 + set / 補助関数版

- [ ] **Step 4: Re-run each file test and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/3.longest-substring-without-repeating-characters.rs arai60/answers/20.valid-parentheses.rs arai60/answers/349.intersection-of-two-arrays.rs arai60/answers/387.first-unique-character-in-a-string.rs arai60/answers/929.unique-email-addresses.rs
git commit -m "feat: upgrade basic hash answers"
```

## Chunk 2: grouping, counting, and prefix-hash problems

### Task 2: rewrite canonical-key and prefix-sum hash problems

**Files:**
- Modify: `arai60/answers/49.group-anagrams.rs`
- Modify: `arai60/answers/347.top-k-frequent-elements.rs`
- Modify: `arai60/answers/560.subarray-sum-equals-k.rs`

- [ ] **Step 1: Write failing tests first**

Per-file focus:
- `49...rs`: 複数グループ、単一文字列、順不同比較
- `347...rs`: 基本ケース、頻度同率ケース
- `560...rs`: 負数を含むケース、複数区間が同じ和になるケース

- [ ] **Step 2: Run `rustc --test` for each file and verify RED**

- [ ] **Step 3: Implement each file in new format**

Required comparisons:
- `49...rs`: sort key / frequency key / key builder helper
- `347...rs`: full sort / bucket / heap
- `560...rs`: 全区間 / prefix count hash / 整理版

- [ ] **Step 4: Re-run tests and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/49.group-anagrams.rs arai60/answers/347.top-k-frequent-elements.rs arai60/answers/560.subarray-sum-equals-k.rs
git commit -m "feat: upgrade hash grouping answers"
```

## Chunk 3: graph-like hash heavy problem

### Task 3: rewrite `127.word-ladder.rs`

**Files:**
- Modify: `arai60/answers/127.word-ladder.rs`

- [ ] **Step 1: Write failing tests first**

Focus:
- 基本ケース
- 到達不能
- 辞書に終点がないケース

- [ ] **Step 2: Run `rustc --test` and verify RED**

- [ ] **Step 3: Implement in new format**

Required comparisons:
- 素朴 graph 構築案
- 本命の 26 文字置換 BFS
- wildcard 中間ノード案 or 双方向 BFS 案

- [ ] **Step 4: Re-run test and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/127.word-ladder.rs
git commit -m "feat: upgrade word ladder answer"
```

## Chunk 4: batch-wide verification

### Task 4: verify consistency across batch 2

**Files:**
- Reference: all files in this batch

- [ ] **Step 1: Check format consistency**

Confirm:
- 必須見出し
- `struct Solution;`
- `#[cfg(test)]`
- コードレベル比較
- Rust 観点 / コード品質観点 / 落とし穴

- [ ] **Step 2: Run all batch file tests**

Run `rustc --test --crate-name ...` for:
- `3.longest-substring-without-repeating-characters.rs`
- `20.valid-parentheses.rs`
- `49.group-anagrams.rs`
- `127.word-ladder.rs`
- `347.top-k-frequent-elements.rs`
- `349.intersection-of-two-arrays.rs`
- `387.first-unique-character-in-a-string.rs`
- `560.subarray-sum-equals-k.rs`
- `929.unique-email-addresses.rs`

Expected: PASS

- [ ] **Step 3: Commit the completed batch**

```bash
git add arai60/answers/3.longest-substring-without-repeating-characters.rs arai60/answers/20.valid-parentheses.rs arai60/answers/49.group-anagrams.rs arai60/answers/127.word-ladder.rs arai60/answers/347.top-k-frequent-elements.rs arai60/answers/349.intersection-of-two-arrays.rs arai60/answers/387.first-unique-character-in-a-string.rs arai60/answers/560.subarray-sum-equals-k.rs arai60/answers/929.unique-email-addresses.rs
git commit -m "feat: upgrade hash answer batch"
```

## Unresolved questions

- `127.word-ladder.rs` は graph 的性質が強いが、HashSet / BFS の組み合わせとしてこのバッチで扱う
