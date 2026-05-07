# arai60 answers rollout redesign

## Goal

`arai60/answers` 配下の全問題を、README で定義した新フォーマットに沿って順次リライトする。

目的は単なる AC コードの蓄積ではない。各問題を通して次の 3 つを同時に鍛える教材へ統一する。

- Rust を使って破綻なく書く力
- いいコードを書く力
- アルゴリズムとデータ構造の力

## Scope

対象は `arai60/answers/README.md` を除く全問題ファイル。

- 問題数: 60
- 単位: 1 問 1 ファイル
- 形式: 各 `.rs` 内に解説、複数解法、コードレベル違いの実装比較、Rust コメント、テストを同居させる

このリライトは、「コードだけ差し替える」作業ではない。各問題ファイルを教材として再設計する作業である。

## Source of truth

全問題の完成条件は以下を基準にする。

- `arai60/answers/README.md`
- `docs/superpowers/specs/2026-05-08-arai60-answers-readme-design.md`

README は日常的に参照する実用版、spec は設計根拠を残す文書とする。

## Why batch by problem type

全 60 問を 1 つずつ場当たり的に直すのではなく、問題の型ごとに処理する。

理由:

- 同じ型の問題に同じ説明軸を適用しやすい
- 別解の候補が揃いやすい
- Rust 実装とテストのパターンを再利用しやすい
- 解説品質のばらつきを抑えやすい

## Batch order

次の順で進める。

1. 配列・two pointers・sliding window
2. HashMap / HashSet
3. binary search
4. linked list
5. tree / graph
6. DP
7. backtracking / recursion
8. heap / queue / stack
9. 横断型 / その他

この順序の意図:

- 初期バッチで、説明テンプレートを多くの問題へ使い回せる
- 後半の tree / graph / DP で、より重い解説を安定して書ける
- 各バッチの学習テーマが明確になる

## Tentative batch mapping

厳密な分類は実装時に微調整してよいが、初期マッピングは以下とする。

### 1. 配列・two pointers・sliding window

- `1.two-sum.rs`
- `6.zigzag-conversion.rs`
- `31.next-permutation.rs`
- `48.rotate-image.rs`
- `53.maximum-subarray.rs`
- `121.best-time-to-buy-and-sell-stock.rs`
- `122.best-time-to-buy-and-sell-stock-ii.rs`
- `209.minimum-size-subarray-sum.rs`
- `252.meeting-rooms.rs`
- `283.move-zeroes.rs`
- `392.is-subsequence.rs`

### 2. HashMap / HashSet

- `3.longest-substring-without-repeating-characters.rs`
- `20.valid-parentheses.rs`
- `49.group-anagrams.rs`
- `127.word-ladder.rs`
- `347.top-k-frequent-elements.rs`
- `349.intersection-of-two-arrays.rs`
- `387.first-unique-character-in-a-string.rs`
- `560.subarray-sum-equals-k.rs`
- `929.unique-email-addresses.rs`

### 3. binary search

- `33.search-in-rotated-sorted-array.rs`
- `35.search-insert-position.rs`
- `50.powx-n.rs`
- `153.find-minimum-in-rotated-sorted-array.rs`
- `300.longest-increasing-subsequence.rs`

### 4. linked list

- `2.add-two-numbers.rs`
- `82.remove-duplicates-from-sorted-list-ii.rs`
- `83.remove-duplicates-from-sorted-list.rs`
- `141.linked-list-cycle.rs`
- `142.linked-list-cycle-ii.rs`
- `206.reverse-linked-list.rs`

### 5. tree / graph

- `102.binary-tree-level-order-traversal.rs`
- `103.binary-tree-zigzag-level-order-traversal.rs`
- `104.maximum-depth-of-binary-tree.rs`
- `105.construct-binary-tree-from-preorder-and-inorder-traversal.rs`
- `108.convert-sorted-array-to-binary-search-tree.rs`
- `111.minimum-depth-of-binary-tree.rs`
- `112.path-sum.rs`
- `200.number-of-islands.rs`
- `323.number-of-connected-components-in-an-undirected-graph.rs`
- `617.merge-two-binary-trees.rs`
- `695.max-area-of-island.rs`
- `776.split-bst.rs`
- `98.validate-binary-search-tree.rs`

### 6. DP

- `62.unique-paths.rs`
- `63.unique-paths-ii.rs`
- `139.word-break.rs`
- `198.house-robber.rs`
- `213.house-robber-ii.rs`
- `276.paint-fence.rs`
- `322.coin-change.rs`

### 7. backtracking / recursion

- `22.generate-parentheses.rs`
- `39.combination-sum.rs`
- `46.permutations.rs`
- `78.subsets.rs`
- `779.k-th-symbol-in-grammar.rs`

### 8. heap / queue / stack

- `253.meeting-rooms-ii.rs`
- `373.find-k-pairs-with-smallest-sums.rs`
- `703.kth-largest-element-in-a-stream.rs`

### 9. 横断型 / その他

- `8.string-to-integer-atoi.rs`

## Per-file completion criteria

各問題ファイルは、少なくとも次を含む。

1. `この問題で鍛えること`
2. `問題の本質`
3. `解法候補`
4. `採用解法の説明`
5. `Rust実装`
6. `別解`
7. `Rust観点の解説`
8. `いいコード観点の解説`
9. `落とし穴`
10. `テスト`

## Explanation depth standard

解説は「面接で一言しゃべれる」深さでは足りない。

各問題で最低限、次まで踏み込む。

- どの抽象化で問題の本質が見えるか
- なぜ本命解法が成立するか
- 他の候補解法がどこで劣るか
- 状態、不変条件、単調性、探索順の意味
- 境界条件や罠ケース
- Rust で自然な表現は何か
- コード品質として何が良いのか
- 同じ解法でも、どこまで 1 メソッドで書き、どこから切り出すべきか

## Code requirements

本命解法は `impl Solution` に置く。別解は 1〜2 個置く。

可能な限り、各問題で次の 3 レベルを比較する。

- 素朴実装: メソッド数を抑え、自前で愚直に書く
- 本命実装: 面接でも実務でも説明しやすい形に整理する
- 整理した実装: 何度も使う処理を補助関数へ切り出す

ここで比較したいのは、アルゴリズムの違いだけではない。コードの育て方、責務の分け方、再利用の判断も含める。

コードに対する要求:

- `use` は必要最小限
- 変数名は役割ベース
- 処理ブロックごとに短い意図コメントを入れる
- Rust 特有で迷いやすい箇所だけ補足する
- 本命は説明しやすさと壊れにくさを優先する
- 別解は比較価値があるものだけ置く
- 素朴実装と整理版の差が分かるようにする

## Test requirements

各 `.rs` に `#[cfg(test)]` を同居させる。

最低限の観点:

- 基本ケース
- 境界ケース
- 罠ケース

必要に応じて追加する観点:

- 重複
- 負数
- 到達不能
- 長い入力
- 破壊的更新
- 元 index の保持

本命解法と別解があるなら、可能な限り同じ仕様で両方を検証する。
コードレベル違いの実装も、同じテストで比較できるようにする。

## Execution model per batch

各バッチは次の流れで処理する。

1. 対象ファイルを読む
2. README 形式に沿って解説構成を決める
3. まずテスト方針を決める
4. 素朴実装を書いて、どこで苦しくなるかを見る
5. 本命解法を実装する
6. 必要なら整理した実装を追加する
7. 別解を 1〜2 個実装する
8. 各実装のコードレベル差を解説する
9. Rust 観点 / コード品質観点 / 落とし穴を書く
10. `#[cfg(test)]` を追加する
11. バッチ単位で見直す

## Quality bar per batch

バッチ完了と呼べるのは次を満たしたときだけ。

- 全問題が README の必須構成を持つ
- 各問題に本命解法と別解がある
- 各問題でコードレベル比較がある、または不要な理由が説明されている
- Rust コメントが方針に従っている
- テストが同居している
- 不要な `use` が消えている
- 解説の粒度がバッチ内で揃っている

## Risks

主なリスク:

- 問題ごとに解説の密度がぶれる
- 別解が比較価値のない実装違いに落ちる
- 素朴実装と整理版の差が曖昧になる
- Rust コメントが実況調になってノイズ化する
- テストが基本ケースだけで終わる
- 型分類が曖昧な問題で迷う

対策:

- README を単なる参考ではなく必須仕様として扱う
- 別解は「考え方が違う」ものに限定する
- 同じ解法内でも、コードの粒度差に学習価値があるなら残す
- コメントは意図とつまずきどころだけに絞る
- 各バッチで境界ケースの観点を先に洗い出す
- 型分類が曖昧なら、本質を優先して所属を決める

## Non-goals

この段階では次はやらない。

- 共通ライブラリ化
- 問題ファイルの自動生成
- 複数ファイルへの分割
- README と spec を超える新しい執筆システムの導入

## Deliverables

この設計の成果物は次の 3 つ。

- README に沿ってリライトされた全 60 問
- 各問題ファイル内のテスト
- 問題タイプごとに揃った解説品質

## Unresolved questions

- `1.two-sum.rs` のように配列本質と hash 本質の両方を持つ問題を、どのバッチに置くかは実装時に微調整してよい
- `20.valid-parentheses.rs` のように stack と hash の境界にある問題も同様
