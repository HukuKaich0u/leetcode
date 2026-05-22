# arai60 Answers Batch 1 Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `配列・two pointers・sliding window` バッチの回答を、新 README 形式に沿って高品質化する

**Architecture:** 各問題ファイルを、厚めの解説、本命解法、比較用の別解、必要に応じた `素朴実装 / 本命実装 / 整理した実装` のコードレベル比較、`#[cfg(test)]` による自己完結テストを持つ教材へ作り替える。テスト実行は `rustc --test <file>` を基本にし、そのために各ファイルへ最小限の standalone 実行用 scaffolding も追加する。

**Tech Stack:** Rust, `rustc --test`, Markdown comments, git

---

## Chunk 1: batch scaffolding and first template

### Task 1: establish the local testable answer-file pattern

**Files:**
- Modify: `arai60/answers/1.two-sum.rs`
- Modify: `arai60/answers/283.move-zeroes.rs`
- Reference: `arai60/answers/README.md`
- Reference: `docs/superpowers/specs/2026-05-08-arai60-answers-rollout-design.md`

- [ ] **Step 1: Read the two files and identify required scaffolding**

Run: `sed -n '1,220p' arai60/answers/1.two-sum.rs`
Run: `sed -n '1,220p' arai60/answers/283.move-zeroes.rs`
Expected: どちらも `struct Solution;` と `#[cfg(test)]` が無く、余計な `use` があることが分かる

- [ ] **Step 2: Write the failing tests for `1.two-sum.rs`**

Add tests for:
- 基本ケース
- 重複値を含むケース
- 負数を含むケース
- 返り値の index 組が正しいこと

Expected compile target shape:

```rust
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
}
```

- [ ] **Step 3: Run the file test to verify it fails**

Run: `rustc --test arai60/answers/1.two-sum.rs -o /private/tmp/two_sum_tests && /private/tmp/two_sum_tests`
Expected: FAIL because tests expect scaffolding / behavior not yet implemented in the new structure

- [ ] **Step 4: Implement `1.two-sum.rs` in new format**

Required content:
- `この問題で鍛えること` から `テスト` までの全見出し
- `素朴実装`: 二重ループ
- `本命実装`: `HashMap`
- `整理した実装`: 相方探索ロジックや index 正規化の切り出しが本当に有益なら追加
- Rust コメント
- 不要な `use` の削除

- [ ] **Step 5: Run the file test to verify it passes**

Run: `rustc --test arai60/answers/1.two-sum.rs -o /private/tmp/two_sum_tests && /private/tmp/two_sum_tests`
Expected: PASS

- [ ] **Step 6: Write the failing tests for `283.move-zeroes.rs`**

Add tests for:
- 基本ケース
- 先頭や末尾に 0 があるケース
- 全て 0 のケース
- 非 0 の相対順が維持されること

- [ ] **Step 7: Run the file test to verify it fails**

Run: `rustc --test arai60/answers/283.move-zeroes.rs -o /private/tmp/move_zeroes_tests && /private/tmp/move_zeroes_tests`
Expected: FAIL before the rewrite is complete

- [ ] **Step 8: Implement `283.move-zeroes.rs` in new format**

Required content:
- `素朴実装`: 別配列を使う安直版
- `本命実装`: write pointer
- `整理した実装`: swap ベース版または書き込み処理を分けた版
- in-place 配列操作の不変条件の説明

- [ ] **Step 9: Run the file test to verify it passes**

Run: `rustc --test arai60/answers/283.move-zeroes.rs -o /private/tmp/move_zeroes_tests && /private/tmp/move_zeroes_tests`
Expected: PASS

- [ ] **Step 10: Commit the first template**

```bash
git add arai60/answers/1.two-sum.rs arai60/answers/283.move-zeroes.rs
git commit -m "feat: upgrade array answer templates"
```

## Chunk 2: sliding window and subsequence problems

### Task 2: rewrite the monotonic-window problems

**Files:**
- Modify: `arai60/answers/209.minimum-size-subarray-sum.rs`
- Modify: `arai60/answers/392.is-subsequence.rs`

- [ ] **Step 1: Write failing tests for `209.minimum-size-subarray-sum.rs`**

Add tests for:
- 基本ケース
- 解が存在しないケース
- 長さ 1 が答えになるケース
- shrink を複数回繰り返すケース

- [ ] **Step 2: Run test to verify it fails**

Run: `rustc --test arai60/answers/209.minimum-size-subarray-sum.rs -o /private/tmp/min_sub_array_tests && /private/tmp/min_sub_array_tests`
Expected: FAIL

- [ ] **Step 3: Implement `209.minimum-size-subarray-sum.rs`**

Required content:
- `素朴実装`: 全区間確認
- `本命実装`: sliding window
- `別解`: prefix sum + binary search
- `整理した実装`: `shrink_window` 相当が有益なら切り出す

- [ ] **Step 4: Run test to verify it passes**

Run: `rustc --test arai60/answers/209.minimum-size-subarray-sum.rs -o /private/tmp/min_sub_array_tests && /private/tmp/min_sub_array_tests`
Expected: PASS

- [ ] **Step 5: Write failing tests for `392.is-subsequence.rs`**

Add tests for:
- 基本ケース
- 空文字列
- `t` が短すぎるケース
- 同じ文字が多いケース

- [ ] **Step 6: Run test to verify it fails**

Run: `rustc --test arai60/answers/392.is-subsequence.rs -o /private/tmp/is_subsequence_tests && /private/tmp/is_subsequence_tests`
Expected: FAIL

- [ ] **Step 7: Implement `392.is-subsequence.rs`**

Required content:
- `素朴実装`: 再帰 or 走査位置を都度探す版
- `本命実装`: two pointers
- `別解`: 多 query 向け前処理案
- greedy の正当化の解説

- [ ] **Step 8: Run test to verify it passes**

Run: `rustc --test arai60/answers/392.is-subsequence.rs -o /private/tmp/is_subsequence_tests && /private/tmp/is_subsequence_tests`
Expected: PASS

- [ ] **Step 9: Commit**

```bash
git add arai60/answers/209.minimum-size-subarray-sum.rs arai60/answers/392.is-subsequence.rs
git commit -m "feat: upgrade sliding window answers"
```

## Chunk 3: greedy and simulation array problems

### Task 3: rewrite array transformation and greedy problems

**Files:**
- Modify: `arai60/answers/31.next-permutation.rs`
- Modify: `arai60/answers/48.rotate-image.rs`
- Modify: `arai60/answers/53.maximum-subarray.rs`
- Modify: `arai60/answers/121.best-time-to-buy-and-sell-stock.rs`
- Modify: `arai60/answers/122.best-time-to-buy-and-sell-stock-ii.rs`

- [ ] **Step 1: For each file, write failing tests first**

Add at least:
- 基本ケース
- 境界ケース
- 典型的な罠ケース

Per-file focus:
- `31.next-permutation.rs`: 降順配列、重複、pivot が中盤にあるケース
- `48.rotate-image.rs`: 1x1、2x2、3x3
- `53.maximum-subarray.rs`: 全負数、単一要素
- `121.best-time-to-buy-and-sell-stock.rs`: 利益ゼロ、最安値が後半
- `122.best-time-to-buy-and-sell-stock-ii.rs`: 上昇下降が交互に来るケース

- [ ] **Step 2: For each file, run `rustc --test` and verify RED**

Run:
- `rustc --test arai60/answers/31.next-permutation.rs -o /private/tmp/next_permutation_tests && /private/tmp/next_permutation_tests`
- `rustc --test arai60/answers/48.rotate-image.rs -o /private/tmp/rotate_image_tests && /private/tmp/rotate_image_tests`
- `rustc --test arai60/answers/53.maximum-subarray.rs -o /private/tmp/max_subarray_tests && /private/tmp/max_subarray_tests`
- `rustc --test arai60/answers/121.best-time-to-buy-and-sell-stock.rs -o /private/tmp/best_stock_tests && /private/tmp/best_stock_tests`
- `rustc --test arai60/answers/122.best-time-to-buy-and-sell-stock-ii.rs -o /private/tmp/best_stock2_tests && /private/tmp/best_stock2_tests`
Expected: FAIL

- [ ] **Step 3: Implement the files one by one in the new format**

Required level comparisons:
- `31.next-permutation.rs`: 素朴な next 全列挙案、本命の pivot + suffix reverse、必要なら swap 候補探索の切り出し
- `48.rotate-image.rs`: 補助行列版、本命の transpose + reverse、必要なら layer rotation
- `53.maximum-subarray.rs`: 全区間、Kadane、本命を補助関数へ切る価値があるかの比較
- `121.best-time-to-buy-and-sell-stock.rs`: 二重ループ、本命の min-so-far
- `122.best-time-to-buy-and-sell-stock-ii.rs`: valley-peak、本命の positive diff 累積

- [ ] **Step 4: Re-run all five file tests and verify GREEN**

Run the same five `rustc --test` commands from Step 2
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/31.next-permutation.rs arai60/answers/48.rotate-image.rs arai60/answers/53.maximum-subarray.rs arai60/answers/121.best-time-to-buy-and-sell-stock.rs arai60/answers/122.best-time-to-buy-and-sell-stock-ii.rs
git commit -m "feat: upgrade greedy and simulation answers"
```

## Chunk 4: remaining batch files and consistency pass

### Task 4: finish the remaining batch and align quality

**Files:**
- Modify: `arai60/answers/6.zigzag-conversion.rs`
- Modify: `arai60/answers/252.meeting-rooms.rs`
- Reference: all files in this batch

- [ ] **Step 1: Write failing tests for `6.zigzag-conversion.rs` and `252.meeting-rooms.rs`**

Per-file focus:
- `6.zigzag-conversion.rs`: `num_rows = 1`, 短い文字列、通常ケース
- `252.meeting-rooms.rs`: 重なりあり / なし、同時刻境界

- [ ] **Step 2: Run tests to verify RED**

Run:
- `rustc --test arai60/answers/6.zigzag-conversion.rs -o /private/tmp/zigzag_tests && /private/tmp/zigzag_tests`
- `rustc --test arai60/answers/252.meeting-rooms.rs -o /private/tmp/meeting_rooms_tests && /private/tmp/meeting_rooms_tests`
Expected: FAIL

- [ ] **Step 3: Implement both files in the new format**

Required content:
- `6.zigzag-conversion.rs`: 素朴な行ごとの simulation、本命の方向転換 simulation、必要なら index 規則ベースの別解
- `252.meeting-rooms.rs`: 素朴な全比較、本命の sort 後隣接比較、必要なら starts/ends 分離案

- [ ] **Step 4: Run tests to verify GREEN**

Run the same two `rustc --test` commands from Step 2
Expected: PASS

- [ ] **Step 5: Perform a batch-wide consistency pass**

Check:
- 全ファイルに `struct Solution;` がある
- 全ファイルに `#[cfg(test)]` がある
- 余計な `use` が消えている
- `素朴実装 / 本命実装 / 整理した実装` の扱いが一貫している
- `落とし穴` と `Rust観点の解説` が空文化していない

- [ ] **Step 6: Run all batch file tests**

Run:
- `rustc --test arai60/answers/1.two-sum.rs -o /private/tmp/two_sum_tests && /private/tmp/two_sum_tests`
- `rustc --test arai60/answers/6.zigzag-conversion.rs -o /private/tmp/zigzag_tests && /private/tmp/zigzag_tests`
- `rustc --test arai60/answers/31.next-permutation.rs -o /private/tmp/next_permutation_tests && /private/tmp/next_permutation_tests`
- `rustc --test arai60/answers/48.rotate-image.rs -o /private/tmp/rotate_image_tests && /private/tmp/rotate_image_tests`
- `rustc --test arai60/answers/53.maximum-subarray.rs -o /private/tmp/max_subarray_tests && /private/tmp/max_subarray_tests`
- `rustc --test arai60/answers/121.best-time-to-buy-and-sell-stock.rs -o /private/tmp/best_stock_tests && /private/tmp/best_stock_tests`
- `rustc --test arai60/answers/122.best-time-to-buy-and-sell-stock-ii.rs -o /private/tmp/best_stock2_tests && /private/tmp/best_stock2_tests`
- `rustc --test arai60/answers/209.minimum-size-subarray-sum.rs -o /private/tmp/min_sub_array_tests && /private/tmp/min_sub_array_tests`
- `rustc --test arai60/answers/252.meeting-rooms.rs -o /private/tmp/meeting_rooms_tests && /private/tmp/meeting_rooms_tests`
- `rustc --test arai60/answers/283.move-zeroes.rs -o /private/tmp/move_zeroes_tests && /private/tmp/move_zeroes_tests`
- `rustc --test arai60/answers/392.is-subsequence.rs -o /private/tmp/is_subsequence_tests && /private/tmp/is_subsequence_tests`
Expected: PASS

- [ ] **Step 7: Commit the completed batch**

```bash
git add arai60/answers/1.two-sum.rs arai60/answers/6.zigzag-conversion.rs arai60/answers/31.next-permutation.rs arai60/answers/48.rotate-image.rs arai60/answers/53.maximum-subarray.rs arai60/answers/121.best-time-to-buy-and-sell-stock.rs arai60/answers/122.best-time-to-buy-and-sell-stock-ii.rs arai60/answers/209.minimum-size-subarray-sum.rs arai60/answers/252.meeting-rooms.rs arai60/answers/283.move-zeroes.rs arai60/answers/392.is-subsequence.rs
git commit -m "feat: upgrade batch1 array answers"
```

## Unresolved questions

- `1.two-sum.rs` をこのバッチに残すか hash バッチへ移すかは実装前に再判断してよい。ただし bridge problem としてここで処理してもよい
