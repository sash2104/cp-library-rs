pub struct BinarySearch {
    ng: i64,
    ok: i64,
}

impl BinarySearch {
    pub fn new(ng: i64, ok: i64) -> Self {
        Self { ng, ok }
    }
    /// f が true を返す境界値を探す
    pub fn search<F>(&self, check: F) -> i64
    where
        F: Fn(i64) -> bool,
    {
        let mut ng = self.ng;
        let mut ok = self.ok;

        while (ok-ng).abs() > 1 {
            let mid = (ok + ng) / 2;

            if check(mid) {
                ok = mid;
            } else {
                ng = mid;
            };
        }
        ok
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn test_binary_search() {
        let s = [0, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let bs = BinarySearch::new(s.len() as i64, -1);
        assert_eq!(bs.search(|mid| s[mid as usize] < 56), s.len() as i64 -1);
        assert_eq!(bs.search(|mid| s[mid as usize] < 10), 5);
        assert_eq!(bs.search(|mid| s[mid as usize] < 2), 1);
        assert_eq!(bs.search(|mid| s[mid as usize] < 0), -1);
    }

    #[test]
    fn test_binary_search_rev() {
        let s = [0, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        let bs = BinarySearch::new(-1, s.len() as i64);
        assert_eq!(bs.search(|mid| s[mid as usize] > 55), s.len() as i64);
        assert_eq!(bs.search(|mid| s[mid as usize] > 10), 6);
        assert_eq!(bs.search(|mid| s[mid as usize] > 2), 3);
        assert_eq!(bs.search(|mid| s[mid as usize] > -1), 0);
    }
}