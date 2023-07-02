pub struct LazySegmentTree {
    n: usize,
    size: usize,
    log: usize,
    d: Vec<i32>,
    lz: Vec<i32>,
}

impl LazySegmentTree {
    pub fn new_with_size(n: usize) -> LazySegmentTree {
        LazySegmentTree::new_from_vec(vec![0; n])
    }

    pub fn new_from_vec(a: Vec<i32>) -> LazySegmentTree {
        let n = a.len();
        let log = (n as f32).log2().ceil() as usize;
        let size = 1 << log;

        let mut d = vec![0; 2 * size];
        let mut lz = vec![0; size];

        for i in 0..n {
            d[size + i] = a[i];
        }

        let mut st = LazySegmentTree {
            n,
            size,
            log,
            d,
            lz,
        };

        for i in (1..size).rev() {
            st.update(i);
        }

        return st;
    }

    pub fn get(&mut self, p: usize) -> i32 {
        assert!(p < self.n, "p must be less than n");

        let mut p = p;
        p += self.size;

        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }

        return self.d[p];
    }

    pub fn set(&mut self, p: usize, x: i32) {
        assert!(p < self.n, "p must be less than n");

        let mut p = p;
        p += self.size;

        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }

        self.d[p] = x;

        for i in 1..=self.log {
            self.update(p >> i);
        }
    }

    pub fn all_prod(&self) -> i32 {
        return self.d[1];
    }

    fn update(&mut self, k: usize) {
        self.d[k] = self.d[2 * k] + self.d[2 * k + 1];
    }

    fn all_apply(&mut self, k: usize, f: i32) {
        self.d[k] += f;
        if k < self.size {
            self.lz[k] += f;
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lz[k]);
        self.all_apply(2 * k + 1, self.lz[k]);

        self.lz[k] = 0;
    }
}
