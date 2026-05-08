/*
この問題で鍛えること:
- アルゴリズム: line sweep / heap
- データ構造: min-heap
- Rust での練習ポイント: `BinaryHeap<Reverse<_>>`, sort, helper 抽象化
- コード設計: 「今何部屋使っているか」をどの状態で表現するか

問題の本質:
- 求めたいのは会議を全部配置する方法ではなく、同時進行数の最大値
- 新しい会議が始まる瞬間に必要なのは「いま一番早く空く部屋」の情報だけ
- したがって終了時刻の min-heap を持つのが自然

解法候補:
1. 全区間を愚直に比較する
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 比較用にはなるが遅い
2. start 昇順 + end min-heap
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 本命
3. start/end 分離の 2 pointer
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 別解として強い

採用解法の説明:
- 開始時刻順に会議を見る
- 各会議に対して、最も早く終わる部屋が再利用できるなら heap から取り除く
- その会議の終了時刻を heap へ入れる
- heap のサイズが、その時点で同時に使っている部屋数になる

Rust観点の解説:
- `BinaryHeap` は max-heap なので、`Reverse<i32>` を使って min-heap 化する
- 区間を `sort_unstable_by_key(|interval| interval[0])` で始点順に固定する
- helper 版では「部屋再利用判定」を関数へ切り出すと heap 操作の意図が見やすい

いいコード観点の解説:
- この問題の状態は「使用中の部屋数」ではなく「使用中の部屋の終了時刻集合」
- 本命実装は、必要な情報だけを残すデータ構造選択がそのまま答えになっている
- 2 pointer 別解を置くと、同じ O(n log n) でも表現が違うことが学べる

落とし穴:
- 1 つの会議開始時に再利用できる部屋を複数 pop する必要があると勘違いする
- `end <= start` のとき再利用可能であることを `end < start` と誤る
- heap の最大サイズではなく最後のサイズを答えにしてしまう
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;
struct NaiveSolution;
struct TwoPointerSolution;

impl NaiveSolution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut rooms: Vec<i32> = Vec::new();

        for interval in intervals {
            let mut placed = false;
            for end_time in &mut rooms {
                if *end_time <= interval[0] {
                    *end_time = interval[1];
                    placed = true;
                    break;
                }
            }
            if !placed {
                rooms.push(interval[1]);
            }
        }

        rooms.len() as i32
    }
}

impl Solution {
    pub fn min_meeting_rooms(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        intervals.sort_unstable_by_key(|interval| interval[0]);
        let mut heap = BinaryHeap::new();
        let mut answer = 0;

        for interval in intervals {
            if let Some(&Reverse(end_time)) = heap.peek() {
                if end_time <= interval[0] {
                    heap.pop();
                }
            }
            heap.push(Reverse(interval[1]));
            answer = answer.max(heap.len() as i32);
        }

        answer
    }
}

impl TwoPointerSolution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }

        let mut starts: Vec<_> = intervals.iter().map(|interval| interval[0]).collect();
        let mut ends: Vec<_> = intervals.iter().map(|interval| interval[1]).collect();
        starts.sort_unstable();
        ends.sort_unstable();

        let mut used = 0;
        let mut answer = 0;
        let mut end_index = 0;

        for start in starts {
            if start < ends[end_index] {
                used += 1;
                answer = answer.max(used);
            } else {
                end_index += 1;
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(intervals: Vec<Vec<i32>>, expected: i32) {
        assert_eq!(NaiveSolution::min_meeting_rooms(intervals.clone()), expected);
        assert_eq!(Solution::min_meeting_rooms(intervals.clone()), expected);
        assert_eq!(TwoPointerSolution::min_meeting_rooms(intervals), expected);
    }

    #[test]
    fn overlapping_meetings_need_two_rooms() {
        assert_all_solutions(vec![vec![0, 30], vec![5, 10], vec![15, 20]], 2);
    }

    #[test]
    fn sequential_meetings_reuse_one_room() {
        assert_all_solutions(vec![vec![7, 10], vec![2, 4], vec![4, 7]], 1);
    }

    #[test]
    fn dense_overlap_case() {
        assert_all_solutions(vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8]], 4);
    }
}
