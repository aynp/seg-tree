pub struct SegmentTree {
    n: usize,
    size: usize,
    log: usize,
    d: Vec<i32>,
}

impl SegmentTree {
    pub fn new_with_size(n: usize) -> SegmentTree {
        let log = (n as f32).log2().ceil() as usize;
        let size = 1 << log;

        let d = vec![0; 2 * size];

        SegmentTree { n, size, log, d }
    }

    pub fn new_from_vec(a: Vec<i32>) -> SegmentTree {
        let n = a.len();
        let log = (n as f32).log2().ceil() as usize;
        let size = 1 << log;
        let mut d = vec![0; 2 * size];

        for i in 0..n {
            d[size + i] = a[i];
        }

        let mut st = SegmentTree { n, size, log, d };

        for i in (1..size).rev() {
            st.update(i);
        }

        return st;
    }

    pub fn get(&self, p: usize) -> i32 {
        assert!(p < self.n, "p must be less than n");

        return self.d[self.n + p];
    }

    pub fn get_range(&self, l: usize, r: usize) -> i32 {
        assert!(l <= r, "l must be less than or equal to r");
        assert!(r <= self.n, "r must be less than or equal to n");

        let mut sml = 0;
        let mut smr = 0;

        let mut l = l;
        let mut r = r;

        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml += self.d[l];
                l += 1;
            }

            if r & 1 != 0 {
                r -= 1;
                smr += self.d[r];
            }

            l >>= 1;
            r >>= 1;
        }

        return sml + smr;
    }

    pub fn set(&mut self, mut p: usize, x: i32) {
        assert!(p < self.n, "p must be less than n");

        p += self.size;
        self.d[p] = x;

        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn get_all(&self) -> i32 {
        return self.d[1];
    }

    pub fn update(&mut self, k: usize) {
        self.d[k] = self.d[2 * k] + self.d[2 * k + 1];
    }
}
