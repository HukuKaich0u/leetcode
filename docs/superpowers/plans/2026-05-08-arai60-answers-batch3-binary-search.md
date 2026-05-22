# arai60 Answers Batch 3 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `binary search` バッチの回答を、新 README 形式に沿って高品質化する

**Architecture:** 各問題ファイルを、厚めの解説、本命解法、比較用の別解、必要に応じた `素朴実装 / 本命実装 / 整理した実装` のコードレベル比較、`#[cfg(test)]` による自己完結テストを持つ教材へ作り替える。検証は引き続き `rustc --test --crate-name ...` を使う。

**Tech Stack:** Rust, `rustc --test`, Markdown comments, git

---

## Chunk 1: basic binary search forms

### Task 1: rewrite exact-position and boundary-search basics

**Files:**
- Modify: `arai60/answers/35.search-insert-position.rs`
- Modify: `arai60/answers/153.find-minimum-in-rotated-sorted-array.rs`

- [ ] **Step 1: Write failing tests first**

Per-file focus:
- `35...rs`: 存在する値、先頭より小さい値、末尾より大きい値
- `153...rs`: 回転あり、回転なし、長さ 1

- [ ] **Step 2: Run `rustc --test` and verify RED**

- [ ] **Step 3: Implement in new format**

Required comparisons:
- `35...rs`: 線形探索 / 本命の lower_bound / helper 切り出し
- `153...rs`: 線形最小探索 / 本命の二分探索 / 区間不変条件 helper

- [ ] **Step 4: Re-run tests and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/35.search-insert-position.rs arai60/answers/153.find-minimum-in-rotated-sorted-array.rs
git commit -m "feat: upgrade basic binary search answers"
```

## Chunk 2: rotated array and exponentiation

### Task 2: rewrite richer binary-search style problems

**Files:**
- Modify: `arai60/answers/33.search-in-rotated-sorted-array.rs`
- Modify: `arai60/answers/50.powx-n.rs`

- [ ] **Step 1: Write failing tests first**

Per-file focus:
- `33...rs`: target あり/なし、片側だけ整列しているケース
- `50...rs`: 正指数、負指数、`i32::MIN`

- [ ] **Step 2: Run `rustc --test` and verify RED**

- [ ] **Step 3: Implement in new format**

Required comparisons:
- `33...rs`: 線形探索 / 片側整列判定 binary search / helper 版
- `50...rs`: 素朴累乗 / 二分累乗 / 再帰 or 反復整理版

- [ ] **Step 4: Re-run tests and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/33.search-in-rotated-sorted-array.rs arai60/answers/50.powx-n.rs
git commit -m "feat: upgrade rotated search answers"
```

## Chunk 3: binary search as optimization helper

### Task 3: rewrite `300.longest-increasing-subsequence.rs`

**Files:**
- Modify: `arai60/answers/300.longest-increasing-subsequence.rs`

- [ ] **Step 1: Write failing tests first**

Focus:
- 基本ケース
- 重複を含むケース
- 単調増加 / 単調減少

- [ ] **Step 2: Run `rustc --test` and verify RED**

- [ ] **Step 3: Implement in new format**

Required comparisons:
- O(n^2) DP
- patience sorting + binary search
- tails 更新 helper 版

- [ ] **Step 4: Re-run tests and verify GREEN**

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/300.longest-increasing-subsequence.rs
git commit -m "feat: upgrade lis answer"
```

## Chunk 4: batch-wide verification

### Task 4: verify consistency across batch 3

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
- `35.search-insert-position.rs`
- `153.find-minimum-in-rotated-sorted-array.rs`
- `33.search-in-rotated-sorted-array.rs`
- `50.powx-n.rs`
- `300.longest-increasing-subsequence.rs`

Expected: PASS

- [ ] **Step 3: Commit the completed batch**

```bash
git add arai60/answers/35.search-insert-position.rs arai60/answers/153.find-minimum-in-rotated-sorted-array.rs arai60/answers/33.search-in-rotated-sorted-array.rs arai60/answers/50.powx-n.rs arai60/answers/300.longest-increasing-subsequence.rs
git commit -m "feat: upgrade binary search answer batch"
```

## Unresolved questions

- `50.powx-n.rs` は純粋な binary search ではないが、二分法・分割統治の思考としてこのバッチで扱う
