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
        let lz = vec![0; size];

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

    pub fn get_range(&mut self, l: usize, r: usize) -> i32 {
        assert!(l <= r, "l must be less than or equal to r");
        assert!(r <= self.n, "r must be less than or equal to n");

        if l == r {
            return 1;
        }

        let mut l = l;
        let mut r = r;

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push(r >> i);
            }
        }

        let mut sml = 1;
        let mut smr = 1;

        while l < r {
            if l & 1 != 0 {
                sml = sml * self.d[l];
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = self.d[r] * smr;
            }

            l >>= 1;
            r >>= 1;
        }

        return sml * smr;
    }

    pub fn get_all(&self) -> i32 {
        return self.d[1];
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

    pub fn set_range(&mut self, l: usize, r: usize) {
        assert!(l <= r, "l must be less than or equal to r");
        assert!(r <= self.n, "r must be less than or equal to n");

        if l == r {
            return;
        }

        let mut l = l;
        let mut r = r;

        l += self.size;
        r += self.size;

        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }

        {
            let mut l2 = l;
            let mut r2 = r;

            while l2 < r2 {
                if l2 & 1 != 0 {
                    self.all_apply(l2, 1);
                    l2 += 1;
                }
                if r2 & 1 != 0 {
                    r2 -= 1;
                    self.all_apply(r2, 1);
                }

                l2 >>= 1;
                r2 >>= 1;
            }
        }

        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.update(l >> i);
            }
            if ((r >> i) << i) != r {
                self.update((r - 1) >> i);
            }
        }
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
