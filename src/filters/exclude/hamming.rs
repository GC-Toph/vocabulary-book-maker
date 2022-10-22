use std::cmp::min;

const MAX_WORD_LENGTH: usize = 20;
pub struct Hamming {
    arr: [[usize; MAX_WORD_LENGTH + 1]; MAX_WORD_LENGTH + 1],
}
impl Hamming {
    pub fn new() -> Hamming {
        let mut ret = Hamming {
            arr: [[0; MAX_WORD_LENGTH + 1]; MAX_WORD_LENGTH + 1],
        };
        for i in 0..MAX_WORD_LENGTH + 1 {
            ret.arr[i][0] = i;
            ret.arr[0][i] = i;
        }
        return ret;
    }

    pub fn dist(&mut self, a: &str, b: &str) -> usize {
        assert!(a.len() <= MAX_WORD_LENGTH && b.len() <= MAX_WORD_LENGTH);

        let m = a.len();
        let n = b.len();
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();

        for i in 1..m + 1 {
            for j in 1..n + 1 {
                let diff = match a[i - 1] != b[j - 1] {
                    true => 1,
                    false => 0,
                };
                self.arr[i][j] = min(
                    min(self.arr[i - 1][j] + 1, self.arr[i][j - 1] + 1),
                    self.arr[i - 1][j - 1] + diff,
                );
            }
        }
        return self.arr[m][n];
    }
}
