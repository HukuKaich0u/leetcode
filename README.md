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
        1. 答えを見ずに5分考える。5分考えて分からなかったら答えを見る
        2. 答えを見て理解したと思ったら、答えを隠して書く
           筆が進まず5分迷ったら答えを見る
        3. 見てしまったら、一回全部消してやり直し
        これを繰り返して、答えを送信して正解なったらStep1が終了
    - Step2:
        1. コードが見やすくなるように、できるだけ整える
        2. 自分のコードに尽きそうなコメントの予測をする
        3. 動くコードになったらStep2が終了
    - Step3:
        1. 全部消す
        2. 今度は時間を計りながらもう1回書く
        3. 書いてAcceptされたら、文字を消してもう1回書く
        これを10分以内に1回もエラーを出さずに書ける状態になるまで続ける
        3回続けてそれができたらその問題はひとまず丸
    - Step4:
        1. 過去に同じ問題を解いている人のコードレビューを読む

    3ステップ目まで終わったタイミングで講師陣にレビューを依頼し、レビューを元にコードを書き直す
    LeetCodeの問題とは直接関係ないが、関連するライブラリの再実装などをするのもよい
    たとえば、@lru_cacheやheapqライブラリなどを使用した会報があったので、
    LRU Cacheの実装やBinary Heapの実装なども行う

# 参考リンク
## すでにRustで実装しているリポジトリ
- https://github.com/t9a-dev/LeetCode_arai60
- https://github.com/Yoshiki-Iwasa/Arai60

# Arai60
## LinkedList
- [141. Linked List Cycle](https://github.com/hayashi-ay/leetcode/pull/15)
- [142. Linked List Cycle II](https://github.com/hayashi-ay/leetcode/pull/18)
- [83. Remove Duplicates from Sorted List](https://github.com/hayashi-ay/leetcode/pull/20)
- [82. Remove Duplicates from Sorted List II](https://github.com/hayashi-ay/leetcode/pull/23)
- [2. Add Two Numbers](https://github.com/hayashi-ay/leetcode/pull/24)

## Stack
- [20. Valid Parentheses](https://github.com/hayashi-ay/leetcode/pull/16)
- [206. Reverse Linked List](https://github.com/hayashi-ay/leetcode/pull/13)

## Heap, PriorityQueue
- [703. Kth Largest Element in a Stream](https://github.com/hayashi-ay/leetcode/pull/54)
- [347. Top K Frequent Elements](https://github.com/hayashi-ay/leetcode/pull/60)
- [373. Find K Pairs with Smallest Sums](https://github.com/hayashi-ay/leetcode/pull/66)

## HashMap
- [1. Two Sum](https://github.com/hayashi-ay/leetcode/pull/14)
- [49. Group Anagrams](https://github.com/hayashi-ay/leetcode/pull/19)
- [349. Intersection of Two Arrays](https://github.com/hayashi-ay/leetcode/pull/21)
- [929. Unique Email Addresses](https://github.com/hayashi-ay/leetcode/pull/25)
- [387. First Unique Character in a String](https://github.com/hayashi-ay/leetcode/pull/28)
- [560. Subarray Sum Equals K](https://github.com/hayashi-ay/leetcode/pull/31)

## Graph, BFS, DFS
- [200. Number of Islands](https://github.com/hayashi-ay/leetcode/pull/33)
- [695. Max Area of Island](https://github.com/hayashi-ay/leetcode/pull/34)
- [323. Number of Connected Components in an Undirected Graph](https://github.com/hayashi-ay/leetcode/pull/37)
- [127. Word Ladder](https://github.com/hayashi-ay/leetcode/pull/42)

## Tree, BT, BST
- [104. Maximum Depth of Binary Tree](https://github.com/hayashi-ay/leetcode/pull/22)
- [111. Minimum Depth of Binary Tree](https://github.com/hayashi-ay/leetcode/pull/26)
- [617. Merge Two Binary Trees](https://github.com/hayashi-ay/leetcode/pull/12)
- [108. Convert Sorted Array to Binary Search Tree](https://github.com/hayashi-ay/leetcode/pull/29)
- [112. Path Sum](https://github.com/hayashi-ay/leetcode/pull/30)
- [102. Binary Tree Level Order Traversal](https://github.com/hayashi-ay/leetcode/pull/32)
- [103. Binary Tree Zigzag Level Order Traversal](https://github.com/hayashi-ay/leetcode/pull/35)
- [98. Validate Binary Search Tree](https://github.com/hayashi-ay/leetcode/pull/38)
- [105. Construct Binary Tree from Preorder and Inorder Traversal](https://github.com/hayashi-ay/leetcode/pull/43)

## Dynamic Programming
- [276. Paint Fence](https://github.com/hayashi-ay/leetcode/pull/17)
- [300. Longest Increasing Subsequence](https://github.com/hayashi-ay/leetcode/pull/27)
- [53. Maximum Subarray](https://github.com/hayashi-ay/leetcode/pull/36)
- [62. Unique Paths](https://github.com/hayashi-ay/leetcode/pull/39)
- [63. Unique Paths II](https://github.com/hayashi-ay/leetcode/pull/44)
- [198. House Robber](https://github.com/hayashi-ay/leetcode/pull/48)
- [213. House Robber II](https://github.com/hayashi-ay/leetcode/pull/50)
- [121. Best Time to Buy and Sell Stock](https://github.com/hayashi-ay/leetcode/pull/52)
- [122. Best Time to Buy and Sell Stock II](https://github.com/hayashi-ay/leetcode/pull/56)
- [139. Word Break](https://github.com/hayashi-ay/leetcode/pull/61)
- [322. Coin Change](https://github.com/hayashi-ay/leetcode/pull/68)

## Binary Search
- [35. Search Insert Position](https://github.com/hayashi-ay/leetcode/pull/40)
- [153. Find Minimum in Rotated Sorted Array](https://github.com/hayashi-ay/leetcode/pull/45)
- [33. Search in Rotated Sorted Array](https://github.com/hayashi-ay/leetcode/pull/49)
- [1011. Capacity To Ship Packages Within D Days](https://github.com/hayashi-ay/leetcode/pull/55)

## Recursion
- [50. Pow(x, n)](https://github.com/hayashi-ay/leetcode/pull/41)
- [779. K-th Symbol in Grammar](https://github.com/hayashi-ay/leetcode/pull/46)
- [776. Split BST](https://github.com/hayashi-ay/leetcode/pull/53)

## Sliding Window
- [3. Longest Substring Without Repeating Characters](https://github.com/hayashi-ay/leetcode/pull/47)
- [209. Minimum Size Subarray Sum](https://github.com/hayashi-ay/leetcode/pull/51)

## Greedy + Backtracking
- [46. Permutations](https://github.com/hayashi-ay/leetcode/pull/57)
- [78. Subsets](https://github.com/hayashi-ay/leetcode/pull/63)
- [39. Combination Sum](https://github.com/hayashi-ay/leetcode/pull/65)
- [22. Generate Parentheses](https://github.com/hayashi-ay/leetcode/pull/70)

## その他
- [283. Move Zeroes](https://github.com/hayashi-ay/leetcode/pull/58)
- [252. Meeting Rooms](https://github.com/hayashi-ay/leetcode/pull/59)
- [253. Meeting Rooms II](https://github.com/hayashi-ay/leetcode/pull/62)
- [392. Is Subsequence](https://github.com/hayashi-ay/leetcode/pull/64)
- [31. Next Permutation](https://github.com/hayashi-ay/leetcode/pull/67)
- [8. String to Integer (atoi)](https://github.com/hayashi-ay/leetcode/pull/69)
- [6. Zigzag Conversion](https://github.com/hayashi-ay/leetcode/pull/71)
