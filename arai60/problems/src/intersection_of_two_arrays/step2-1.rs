// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - ループでnums2の要素を取り出す時にn2って書いてるけど、実務コードではn2って命名に
何も意味を感じないので、前から読んでいると何に使うんだこれ、ってその時点で読みづらい
というか、もどかしいので意味を含めた命名にするべき

  他の人のコードを読んで考えたこと
  - (Iwasaさん)
  - 配列の型宣言の手法で、turbofish(::<HashSet<_>>)の存在を知らなかったので学びになった
  - スタイルの統一の観点で、turbofishで書くならその関数内ではturbofishで統一
したりすることも、今後は自分で一から書く時には懸念しないといけない

  他の想定ユースケース
  - 与えられる配列の要素の型がStringの時にどんなコードになるかを想定した
  - 所有権周りがまだ未熟なので、for式ら辺のコードが瞬時に思いつけなかったので


  改善する時にかんがえたこと
  - コード全体での思想というか、スタイルの統一
  - 今回のコードであれば配列の要素を取り出して何をしたいのかを明確にする
  - Vec<i32>に対してinto_iter(), iter()でどっちを使うか
　- nums1, nums2の所有権をintersection()の内部処理でムーブするかどうかとか
*/

// current version
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::new();

        for n2 in nums2 {
            if nums1_set.contains(&n2) {
                // HashSet::insert()で、既に要素が入って入ればfalse
                // 入っていなければ追加してtrueを返すので、intersectionの重複は
                // ここで回避できるようになっている
                intersection_set.insert(n2);
            }
        }

        intersection.into_iter().collect::<Vec<_>>()
    }
}

// incoming version
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        // ここで、nums1.iter()でもnums1_set: HashSet<&i32>になるだけで、コード上は問題ないが、
        // そのあとintersection_set.into_iter()を使っている
        // nums1をそのあと使うことがない
        // forでnums2はムーブしている
        // nums1_setする時に、値を直接containsで調べるか、参照から調べるかだと
        // 複雑さが幾分かマシになるので、今回はここでinto_iter()を使う方を採用した
        let nums1_set = nums1.into_iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::new();

        for element in nums2 {
            if nums1_set.contains(&element) {
                intersection_set.insert(element);
            }
        }

        // 戻り値の型がVec<i32>なのでinto_iter()
        intersection_set.into_iter().collect::<HashSet<_>>()
    }
}
