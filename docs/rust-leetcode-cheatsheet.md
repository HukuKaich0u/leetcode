# Rust LeetCode Cheat Sheet

Rust で LeetCode を解くときに、よく使う `Vec` / `HashMap` / `HashSet` / `BinaryHeap` / `VecDeque` / `BTreeMap` / `String` / `str` 系のメソッドをまとめる。

対象は「典型処理で即使うもの」だけ。標準ライブラリの完全な網羅ではなく、手を動かすときの実用優先。

## 先に結論

- 文字列が `lowercase English letters` みたいな制約なら、`chars()` より `bytes()` / `as_bytes()` の方が単純
- `String` は index できないので、必要なら `Vec<char>` か `&[u8]` に落とす
- 個数カウントは `HashMap::entry(...).or_insert(...)`
- 集合 membership は `HashSet::contains(...)`
- `split_once`, `split`, `trim`, `replace`, `push`, `pop` は文字列問題でかなり使う

## よく使う import

```rust
use std::collections::{HashMap, HashSet};
```

必要に応じて:

```rust
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, VecDeque};
```

## `Vec<T>`

LeetCode では配列・stack・DP テーブル・可変長バッファとして最頻出。

### 基本メソッド

```rust
Vec::<T>::new() -> Vec<T>
```

```rust
Vec::<T>::with_capacity(capacity: usize) -> Vec<T>
```

```rust
vec.len() -> usize
vec.is_empty() -> bool
```

```rust
vec.push(value: T) -> ()
vec.pop() -> Option<T>
```

```rust
vec.first() -> Option<&T>
vec.last() -> Option<&T>
```

```rust
vec.get(index: usize) -> Option<&T>
vec.get_mut(index: usize) -> Option<&mut T>
```

```rust
vec.swap(i: usize, j: usize) -> ()
```

```rust
vec.reverse() -> ()
```

```rust
vec.sort() -> ()
vec.sort_unstable() -> ()
```

```rust
vec.binary_search(x: &T) -> Result<usize, usize>
```

```rust
vec.insert(index: usize, element: T) -> ()
vec.remove(index: usize) -> T
```

### 典型パターン

#### 1. stack

```rust
let mut stack: Vec<char> = Vec::new();
stack.push('(');
let top: Option<char> = stack.pop();
```

#### 2. `Vec<Vec<T>>` の初期化

```rust
let rows: usize = 3;
let cols: usize = 4;
let grid: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
```

#### 3. two pointers

```rust
let mut left: usize = 0;
let mut right: usize = nums.len() - 1;

while left < right {
    // ...
}
```

#### 4. backtracking

```rust
let mut path: Vec<i32> = Vec::new();
path.push(x);
path.pop();
```

#### 5. 並び替え

```rust
nums.sort_unstable();
```

使い分け:

- `sort()`: 安定ソート
- `sort_unstable()`: 追加メモリが少なく、競プロではこちらを使うことが多い

#### 6. 部分列を順に見る

```rust
for window in nums.windows(2) {
    // window: &[i32]
}
```

## `HashMap<K, V>`

### 基本メソッド

```rust
HashMap::<K, V>::new() -> HashMap<K, V>
```

- 空の `HashMap` を作る

```rust
map.insert(k: K, v: V) -> Option<V>
```

- `k` に `v` を入れる
- すでに値があれば古い値を `Some(old_v)` で返す

```rust
map.get<Q>(&self, key: &Q) -> Option<&V>
```

- 読み取り
- `HashMap<i32, usize>` なら `Option<&usize>`

```rust
map.get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
```

- 可変参照で取りたいときに使う

```rust
map.contains_key<Q>(&self, key: &Q) -> bool
```

- key があるかだけ見たいとき

```rust
map.remove<Q>(&mut self, key: &Q) -> Option<V>
```

- key を消して値を返す

```rust
map.entry(key: K) -> Entry<K, V>
```

- 「なければ作る」を 1 回で書く

### 典型パターン

#### 1. 出現回数カウント

```rust
let mut count: HashMap<i32, i32> = HashMap::new();

for x in nums {
    *count.entry(x).or_insert(0) += 1;
}
```

関連メソッド:

```rust
entry.or_insert(default: V) -> &mut V
entry.or_insert_with(f: FnOnce() -> V) -> &mut V
```

#### 2. 逆引きテーブル

```rust
let mut seen: HashMap<i32, usize> = HashMap::new();

for (i, &x) in nums.iter().enumerate() {
    let need = target - x;
    if let Some(&j) = seen.get(&need) {
        return vec![j as i32, i as i32];
    }
    seen.insert(x, i);
}
```

ポイント:

- `get` は `Option<&usize>` を返す
- `if let Some(&j)` と書くと参照を外して `usize` で受けられる

#### 3. 値を更新する

```rust
if let Some(v) = map.get_mut(&key) {
    *v += 1;
}
```

#### 4. key / value をなめる

```rust
map.keys() -> Keys<'_, K, V>
map.values() -> Values<'_, K, V>
map.iter() -> Iter<'_, K, V>           // Item = (&K, &V)
map.into_iter() -> IntoIter<K, V>      // Item = (K, V)
```

例:

```rust
for (&num, &freq) in count.iter() {
    // ...
}
```

## `HashSet<T>`

### 基本メソッド

```rust
HashSet::<T>::new() -> HashSet<T>
```

```rust
set.insert(value: T) -> bool
```

- 新しく入ったら `true`
- すでにあれば `false`

```rust
set.contains<Q>(&self, value: &Q) -> bool
```

```rust
set.remove<Q>(&mut self, value: &Q) -> bool
```

```rust
set.get<Q>(&self, value: &Q) -> Option<&T>
```

```rust
set.iter() -> Iter<'_, T>
```

### 典型パターン

#### 1. membership 判定

```rust
let dict: HashSet<String> = word_list.into_iter().collect();

if dict.contains(&end_word) {
    // ...
}
```

#### 2. 重複除去

```rust
let set: HashSet<i32> = nums.into_iter().collect();
let unique: Vec<i32> = set.into_iter().collect();
```

#### 3. sliding window

```rust
let mut seen: HashSet<char> = HashSet::new();

while seen.contains(&chars[right]) {
    seen.remove(&chars[left]);
    left += 1;
}
seen.insert(chars[right]);
```

#### 4. 集合演算

```rust
set1.intersection(&set2) -> Intersection<'_, T, S>
set1.union(&set2) -> Union<'_, T, S>
set1.difference(&set2) -> Difference<'_, T, S>
```

例:

```rust
let ans: Vec<i32> = set1.intersection(&set2).copied().collect();
```

## `BTreeMap<K, V>`

key 順に持ちたいときに使う。`HashMap` と違って順序が安定する。

### 基本メソッド

```rust
BTreeMap::<K, V>::new() -> BTreeMap<K, V>
```

```rust
map.insert(k: K, v: V) -> Option<V>
map.get<Q>(&self, key: &Q) -> Option<&V>
map.get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
map.remove<Q>(&mut self, key: &Q) -> Option<V>
map.contains_key<Q>(&self, key: &Q) -> bool
map.entry(key: K) -> Entry<'_, K, V>
```

```rust
map.first_key_value() -> Option<(&K, &V)>
map.last_key_value() -> Option<(&K, &V)>
```

```rust
map.range<R>(&self, range: R) -> Range<'_, K, V>
```

### 典型パターン

#### 1. key 順に走査

```rust
let mut map: BTreeMap<i32, i32> = BTreeMap::new();
map.insert(3, 30);
map.insert(1, 10);

for (&k, &v) in map.iter() {
    // k は昇順
}
```

#### 2. 座標圧縮や sweep line 用の差分管理

```rust
*map.entry(left).or_insert(0) += 1;
*map.entry(right).or_insert(0) -= 1;
```

#### 3. 最小 key / 最大 key を見る

```rust
let smallest: Option<(&i32, &i32)> = map.first_key_value();
let largest: Option<(&i32, &i32)> = map.last_key_value();
```

## `BinaryHeap<T>`

Rust の `BinaryHeap` はデフォルトで max-heap。

### 基本メソッド

```rust
BinaryHeap::<T>::new() -> BinaryHeap<T>
```

```rust
BinaryHeap::<T>::with_capacity(capacity: usize) -> BinaryHeap<T>
```

```rust
heap.push(item: T) -> ()
heap.pop() -> Option<T>
heap.peek() -> Option<&T>
```

```rust
heap.len() -> usize
heap.is_empty() -> bool
```

### 典型パターン

#### 1. max-heap

```rust
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(3);
heap.push(10);
let top: Option<&i32> = heap.peek();
let popped: Option<i32> = heap.pop();
```

#### 2. min-heap

```rust
let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
heap.push(Reverse(3));
heap.push(Reverse(10));

let smallest: Option<i32> = heap.pop().map(|Reverse(x)| x);
```

#### 3. `(priority, value)` を入れる

```rust
let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
heap.push((freq, value));
```

#### 4. 上位 `k` 件を保つ

```rust
let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

for x in nums {
    heap.push(Reverse(x));
    if heap.len() > k {
        heap.pop();
    }
}
```

## `VecDeque<T>`

両端 push / pop が `O(1)`。BFS の queue で定番。

### 基本メソッド

```rust
VecDeque::<T>::new() -> VecDeque<T>
```

```rust
queue.push_back(value: T) -> ()
queue.push_front(value: T) -> ()
```

```rust
queue.pop_back() -> Option<T>
queue.pop_front() -> Option<T>
```

```rust
queue.front() -> Option<&T>
queue.back() -> Option<&T>
```

```rust
queue.len() -> usize
queue.is_empty() -> bool
```

### 典型パターン

#### 1. BFS queue

```rust
let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
queue.push_back((0, 0));

while let Some((r, c)) = queue.pop_front() {
    // ...
}
```

#### 2. level-order traversal

```rust
let level_size: usize = queue.len();
for _ in 0..level_size {
    let node = queue.pop_front().unwrap();
    // ...
}
```

#### 3. monotonic queue の土台

```rust
while let Some(&back) = deque.back() {
    if back < x {
        deque.pop_back();
    } else {
        break;
    }
}
deque.push_back(x);
```

## `String`

### 基本メソッド

```rust
String::new() -> String
```

```rust
String::with_capacity(capacity: usize) -> String
```

```rust
s.len() -> usize
```

- 文字数ではなく byte 数

```rust
s.is_empty() -> bool
```

```rust
s.push(ch: char) -> ()
```

```rust
s.push_str(string: &str) -> ()
```

```rust
s.pop() -> Option<char>
```

```rust
s.clear() -> ()
```

```rust
s.as_str() -> &str
```

```rust
s.as_bytes() -> &[u8]
```

```rust
s.into_bytes() -> Vec<u8>
```

### 典型パターン

#### 1. backtracking

```rust
let mut path = String::new();

path.push('(');
path.pop();
```

#### 2. 行ごとに貯める

```rust
let mut rows = vec![String::new(); num_rows];
rows[row].push(ch);
let answer = rows.concat();
```

#### 3. byte 配列へ変換

```rust
let bytes: &[u8] = s.as_bytes();
let owned: Vec<u8> = s.into_bytes();
```

使い分け:

- `as_bytes()`: 借用だけ
- `into_bytes()`: `String` を消費して所有権ごと `Vec<u8>` にする

## `str`

`String` を借用した view。LeetCode では「分割」「前後の空白処理」「部分一致」でよく使う。

### 基本メソッド

```rust
s.trim() -> &str
```

```rust
s.split(delimiter: char) -> Split<'_, char>
```

```rust
s.split(delimiter: &str) -> Split<'_, &str>
```

```rust
s.split_once(delimiter: char) -> Option<(&str, &str)>
```

```rust
s.split_once(delimiter: &str) -> Option<(&str, &str)>
```

```rust
s.starts_with(pattern: &str) -> bool
```

```rust
s.ends_with(pattern: &str) -> bool
```

```rust
s.contains(pattern: &str) -> bool
```

```rust
s.replace(from: &str, to: &str) -> String
```

```rust
s.chars() -> Chars<'_>
```

```rust
s.bytes() -> Bytes<'_>
```

### 典型パターン

#### 1. メールアドレス正規化

```rust
let (local, domain): (&str, &str) = email.split_once('@').unwrap();
let local: String = local.split('+').next().unwrap().replace('.', "");
let normalized: String = format!("{local}@{domain}");
```

#### 2. 前後の空白を飛ばす

```rust
let trimmed: &str = s.trim();
```

#### 3. 文字列分割

```rust
for token in s.split(',') {
    // token: &str
}
```

#### 4. 括弧を何度も消す

```rust
let reduced: String = s
    .replace("()", "")
    .replace("[]", "")
    .replace("{}", "");
```

## `chars()`, `bytes()`, `as_bytes()`

### 型

```rust
s.chars() -> Chars<'_>       // Item = char
s.bytes() -> Bytes<'_>       // Item = u8
s.as_bytes() -> &[u8]
```

### 使い分け

- Unicode の文字単位で見たい: `chars()`
- ASCII 前提で速く単純に書きたい: `bytes()` / `as_bytes()`
- index 付きで配列っぽく扱いたい: `as_bytes()` または `Vec<char>`

### 典型パターン

#### 1. index と一緒に走査

```rust
for (i, ch) in s.chars().enumerate() {
    // i: usize, ch: char
}
```

```rust
for (i, b) in s.bytes().enumerate() {
    // i: usize, b: u8
}
```

#### 2. byte slice で DP / two pointers

```rust
let bytes: &[u8] = s.as_bytes();
let mut left: usize = 0;
let mut right: usize = bytes.len();
```

#### 3. `Vec<char>` 化して index したい

```rust
let chars: Vec<char> = s.chars().collect();
let first: char = chars[0];
```

注意:

- `s[i]` はできない
- `chars().nth(i)` は毎回先頭から進むので、繰り返し使うと重い

## `char`

### よく使うメソッド

```rust
ch.is_ascii_digit() -> bool
ch.is_ascii_lowercase() -> bool
ch.is_ascii_uppercase() -> bool
ch.to_digit(radix: u32) -> Option<u32>
ch.to_ascii_lowercase() -> char
ch.to_ascii_uppercase() -> char
```

### 典型パターン

```rust
if ch.is_ascii_digit() {
    let digit: u32 = ch.to_digit(10).unwrap();
}
```

## `Vec<u8>` / `&[u8]`

文字列問題でかなり強い。ASCII 前提ならこれが最も書きやすいことが多い。

### よく使うメソッド

```rust
bytes.len() -> usize
bytes.is_empty() -> bool
bytes[i] -> u8
bytes.get(i: usize) -> Option<&u8>
bytes.windows(size: usize) -> Windows<'_, u8>
```

### 典型パターン

#### 1. 小文字英字の頻度

```rust
let mut freq = [0; 26];
for b in s.bytes() {
    freq[(b - b'a') as usize] += 1;
}
```

#### 2. anagram key

```rust
let mut count = [0u8; 26];
for b in word.bytes() {
    count[(b - b'a') as usize] += 1;
}
```

#### 3. 部分列判定

```rust
let s = s.as_bytes();
let t = t.as_bytes();

let mut i = 0;
for &b in t {
    if i < s.len() && s[i] == b {
        i += 1;
    }
}
```

## `Iterator` と組み合わせてよく使うもの

### collect

```rust
iter.collect::<Vec<_>>() -> Vec<_>
iter.collect::<HashSet<_>>() -> HashSet<_>
iter.collect::<HashMap<_, _>>() -> HashMap<_, _>
iter.collect::<String>() -> String
```

例:

```rust
let chars: Vec<char> = s.chars().collect();
let set: HashSet<i32> = nums.into_iter().collect();
let map: HashMap<i32, usize> = nums.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
```

### enumerate

```rust
iter.enumerate() -> Enumerate<I>   // Item = (usize, I::Item)
```

### map

```rust
iter.map(f) -> Map<I, F>
```

例:

```rust
let words: Vec<String> = slices.iter().map(|s| s.to_string()).collect();
```

### filter

```rust
iter.filter(predicate) -> Filter<I, P>
```

### any / all

```rust
iter.any(predicate) -> bool
iter.all(predicate) -> bool
```

## LeetCode での頻出レシピ

### 1. 頻度カウント

```rust
let mut count: HashMap<i32, i32> = HashMap::new();
for x in nums {
    *count.entry(x).or_insert(0) += 1;
}
```

### 2. 文字の頻度カウント

```rust
let mut count = [0; 26];
for b in s.bytes() {
    count[(b - b'a') as usize] += 1;
}
```

### 3. 文字列を安全に index したい

```rust
let chars: Vec<char> = s.chars().collect();
let bytes: &[u8] = s.as_bytes();
```

使い分け:

- 問題が Unicode を意識する: `Vec<char>`
- 問題が英小文字や数字だけ: `&[u8]`

### 4. sliding window

```rust
let chars: Vec<char> = s.chars().collect();
let mut seen: HashSet<char> = HashSet::new();
let mut left = 0;

for right in 0..chars.len() {
    while seen.contains(&chars[right]) {
        seen.remove(&chars[left]);
        left += 1;
    }
    seen.insert(chars[right]);
}
```

### 5. split して parse

```rust
let nums: Vec<i32> = s
    .split(',')
    .map(|x| x.parse::<i32>().unwrap())
    .collect();
```

`parse` の型:

```rust
str::parse::<T>(&self) -> Result<T, T::Err>
```

## 迷ったときの指針

- key で引く: `HashMap`
- あるかどうかだけ欲しい: `HashSet`
- 文字列を削る / 分ける / 置換する: `str`
- 1 文字ずつ意味がある: `chars()`
- 英数字だけで index したい: `bytes()` / `as_bytes()`
- 再帰で 1 本の文字列を育てる: `String::push` / `String::pop`
