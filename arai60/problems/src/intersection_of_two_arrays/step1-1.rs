/// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 解く時に考えていた方針をコードにできなかった
  - HashMapで使えるメソッドと、その所有権周りの仕様
  - HashSetの存在を知らなかった
  - 使った要素を配列にしまって、次の候補をそのusedの配列にないか確認するって考え
が思いつかなかった。

  何を考えて解いていたか
  - 一方の配列の要素を回して、もう一方の配列の要素と比較して、一致していた場合は
関数で返す配列にその値を追加する関数を作る
  - この工程を、0001-two-sumで習ったことを生かし、HashMapで実装してみる
  - 0001-two-sumでレビューしてもらった、パターン側で参照を解くか、式側で参照を解くかに注意する
  - 返す配列では値はuniqueなので、同じ値が追加されないように配慮する。具体的には、
配列の要素同士を比較したときに、等しかったらさらに関数の戻り値の配列にその要素が
無いことを確認する

  想定ユースケース
  -

  正解してから気づいたこと
  - HashSetの存在を知らないとなると、自分の開放以外に思いつけなかった
  - まだ所有権がおぼつかない。ゆっくり読めば処理できるが
  - なるほど、自分の方針だとHashMapを使う必要がないな。Arrayだけで処理できるので
*/

// 1回目で途中まで自力で書いたコード
use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let answer_vec: Vec<i32> = Vec::new();

        let element_map = HashMap::<i23, usize>::new();

        for (index, &element) in nums1.iter().enumerate() {
            element_map.insert(index, &element);
        }

        for (index, &element) in nums2.iter().enumerate() {
            let intersection = element_map.get(&element);

            match intersection {
                Some(&number) => {
                    if !answer_vec.contains(&number) {
                        answer_vec.push(number);
                    }
                },
                None => {},
            }
        }

        answer_vec
    }
}

//自分の方針での回答
use std::collections::HashMap;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut seen_number = HashMap::<i32, bool>::new();

        for x in nums1 {
            seen_number.insert(x, true);
        }

        let used_number = Vec::<i32>::new();
        let answer_set = Vec::<i32>::new();

        for x in nums2 {
            if seen_number.contains(&x) && !used_number.contains_key(&x) {
                answer_set.push(x);
                used_number.insert(x, true);
            }
        }

        answer_set
    }
}

// Iwasaさんのコードを回答として
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let mut intersection_set = HashSet::new();

        for n2 in nums2 {
            if nums1_set.contains(&n2) {
                intersection_set.insert(n2);
            }
        }
        intersection_set.into_iter().collect::<Vec<_>>()
    }

    pub fn intersection_two_sets(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let nums1_set = nums1.iter().collect::<HashSet<_>>();
        let nums2_set = nums2.iter().collect::<HashSet<_>>();

        let mut intersection = vec![];

        for n1 in nums1_set {
            if nums2_set.contains(n1) {
                interescrtion.push(*n1)
            }
        }
        intersection
    }
}
