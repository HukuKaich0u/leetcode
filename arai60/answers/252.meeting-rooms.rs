/*
この問題で鍛えること:
- アルゴリズム: interval 問題の sort 発想
- データ構造: ソート済み配列と sweep line
- Rust での練習ポイント: `sort_unstable_by_key`、`windows(2)`、イベント列の処理
- コード設計: 全比較から「局所チェックで十分」と言えるようになる感覚

問題の本質:
- 1 室で足りるかを知りたいので、どこかに overlap が 1 つでもあれば false
- 区間を開始時刻順に並べると、衝突は直前の会議の終了時刻だけ見ればよい
- つまり問題は「全区間の衝突判定」ではなく、「順序付けによって局所判定へ落とせるか」

解法候補:
1. 全ての会議ペアを比較する
   - 時間計算量: O(n^2)
   - 空間計算量: O(1)
   - 採用/非採用理由: 素朴だが、順序性を使えていない
2. 開始時刻で sort して隣接比較
   - 時間計算量: O(n log n)
   - 空間計算量: O(1) 追加なし
   - 採用/非採用理由: 判定問題として最短で、本命
3. sweep line
   - 時間計算量: O(n log n)
   - 空間計算量: O(n)
   - 採用/非採用理由: 会議室数に一般化しやすい

採用解法の説明:
- 会議を開始時刻順に並べる
- もし overlap があるなら、ある会議の開始時刻は直前会議の終了時刻より前になる
- 逆に、すべての隣接会議で `prev_end <= next_start` なら、それ以外の組でも衝突しない
- これで全比較を、ソート後の局所チェックへ落とせる

Rust観点の解説:
- `sort_unstable_by_key(|interval| interval[0])` で開始時刻順に並べる
- `windows(2)` を使うと、隣接会議の比較が簡潔に書ける
- sweep line 版では `(time, delta)` をソートして同時刻の扱いを明示できる

いいコード観点の解説:
- 本命実装は「なぜ隣だけ見ればよいか」が説明できて初めて意味がある
- sweep line 版は一般化の方向を示す比較材料として価値がある
- interval を `Vec<Vec<i32>>` のまま受けるので、意味のある変数名で補うことが重要

落とし穴:
- 同時刻 `end == start` を overlap と誤判定する
- sort 前提なのに、ソートせず局所比較してしまう
- sweep line 版で同時刻の終了と開始の優先順位を誤る
*/

struct Solution;
struct BruteForceSolution;
struct SweepLineSolution;
struct RefinedSolution;

impl BruteForceSolution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        for left in 0..intervals.len() {
            for right in left + 1..intervals.len() {
                if Self::overlap(&intervals[left], &intervals[right]) {
                    return false;
                }
            }
        }

        true
    }

    fn overlap(first: &[i32], second: &[i32]) -> bool {
        first[0] < second[1] && second[0] < first[1]
    }
}

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|interval| interval[0]);
        intervals
            .windows(2)
            .all(|pair| pair[0][1] <= pair[1][0])
    }
}

impl SweepLineSolution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        let mut events = Vec::with_capacity(intervals.len() * 2);

        for interval in intervals {
            events.push((interval[0], 1));
            events.push((interval[1], -1));
        }

        // 同時刻なら終了を先に処理し、[a, b] と [b, c] を重なりなしとみなす。
        events.sort_unstable_by_key(|&(time, delta)| (time, delta));

        let mut ongoing = 0;
        for (_, delta) in events {
            ongoing += delta;
            if ongoing > 1 {
                return false;
            }
        }

        true
    }
}

impl RefinedSolution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by_key(|interval| interval[0]);

        for pair in intervals.windows(2) {
            if Self::has_conflict(&pair[0], &pair[1]) {
                return false;
            }
        }

        true
    }

    fn has_conflict(previous: &[i32], current: &[i32]) -> bool {
        previous[1] > current[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all_solutions(intervals: Vec<Vec<i32>>, expected: bool) {
        assert_eq!(
            BruteForceSolution::can_attend_meetings(intervals.clone()),
            expected
        );
        assert_eq!(Solution::can_attend_meetings(intervals.clone()), expected);
        assert_eq!(
            SweepLineSolution::can_attend_meetings(intervals.clone()),
            expected
        );
        assert_eq!(RefinedSolution::can_attend_meetings(intervals), expected);
    }

    #[test]
    fn non_overlapping_intervals() {
        assert_all_solutions(vec![vec![0, 30], vec![35, 40]], true);
    }

    #[test]
    fn overlap_is_rejected() {
        assert_all_solutions(vec![vec![0, 30], vec![5, 10], vec![15, 20]], false);
    }

    #[test]
    fn touching_boundaries_are_allowed() {
        assert_all_solutions(vec![vec![5, 10], vec![10, 15]], true);
    }
}
