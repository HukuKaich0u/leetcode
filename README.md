# leetcode
LeetCodeの練習用リポジトリ
[問題集](https://1kohei1.com/leetcode/)
番号順ではなく、カテゴリ別で解いていくのが良さそう

# 利用手順
- `1. Two Sum` みたいな感じで、問題名のブランチを作成する
- Pull Requestのメッセージは以下のフォーマットで行う

```md
問題: [問題へのリンク](https://leetcode.com/problems/example)
次に解く問題: [次に解く問題へのリンク](https://leetcode.com/problems/example)
ファイルの構成: `./src/bin/<各ステップ>.rs`
```

- Discordのレビュー依頼チャンネルに投稿する
```md
お世話になっております。
1. Two Sum に取り組みました。
お手隙の際にレビューをお願いいたします。
問題: https://leetcode.com/problems/two-sum/description
PR: https://github.com/hukukaich0u/leetcode/pull/example
言語: Rust
<直近で同じ問題を解いた5人のメンション>
```

# 練習方法
    - Step1:
        答えを見ずに5分考える。5分考えて分からなかったら答えを見る
        答えを見て理解したと思ったら、答えを隠して書く
        筆が進まず5分迷ったら答えを見る
        見てしまったら、一回全部消してやり直し
        これを繰り返して、答えを送信して正解なったらStep1が終了
    - Step2:
        コードが見やすくなるように、できるだけ整える
        自分のコードに尽きそうなコメントの予測をする
        動くコードになったらStep2が終了
    - Step3:
        全部消す
        今度は時間を計りながらもう1回書く
        書いてAcceptされたら、文字を消してもう1回書く
        これを10分以内に1回もエラーを出さずに書ける状態になるまで続ける
        3回続けてそれができたらその問題はひとまず丸
    - Step4:
        過去に同じ問題を解いている人のコードレビューを読む
    3ステップ目まで終わったタイミングで講師陣にレビューを依頼し、レビューを元にコードを書き直す
    LeetCodeの問題とは直接関係ないが、関連するライブラリの再実装などをするのもよい
    たとえば、@lru_cacheやheapqライブラリなどを使用した会報があったので、
    LRU Cacheの実装やBinary Heapの実装なども行う

# 参考リンク
## すでにRustで実装しているリポジトリ
- https://github.com/t9a-dev/LeetCode_arai60
        
