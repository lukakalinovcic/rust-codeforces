use std::cmp::min;

pub trait LazySegTreeSpec<T, U> {
    fn identity(&self) -> T;
    fn op(&self, a: &T, b: &T) -> T;
    fn compose(&self, a: &U, b: &U) -> U;
    fn update(&self, t: &T, u: &U) -> T;
}

#[derive(Clone, Debug)]
pub struct LazySegTree<T, U, Spec> {
    size: usize,
    a: Vec<T>,
    b: Vec<Option<U>>,
    spec: Spec,
}
impl<T: Clone, U: Clone, Spec: LazySegTreeSpec<T, U>> LazySegTree<T, U, Spec> {
    pub fn new(spec: Spec, size: usize) -> Self {
        let tsz = if size == 0 {
            0
        } else {
            size.next_power_of_two()
        };
        Self {
            size,
            a: vec![spec.identity(); tsz * 2],
            b: vec![None; tsz],
            spec,
        }
    }

    fn _tsize(&self) -> usize {
        self.b.len()
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn spec(&self) -> &Spec {
        &self.spec
    }

    pub fn init<F: FnMut(usize) -> T>(&mut self, mut gen: F) {
        let tsz = self._tsize();
        for i in 0..self.size {
            self.a[i + tsz] = gen(i);
        }
        for i in (1..tsz).rev() {
            self.a[i] = self.spec.op(&self.a[i << 1], &self.a[i << 1 | 1])
        }
    }

    fn _calc(&mut self, i: usize) {
        self.a[i] = self.spec.op(&self.a[i << 1], &self.a[i << 1 | 1]);
        if let Some(u) = &self.b[i] {
            self.a[i] = self.spec.update(&self.a[i], u)
        }
    }

    fn _apply(&mut self, i: usize, u: &U) {
        self.a[i] = self.spec.update(&self.a[i], u);
        if i < self._tsize() {
            self.b[i] = (&self.b[i])
                .as_ref()
                .map(|x| self.spec.compose(x, u))
                .or_else(|| Some(u.clone()));
        }
    }

    fn _build(&mut self, mut l: usize) {
        while l > 1 {
            l >>= 1;
            self._calc(l);
        }
    }

    fn _propagate(&mut self, i: usize) {
        if let Some(u) = self.b[i].take() {
            self._apply(i << 1, &u);
            self._apply(i << 1 | 1, &u);
        }
    }

    fn _push(&mut self, l: usize) {
        let h = 1usize.leading_zeros() - l.leading_zeros();
        for s in (1..=h).rev() {
            self._propagate(l >> s);
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        debug_assert!(index < self.len());
        let mut i = index + self._tsize();
        self._push(i);
        self.a[i] = value;
        loop {
            i >>= 1;
            if i == 0 {
                break;
            }
            self.a[i] = self.spec.op(&self.a[i << 1], &self.a[i << 1 | 1])
        }
    }

    pub fn get(&mut self, index: usize) -> &T {
        debug_assert!(index < self.len());
        let i = index + self._tsize();
        self._push(i);
        &self.a[i]
    }

    pub fn update(&mut self, mut l: usize, rx: usize, upd: U) {
        let tsz = self._tsize();
        l += tsz;
        let mut r = min(rx, self.len()) + tsz;
        if l >= r {
            return;
        }

        let l0 = l;
        let r0 = r - 1;
        self._push(l0);
        self._push(r0);

        while l < r {
            if l & 1 == 1 {
                self._apply(l, &upd);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                self._apply(r, &upd)
            }

            l >>= 1;
            r >>= 1;
        }

        self._build(l0);
        self._build(r0);
    }

    pub fn product(&mut self, mut l: usize, rx: usize) -> T {
        let tsz = self._tsize();
        l += tsz;
        let mut r = min(rx, self.len()) + tsz;

        if l >= r {
            return self.spec.identity();
        }
        self._push(l);
        self._push(r - 1);

        let mut l_acc = self.spec.identity();
        let mut r_acc = self.spec.identity();

        while l < r {
            if l & 1 == 1 {
                l_acc = self.spec.op(&l_acc, &self.a[l]);
                l += 1;
            }

            if r & 1 == 1 {
                r -= 1;
                r_acc = self.spec.op(&self.a[r], &r_acc);
            }

            l >>= 1;
            r >>= 1;
        }

        self.spec.op(&l_acc, &r_acc)
    }

    pub fn all_product(&self) -> &T {
        &self.a[1]
    }
}
