// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - しないといけない処理の大枠は理解していたつもりだが、コードが書けず。
  - どのタイミングでvecに2つのインデックスをpushしていくのか。
  - HashMapを使うのか。
  - 同じインデックスをskipする処理

  何を考えて解いていたか
  - 与えらる配列を二重のforで回して、2つの要素の和がtargetになる時のループのインデックスを返す関数をつくる
  - 配列内の任意の2要素の和なので、二重ループのiとjは異なるって条件が必要
  - 解がちょうど1つって条件があるので、一個見つけたところでreturnする
  - 関数のシグニチャで所有権とライフタイムに注意する

  想定ユースケース
  -

  正解してから気づいたこと
  - 調べたこと->  pushにi,jを入れるためにu32 -> i32に変換しないといけず、その処理
  - Iwasaさんのコードで、HashMapが<usize, i32>じゃなく<i32, usize>であることに若干
の違和感があったけど、そのあとにget(&compliment)するために最初のハッシュマップ
の型を工夫してたのかなとか。
*/

// はじめに自分で書けたところまでのコード
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//
//         let mut answer: Vec<i32> = Vec::new();
//
//         for i in 0..nums.len() {
//             for j in (i+1)..nums.len() {
//
//                 if nums[i] + nums[j] == *target {
//                     answer.push(i)
//                 }
//
//             }
//         }
//     }
// }

// 自己流の回答
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut answer: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            for j in (i+1)..nums.len() {

                if nums[i] + nums[j] == target {
                    answer.push(i as i32);
                    answer.push(j as i32);
                    return answer;
                }
            }
        }

        answer
    }
}


// Iwasaさんのコードを回答として
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> i32 {
        let compliment_map = HashMap<i32, usize>::new();

        for (index, number) in nums.iter().enumerate() {
            let compliment = target - *number;
            match compliment_map.get(&compliment) {
                Some(compliment_index) => return vec![index as i32, *compliment_index as i32],
                None => compliment_map.insert(*number, index);
            }
        }
    }
    vec![]
}
