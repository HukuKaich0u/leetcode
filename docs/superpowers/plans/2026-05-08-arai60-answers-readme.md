# arai60 Answers README Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `arai60/answers/README.md` を、回答品質の仕様書として全面改稿する

**Architecture:** 既存 README を薄い説明文から、教材設計・執筆規約・回答テンプレートを定義する文書へ置き換える。仕様の根拠は `docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md` に寄せ、README は日常的に参照する実用版にする。

**Tech Stack:** Markdown, git

---

## Chunk 1: README structure rewrite

### Task 1: replace thin overview with purpose and training goals

**Files:**
- Modify: `arai60/answers/README.md`
- Reference: `docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md`

- [ ] **Step 1: Read current README and spec side by side**

Run: `sed -n '1,220p' arai60/answers/README.md`
Run: `sed -n '1,260p' docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md`
Expected: 現 README の不足と、新 README に入れる章が明確になる

- [ ] **Step 2: Rewrite README intro and positioning**

更新内容:
- このディレクトリの目的
- 鍛える 3 軸
- `面接で説明できる` は最低ラインで、主目的は基礎固めであること

- [ ] **Step 3: Add mandatory per-file structure**

README に以下を必須構成として書く:
- `この問題で鍛えること`
- `問題の本質`
- `解法候補`
- `採用解法の説明`
- `Rust実装`
- `別解`
- `Rust観点の解説`
- `いいコード観点の解説`
- `落とし穴`
- `テスト`

- [ ] **Step 4: Review prose for depth and redundancy**

確認観点:
- 解説が薄すぎない
- 仕様書の丸写しで読みにくくなっていない
- README 単体で執筆ルールが分かる

- [ ] **Step 5: Commit**

```bash
git add arai60/answers/README.md
git commit -m "docs: redesign arai60 answers README"
```

## Chunk 2: authoring rules and examples

### Task 2: add implementation, comments, and tests guidance

**Files:**
- Modify: `arai60/answers/README.md`
- Reference: `docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md`

- [ ] **Step 1: Add multi-solution policy**

README に明記:
- 本命解法 1 個
- 比較用の別解 1〜2 個
- 各解法に `考え方` `計算量` `データ構造` `採用/非採用理由`
- 可能なら `素朴実装` `本命実装` `整理した実装` の 3 レベル比較

- [ ] **Step 2: Add Rust comment policy**

README に明記:
- コメントを書く場所
- コメントに書くべき内容
- 避けるべきコメント
- 短いコード例

- [ ] **Step 3: Add test policy**

README に明記:
- `#[cfg(test)]` を各 `.rs` に同居
- 基本ケース
- 境界ケース
- 罠ケース
- 本命解法と別解の共通検証
- コードレベル違いの実装も同じ仕様で検証

- [ ] **Step 4: Add good/bad answer criteria and checklist**

README に明記:
- 良い回答の特徴
- 悪い回答の特徴
- 執筆前後の確認チェックリスト

- [ ] **Step 5: Verify final diff**

Run: `git diff -- arai60/answers/README.md`
Expected: README が「回答置き場の説明」から「回答品質の仕様書」に変わっている

- [ ] **Step 6: Commit**

```bash
git add arai60/answers/README.md
git commit -m "docs: add arai60 answers authoring rules"
```

## Chunk 3: final verification

### Task 3: verify readability and alignment with spec

**Files:**
- Modify: `arai60/answers/README.md` if needed
- Reference: `docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md`

- [ ] **Step 1: Read final README top to bottom**

Run: `sed -n '1,260p' arai60/answers/README.md`
Expected: README 単体で、各回答ファイルをどう書くか迷わない

- [ ] **Step 2: Spot-check against spec**

確認項目:
- 目的
- 必須構成
- 複数解法
- コードレベル比較
- Rust コメント
- テスト
- 良い/悪い回答
- チェックリスト

- [ ] **Step 3: Make final wording edits if gaps remain**

必要なら調整:
- 冗長な箇所を圧縮
- 足りない規約を追加
- 見出し名を揃える

- [ ] **Step 4: Re-run diff for final sanity check**

Run: `git diff -- arai60/answers/README.md`
Expected: 意図しない削除や不足がない

- [ ] **Step 5: Final commit if Step 3 changed content**

```bash
git add arai60/answers/README.md
git commit -m "docs: polish arai60 answers README"
```

## Unresolved questions

- なし
