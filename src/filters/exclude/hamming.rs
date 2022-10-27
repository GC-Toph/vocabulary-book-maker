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

    pub fn dist(&mut self, a: &[u8], b: &[u8]) -> usize {
        let m = a.len();
        let n = b.len();
        assert!(m <= MAX_WORD_LENGTH && n <= MAX_WORD_LENGTH);

        for i in 1..m + 1 {
            for j in 1..n + 1 {
                unsafe {
                    let diff = match a.get_unchecked(i - 1) != b.get_unchecked(j - 1) {
                        true => 1,
                        false => 0,
                    };
                    *self.arr.get_unchecked_mut(i).get_unchecked_mut(j) = min(
                        min(
                            *self.arr.get_unchecked(i - 1).get_unchecked(j) + 1,
                            *self.arr.get_unchecked(i).get_unchecked(j - 1) + 1,
                        ),
                        *self.arr.get_unchecked(i - 1).get_unchecked(j - 1) + diff,
                    );
                }
            }
        }
        return self.arr[m][n];
    }
}
