# arai60 tree / BST batch design

## Goal

`arai60/answers` に残っている tree / BST 系 9 問を、`arai60/answers/README.md` の新フォーマットに揃えて完成させる。

このバッチでも主目的は変えない。

- Rust を使って破綻なく書く力
- いいコードを書く力
- アルゴリズムとデータ構造の力

各ファイルは AC コードではなく、その問題だけ読めば学習の主要論点を一通り辿れる教材にする。

## Scope

対象は次の 9 ファイルに限定する。

- `arai60/answers/103.binary-tree-zigzag-level-order-traversal.rs`
- `arai60/answers/104.maximum-depth-of-binary-tree.rs`
- `arai60/answers/105.construct-binary-tree-from-preorder-and-inorder-traversal.rs`
- `arai60/answers/108.convert-sorted-array-to-binary-search-tree.rs`
- `arai60/answers/111.minimum-depth-of-binary-tree.rs`
- `arai60/answers/112.path-sum.rs`
- `arai60/answers/617.merge-two-binary-trees.rs`
- `arai60/answers/776.split-bst.rs`
- `arai60/answers/98.validate-binary-search-tree.rs`

この spec は、`docs/superpowers/specs/2026-05-08-arai60-answers-rollout-design.md` の `tree / graph` バッチを、残件だけに絞って具体化した補助 spec とする。

## Positioning

このバッチは tree 問題を 1 問ずつ個別最適化するのではなく、次の 3 群に分けて揃える。

1. `tree traversal` 群
2. `BST invariant` 群
3. `tree reconstruction` 群

狙いは、説明の統一感を保ちつつ、問題固有で厚く書くべき論点だけを追加で強化することにある。

## Shared file contract

各ファイルは README で定義した次の 10 見出しを必須とする。

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

加えて、各ファイルで最低限そろえる実装要件は以下とする。

- `struct Solution;` を本命解法に使う
- 別解は比較価値があるものだけ `1 個以上` 置く
- `#[cfg(test)]` でファイル内完結のテストを置く
- 木構造の型定義と test helper も同じファイルに閉じる
- コメントは不変条件、状態、Rust 特有の所有権処理に限定する

## Shared learning axes

全 9 問で共通して見せたい学習軸は次の 4 つ。

### 1. 木をどう走査するか

- DFS か BFS か
- 再帰か反復か
- その問題で最も自然な探索順は何か

### 2. 何を状態として持つか

- 深さ
- 残り和
- preorder / inorder の区間
- BST の上下限
- 分割後の左右部分木

### 3. どの不変条件で正しさを支えるか

- queue / stack に何が積まれているか
- 再帰が何を返すか
- その時点で何が確定しているか
- BST の順序性をどこで使うか

### 4. Rust で木をどう扱うか

- `Rc<RefCell<TreeNode>>` を clone しながら読む場面
- `borrow()` と `borrow_mut()` の使い分け
- `take()` で部分木を切り離す場面
- test helper を含めても読みやすさを崩さない構成

## Group 1: tree traversal

対象:

- `103.binary-tree-zigzag-level-order-traversal.rs`
- `104.maximum-depth-of-binary-tree.rs`
- `111.minimum-depth-of-binary-tree.rs`
- `112.path-sum.rs`
- `617.merge-two-binary-trees.rs`

### Design rule

この群では毎回、`node を見た瞬間に分かること` と `子へ渡す状態` を中心に解説する。

同じ tree 問題でも、持つ状態が違うことを比較で見せる。

- `103`: level と左右方向
- `104`: 部分木の深さ
- `111`: 最初に葉へ着いた深さ
- `112`: root から現在 node までの残り和
- `617`: 対応する 2 node の組

### Per-file strategy

#### `103.binary-tree-zigzag-level-order-traversal.rs`

- 本命: BFS level order + 奇数段だけ reverse
- 別解: DFS で depth ごとに蓄積
- 本質: level ごとに集める処理と zigzag の向き制御を分けて考える

#### `104.maximum-depth-of-binary-tree.rs`

- 本命: 再帰 DFS
- 別解: BFS または stack
- 本質: depth は `1 + max(left, right)` の返り値として自然に定義できる

#### `111.minimum-depth-of-binary-tree.rs`

- 本命: BFS
- 別解: 再帰 DFS
- 本質: 最小 depth は最初に葉へ到達した時点で確定するので BFS が自然
- 強調点: `片側だけ子がある node` を `min(left, right)` で雑に処理しない

#### `112.path-sum.rs`

- 本命: 再帰 DFS
- 別解: stack に `(node, remain)` を積む反復版
- 本質: 判定対象は `root-to-leaf` に限られ、状態は `remain = target - path_sum_so_far`

#### `617.merge-two-binary-trees.rs`

- 本命: 再帰 merge
- 別解: queue を使う反復 merge
- 本質: 各位置での局所ルールは単純なので、再帰で左右部分木へ同じ規則を適用できる
- 強調点: `None` を含む場合の分岐と、必要なら clone_tree を補助関数で分ける

## Group 2: BST invariant

対象:

- `98.validate-binary-search-tree.rs`
- `108.convert-sorted-array-to-binary-search-tree.rs`
- `776.split-bst.rs`

### Design rule

この群では毎回、`BST だから何を省けるか` を明示する。

全探索木として扱うのではなく、順序性や範囲制約で枝刈りできる点を前面に出す。

### Per-file strategy

#### `98.validate-binary-search-tree.rs`

- 本命: `low/high` を持つ範囲制約 DFS
- 別解: inorder が strictly increasing かを見る版
- 本質: 親子関係だけでなく、祖先から受け継いだ制約まで含めて検証する必要がある
- 強調点: 局所的には正しそうでも全体で壊れる反例を厚く扱う

#### `108.convert-sorted-array-to-binary-search-tree.rs`

- 本命: 中央要素を root にする再帰
- 別解: index range を明示する版
- 本質: inorder が元配列と一致し、高さ差を抑えるには常に中央を根にするのが自然
- 強調点: `balanced` の意味と、slice 版 / index 版の読みやすさの比較

#### `776.split-bst.rs`

- 本命: 部分木を `take()` して再接続する destructive split
- 別解: 条件に合う node だけ clone して作り直す版
- 本質: root の値と target の比較だけで、片側の部分木は丸ごと結果へ残せる
- 強調点: 再帰の返り値を `(small, large)` として固定し、不変条件を保ちながら右または左をつなぎ直す

## Group 3: tree reconstruction

対象:

- `105.construct-binary-tree-from-preorder-and-inorder-traversal.rs`

### Design rule

この問題だけは traversal でも BST でもなく、`配列上の区間管理を木へ写像する問題` として扱う。

### Strategy

- 本命: inorder の index map を使う区間 DFS
- 別解: slice を切りながら線形探索する版
- 本質:
  - preorder の先頭が root を決める
  - inorder の root 位置が左右部分木の境界を決める
  - 区間だけを正しく更新すれば木全体を再構築できる

### Why thicker explanation

この問題は再帰の形だけ真似しても理解が浅くなりやすい。

必ず次を説明する。

- 返り値の木と入力区間の対応
- `left_size` がどこから出るか
- preorder / inorder の区間がどう同期して縮むか
- `HashMap` を使うと何が O(1) になるか

## Test strategy

各ファイルで `基本ケース + 境界ケース + 罠ケース` を最低ラインにする。

### Required edge cases

#### `103`

- 空木
- 1 level
- 片側だけ深い木

#### `104`

- 空木
- 1 node
- 左右で深さが偏る木

#### `105`

- 空木
- 1 node
- 片側に偏る木

#### `108`

- 空配列
- 1 要素
- 偶数長 / 奇数長
- inorder が元配列へ戻ること
- 高さ差が 1 以内に収まること

#### `111`

- 空木
- 片側だけ深い木
- 浅い葉と深い葉が混在する木

#### `112`

- path が存在するケース
- path が存在しないケース
- 途中和は一致するが leaf で終わらないケース

#### `617`

- 両方非空
- 片方だけ空の部分木を含むケース
- 片方全体が空のケース

#### `776`

- 全 node が `<= target`
- 全 node が `> target`
- target が root より小さい / 大きい
- inorder の順序が保たれること

#### `98`

- 正常 BST
- 局所的には正しいが全体で壊れている木
- 重複値を含むケース
- `i32` 境界値を含むケース

## Code quality rules

このバッチで特に守るコード品質ルールは次の通り。

- 木 problem だからといって helper を増やしすぎない
- 本命実装は、探索の骨格が一目で追える形を優先する
- clone は必要な場面に限定し、`Rc<RefCell<_>>` 特有の都合であることを説明する
- test helper は冗長でもよいが、本体ロジックの読みやすさを壊さない場所に閉じる
- 重い問題だけ厚く書くが、見出し構成そのものは崩さない

## Completion bar

このバッチが完了したと言える条件は次。

- 9 問すべてが README の 10 見出しを持つ
- 各問題で `本命 + 別解` の比較がある
- `Rustを書く力 / いいコードを書く力 / アルゴリズムとデータ構造の力` の 3 軸が見える
- tree traversal 群、BST invariant 群、tree reconstruction 群で説明の軸が揃っている
- `#[cfg(test)]` のテストが file-local にあり、主要ケースを押さえている
- `Rc<RefCell<_>>` や `take()` の扱いが、コードと解説の両方で破綻していない

## Risks

主なリスクは次の通り。

- `104` と `111` の説明が似すぎて、BFS を選ぶ理由の差が薄くなる
- `98` を inorder 性質だけで済ませると、範囲制約の学習が弱くなる
- `105` を実装手順だけで説明すると、区間設計の本質が伝わらない
- `776` で destructive split の不変条件が曖昧だと、Rust 実装の意図が伝わらない
- test helper の都合で file が長くなりすぎると、本命実装が埋もれる
