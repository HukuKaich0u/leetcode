# arai60 Answers Tree/BST Batch Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 残り 9 問の tree / BST 回答を README 形式へ揃え、`1 問 1 ファイルで学習が完結する教材` にする

**Architecture:** `tree traversal`、`BST invariant`、`tree reconstruction` の 3 群で処理。各ファイルは `本命 + 別解 + file-local test` を必須にし、検証は `rustc --test --crate-name ...` で統一。

**Tech Stack:** Rust, `rustc --test`, `Rc<RefCell<TreeNode>>`, `VecDeque`, git

---

## Chunk 1: traversal core

### Task 1: rewrite traversal-focused tree files

**Files:**
- Modify: `arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs`
- Modify: `arai60/answers/104.maximum-depth-of-binary-tree.rs`
- Modify: `arai60/answers/111.minimum-depth-of-binary-tree.rs`
- Modify: `arai60/answers/112.path-sum.rs`
- Reference: `arai60/answers/README.md`
- Reference: `docs/superpowers/specs/2026-05-11-arai60-tree-bst-batch-design.md`

- [ ] **Step 1: Write failing tests first**

Focus:
- `103`: empty, single level, skewed tree
- `104`: empty, single node, unbalanced tree
- `111`: shallow leaf vs deep leaf, single-child trap
- `112`: hit, miss, prefix-only-not-leaf trap

- [ ] **Step 2: Run RED**

Run:
- `rustc --test --crate-name answer103 arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs -o /private/tmp/answer103 && /private/tmp/answer103`
- `rustc --test --crate-name answer104 arai60/answers/104.maximum-depth-of-binary-tree.rs -o /private/tmp/answer104 && /private/tmp/answer104`
- `rustc --test --crate-name answer111 arai60/answers/111.minimum-depth-of-binary-tree.rs -o /private/tmp/answer111 && /private/tmp/answer111`
- `rustc --test --crate-name answer112 arai60/answers/112.path-sum.rs -o /private/tmp/answer112 && /private/tmp/answer112`

Expected: FAIL

- [ ] **Step 3: Implement new format**

Required comparisons:
- `103`: BFS level-order + reverse / DFS by depth
- `104`: recursive DFS / BFS or explicit stack
- `111`: BFS first-leaf / recursive DFS
- `112`: recursive remain / iterative stack `(node, remain)`

Must cover:
- 10 見出し
- `Rc<RefCell<_>>` の clone / borrow 意図
- 状態と不変条件
- file-local builder/test helper

- [ ] **Step 4: Run GREEN**

Run same 4 commands from Step 2  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs arai60/answers/104.maximum-depth-of-binary-tree.rs arai60/answers/111.minimum-depth-of-binary-tree.rs arai60/answers/112.path-sum.rs
git commit -m "feat: upgrade traversal tree answers"
```

### Task 2: rewrite merge tree file

**Files:**
- Modify: `arai60/answers/617.merge-two-binary-trees.rs`

- [ ] **Step 1: Write failing tests first**

Focus:
- basic merge
- subtree missing on one side
- one whole root `None`

- [ ] **Step 2: Run RED**

Run:
- `rustc --test --crate-name answer617 arai60/answers/617.merge-two-binary-trees.rs -o /private/tmp/answer617 && /private/tmp/answer617`

Expected: FAIL

- [ ] **Step 3: Implement new format**

Required comparisons:
- recursive merge
- queue-based merge

Must cover:
- local merge rule
- `None` handling
- `clone_tree` を切る理由

- [ ] **Step 4: Run GREEN**

Run same command from Step 2  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/617.merge-two-binary-trees.rs
git commit -m "feat: upgrade merge tree answer"
```

## Chunk 2: BST and reconstruction

### Task 3: rewrite BST invariant files

**Files:**
- Modify: `arai60/answers/98.validate-binary-search-tree.rs`
- Modify: `arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs`

- [ ] **Step 1: Write failing tests first**

Focus:
- `98`: valid, global-invalid, duplicate, boundary values
- `108`: empty, one node, even/odd length, inorder preserved, balanced

- [ ] **Step 2: Run RED**

Run:
- `rustc --test --crate-name answer98 arai60/answers/98.validate-binary-search-tree.rs -o /private/tmp/answer98 && /private/tmp/answer98`
- `rustc --test --crate-name answer108 arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs -o /private/tmp/answer108 && /private/tmp/answer108`

Expected: FAIL

- [ ] **Step 3: Implement new format**

Required comparisons:
- `98`: range-check DFS / inorder monotonic check
- `108`: middle-pick recursion / index-range recursion

Must cover:
- BST だから省ける探索
- range invariant
- balanced の意味

- [ ] **Step 4: Run GREEN**

Run same 2 commands from Step 2  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/98.validate-binary-search-tree.rs arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs
git commit -m "feat: upgrade bst invariant answers"
```

### Task 4: rewrite reconstruction and split files

**Files:**
- Modify: `arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs`
- Modify: `arai60/answers/776.split-bst.rs`

- [ ] **Step 1: Write failing tests first**

Focus:
- `105`: empty, single node, left-skewed or right-skewed
- `776`: all small, all large, split at root, inorder preserved

- [ ] **Step 2: Run RED**

Run:
- `rustc --test --crate-name answer105 arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs -o /private/tmp/answer105 && /private/tmp/answer105`
- `rustc --test --crate-name answer776 arai60/answers/776.split-bst.rs -o /private/tmp/answer776 && /private/tmp/answer776`

Expected: FAIL

- [ ] **Step 3: Implement new format**

Required comparisons:
- `105`: index-map interval DFS / slice + linear-search
- `776`: destructive split with `take()` / clone-filter rebuild

Must cover:
- `105`: preorder root, inorder boundary, `left_size`, synced intervals
- `776`: `(small, large)` return contract, reconnect invariant

- [ ] **Step 4: Run GREEN**

Run same 2 commands from Step 2  
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs arai60/answers/776.split-bst.rs
git commit -m "feat: upgrade tree reconstruction answers"
```

## Chunk 3: batch verification

### Task 5: align format and verify all 9 files

**Files:**
- Reference: all files in this batch

- [ ] **Step 1: Check format consistency**

Confirm in all 9:
- 10 見出し
- `struct Solution;`
- `#[cfg(test)]`
- `本命 + 別解`
- Rust / code-quality / pitfalls sections

- [ ] **Step 2: Run all tests**

Run:
- `rustc --test --crate-name answer103 arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs -o /private/tmp/answer103 && /private/tmp/answer103`
- `rustc --test --crate-name answer104 arai60/answers/104.maximum-depth-of-binary-tree.rs -o /private/tmp/answer104 && /private/tmp/answer104`
- `rustc --test --crate-name answer105 arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs -o /private/tmp/answer105 && /private/tmp/answer105`
- `rustc --test --crate-name answer108 arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs -o /private/tmp/answer108 && /private/tmp/answer108`
- `rustc --test --crate-name answer111 arai60/answers/111.minimum-depth-of-binary-tree.rs -o /private/tmp/answer111 && /private/tmp/answer111`
- `rustc --test --crate-name answer112 arai60/answers/112.path-sum.rs -o /private/tmp/answer112 && /private/tmp/answer112`
- `rustc --test --crate-name answer617 arai60/answers/617.merge-two-binary-trees.rs -o /private/tmp/answer617 && /private/tmp/answer617`
- `rustc --test --crate-name answer776 arai60/answers/776.split-bst.rs -o /private/tmp/answer776 && /private/tmp/answer776`
- `rustc --test --crate-name answer98 arai60/answers/98.validate-binary-search-tree.rs -o /private/tmp/answer98 && /private/tmp/answer98`

Expected: PASS

- [ ] **Step 3: Commit batch**

```bash
git add arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs arai60/answers/104.maximum-depth-of-binary-tree.rs arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs arai60/answers/111.minimum-depth-of-binary-tree.rs arai60/answers/112.path-sum.rs arai60/answers/617.merge-two-binary-trees.rs arai60/answers/776.split-bst.rs arai60/answers/98.validate-binary-search-tree.rs
git commit -m "feat: upgrade tree bst answer batch"
```

## Unresolved questions

- none
