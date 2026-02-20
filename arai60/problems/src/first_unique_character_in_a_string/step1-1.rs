/// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - HashSetが順序を持たないということ

  何を考えて解いていたか
  - 文字列スライスを前から探索して、charをHashSetに追加する
  - HashSet[0]を文字列スライスから探索して、インデックスを返す

  想定ユースケース
  -

  正解してから気づいたこと
  - HashSetには順序がないので、先頭を返すみたいな順序が関わってくる時に選ぶ解法として
まずはネガティブなイメージを持っていた方がいいかもしれない

  - 今回の問題であれば、charの重複ってイメージに引っ張られすぎているが、
文字の出現回数を数えて1のもので先頭のもののindexを返そう、がモチベーションなので
もっとシンプルに出現回数を数えよう、って考えが大切
返すだけで、シンプル

  - Arai60のHashの数問で、HashMapやHashSetを使ってメソッドをこねくり回すみたいな
手ぐせが悪い意味でついてしまっている
*/


// 途中まで自力で回答したコード
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut character_set = HashMap::<usize, char>::new();
        let mut unique_character_set = HashSet::new();

        for (index, character)in s.chars().enumerate() {
            character_set.insert(index, character);
        }

        for character in s.chars() {
            unique_character_set.insert(character);
        }

        let Some(first_unique_character) = character_set.get(&unique_character_set[0]) else {
            eprintln!("The string does not have a unique character in {}", s);
            return -1;
        };
        *first_unique_character as i32
    }
}

// Editorialのコードをgpt-5.3-codex mediumにRustで書かせた回答コード
use std::collections::HashMap;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut count = HashMap::<char, i32>::new();
        let chars: Vec<char> = s.chars().collect();

        for &c in &chars {
            *count.entry(c).or_insert(0) += 1;
        }

        for (i, &c) in chars.iter().enumerate() {
            if count.get(&c) == Some(&1) {
                return i as i32;
            }
        }

        -1
    }
}

