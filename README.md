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
        これを3回繰り返して、答えを送信して正解なったらStep1が終了
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
- [141. Linked List Cycle(Easy)](https://leetcode.com/problems/linked-list-cycle/description/)
- [142. Linked List Cycle II(Med.)](https://leetcode.com/problems/linked-list-cycle-ii/description/)
- [83. Remove Duplicates from Sorted List(Easy)](https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/)
- [82. Remove Duplicates from Sorted List II(Med.)](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/description/)
- [2. Add Two Numbers(Med.)](https://leetcode.com/problems/add-two-numbers/description/)

## Stack
- [20. Valid Parentheses(Easy)](https://leetcode.com/problems/valid-parentheses/description/)
- [206. Reverse Linked List(Easy)](https://leetcode.com/problems/reverse-linked-list/description/)

## Heap, PriorityQueue
- [703. Kth Largest Element in a Stream(Easy)](https://leetcode.com/problems/kth-largest-element-in-a-stream/description/)
- [347. Top K Frequent Elements(Med.)](https://leetcode.com/problems/top-k-frequent-elements/description/)
- [373. Find K Pairs with Smallest Sums(Med.)](https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/)

## HashMap
- [1. Two Sum(Easy)](https://leetcode.com/problems/two-sum/description/)
- [49. Group Anagrams(Med.)](https://leetcode.com/problems/group-anagrams/description/)
- [349. Intersection of Two Arrays(Easy)](https://leetcode.com/problems/intersection-of-two-arrays/description/)
- [929. Unique Email Addresses(Easy)](https://leetcode.com/problems/unique-email-addresses/description/)
- [387. First Unique Character in a String(Easy)](https://leetcode.com/problems/first-unique-character-in-a-string/description/)
- [560. Subarray Sum Equals K(Med.)](https://leetcode.com/problems/subarray-sum-equals-k/description/)

## Graph, BFS, DFS
- [200. Number of Islands(Med.)](https://leetcode.com/problems/number-of-islands/description/)
- [695. Max Area of Island(Med.)](https://leetcode.com/problems/max-area-of-island/description/)
- [323. Number of Connected Components in an Undirected Graph(Med.)](https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/description/)
- [127. Word Ladder(Hard)](https://leetcode.com/problems/word-ladder/description/)

## Tree, BT, BST
- [104. Maximum Depth of Binary Tree(Easy)](https://leetcode.com/problems/maximum-depth-of-binary-tree/description/)
- [111. Minimum Depth of Binary Tree(Easy)](https://leetcode.com/problems/minimum-depth-of-binary-tree/description/)
- [617. Merge Two Binary Trees(Easy)](https://leetcode.com/problems/merge-two-binary-trees/description/)
- [108. Convert Sorted Array to Binary Search Tree(Easy)](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/description/)
- [112. Path Sum(Easy)](https://leetcode.com/problems/path-sum/description/)
- [102. Binary Tree Level Order Traversal(Med.)](https://leetcode.com/problems/binary-tree-level-order-traversal/description/)
- [103. Binary Tree Zigzag Level Order Traversal(Med.)](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/description/)
- [98. Validate Binary Search Tree(Med.)](https://leetcode.com/problems/validate-binary-search-tree/description/)
- [105. Construct Binary Tree from Preorder and Inorder Traversal(Med.)](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/)

## Dynamic Programming
- [276. Paint Fence(Med.)](https://leetcode.com/problems/paint-fence/description/)
- [300. Longest Increasing Subsequence(Med.)](https://leetcode.com/problems/longest-increasing-subsequence/description/)
- [53. Maximum Subarray(Med.)](https://leetcode.com/problems/maximum-subarray/description/)
- [62. Unique Paths(Med.)](https://leetcode.com/problems/unique-paths/description/)
- [63. Unique Paths II(Med.)](https://leetcode.com/problems/unique-paths-ii/description/)
- [198. House Robber(Med.)](https://leetcode.com/problems/house-robber/description/)
- [213. House Robber II(Med.)](https://leetcode.com/problems/house-robber-ii/description/)
- [121. Best Time to Buy and Sell Stock(Easy)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/)
- [122. Best Time to Buy and Sell Stock II(Med.)](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/)
- [139. Word Break(Med.)](https://leetcode.com/problems/word-break/description/)
- [322. Coin Change(Med.)](https://leetcode.com/problems/coin-change/description/)

## Binary Search
- [35. Search Insert Position(Easy)](https://leetcode.com/problems/search-insert-position/description/)
- [153. Find Minimum in Rotated Sorted Array(Med.)](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/)
- [33. Search in Rotated Sorted Array(Med.)](https://leetcode.com/problems/search-in-rotated-sorted-array/description/)
- [1011. Capacity To Ship Packages Within D Days(Med.)](https://leetcode.com/problems/capacity-to-ship-packages-within-d-days/description/)

## Recursion
- [50. Pow(x, n)(Med.)](https://leetcode.com/problems/powx-n/description/)
- [779. K-th Symbol in Grammar(Med.)](https://leetcode.com/problems/k-th-symbol-in-grammar/description/)
- [776. Split BST(Med.)](https://leetcode.com/problems/split-bst/description/)

## Sliding Window
- [3. Longest Substring Without Repeating Characters(Med.)](https://leetcode.com/problems/longest-substring-without-repeating-characters/description/)
- [209. Minimum Size Subarray Sum(Med.)](https://leetcode.com/problems/minimum-size-subarray-sum/description/)

## Greedy + Backtracking
- [46. Permutations(Med.)](https://leetcode.com/problems/permutations/description/)
- [78. Subsets(Med.)](https://leetcode.com/problems/subsets/description/)
- [39. Combination Sum(Med.)](https://leetcode.com/problems/combination-sum/description/)
- [22. Generate Parentheses(Med.)](https://leetcode.com/problems/generate-parentheses/description/)

## その他
- [283. Move Zeroes(Easy)](https://leetcode.com/problems/move-zeroes/description/)
- [252. Meeting Rooms(Easy)](https://leetcode.com/problems/meeting-rooms/description/)
- [253. Meeting Rooms II(Med.)](https://leetcode.com/problems/meeting-rooms-ii/description/)
- [392. Is Subsequence(Med.)](https://leetcode.com/problems/is-subsequence/description/)
- [31. Next Permutation(Med.)](https://leetcode.com/problems/next-permutation/description/)
- [8. String to Integer (atoi)(Med.)](https://leetcode.com/problems/string-to-integer-atoi/description/)
- [6. Zigzag Conversion(Med.)](https://leetcode.com/problems/zigzag-conversion/description/)
