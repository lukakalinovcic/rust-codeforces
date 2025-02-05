pub struct RMQ<T: PartialOrd> {
    a: Vec<Vec<T>>,
}

impl<T: Ord + Copy> RMQ<T> {
    pub fn new(s: Vec<T>) -> Self {
        let n = s.len();
        let m = (64 - (n - 1).leading_zeros().min(63)) as usize;
        let mut a = vec![s];
        for j in 1..m {
            let mut next = vec![];
            let half = 1 << (j - 1);
            let full = 1 << j;
            for i in 0..=n - full {
                let x = a[j - 1][i].min(a[j - 1][i + half]);
                next.push(x);
            }
            a.push(next);
        }
        Self { a }
    }

    pub fn query(&self, lo: usize, hi: usize) -> T {
        let j = (63 - (hi - lo).leading_zeros().min(63)) as usize;
        self.a[j][lo].min(self.a[j][hi + 1 - (1 << j)])
    }
}
