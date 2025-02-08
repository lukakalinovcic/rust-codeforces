//{"name":"F. Large Graph","group":"Codeforces - Codeforces Round 953 (Div. 2)","url":"https://codeforces.com/contest/1978/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"6\n3 3\n3 4 5\n3 3\n3 4 9\n3 2\n3 4 9\n2 2\n2 8\n5 3\n8 27 5 4 3\n4 10\n2 2 2 2\n","output":"3\n2\n3\n1\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLargeGraph"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    primes: Vec<i32>,
    pos1: Vec<Vec<i32>>,
    pos2: Vec<Vec<i32>>,
}

impl PreCalc {
    fn new() -> Self {
        const MAX: usize = 1000000;
        let mut not_prime = vec![false; MAX];
        let mut primes = vec![];
        for i in 2..MAX {
            if not_prime[i] {
                continue;
            }
            primes.push(i as i32);
            let mut j = i * i;
            while j < MAX {
                not_prime[j] = true;
                j += i;
            }
        }
        let n = primes.len();
        Self {
            primes,
            pos1: vec![vec![]; n],
            pos2: vec![vec![]; n],
        }
    }
}

struct DSU {
    p: Vec<i32>,
    sz: Vec<i64>,
}

impl DSU {
    fn new(sz: Vec<i64>) -> Self {
        let n = sz.len();
        Self {
            p: (0..n as i32).collect::<Vec<_>>(),
            sz,
        }
    }

    fn find(&mut self, u: i32) -> i32 {
        if self.p[u as usize] != u {
            self.p[u as usize] = self.find(self.p[u as usize]);
        }
        self.p[u as usize]
    }

    fn union(&mut self, u: i32, v: i32) -> bool {
        let u = self.find(u);
        let v = self.find(v);
        if u == v {
            false
        } else {
            self.p[v as usize] = u;
            self.sz[u as usize] += self.sz[v as usize];
            true
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let a = input.read_int_vec(n);

    out.print_line(doit(a, k, precalc));
}

fn doit(a: Vec<i32>, k: usize, precalc: &mut PreCalc) -> i64 {
    let n = a.len();
    let primes = &precalc.primes;
    let pos1 = &mut precalc.pos1;
    let pos2 = &mut precalc.pos2;
    let mut sz = vec![0; 2 * n];
    let mut active = HashSet::new();
    for i in 0..n {
        sz[i] = i as i64;
        sz[n + i] = (n - i) as i64;
        let mut x = a[i];
        for j in 0..primes.len() {
            let p = primes[j];
            if p * p > x {
                break;
            }
            if x % p == 0 {
                if i > 0 {
                    pos1[j].push(i as i32);
                }
                pos2[j].push((n + i) as i32);
                active.insert(j);
                while x % p == 0 {
                    x /= p;
                }
            }
        }
        if x > 1 {
            let (Ok(j) | Err(j)) = primes.binary_search(&x);
            if i > 0 {
                pos1[j].push(i as i32);
            }
            pos2[j].push((n + i) as i32);
            active.insert(j);
        }
    }

    let mut dsu = DSU::new(sz.clone());
    for j in active.drain() {
        pos1[j].extend(pos2[j].drain(..));
        let pos = &mut pos1[j];
        let mut tail = 0;
        for head in 0..pos.len() {
            while pos[head] - pos[tail] > k as i32 {
                tail += 1;
            }
            dsu.union(pos[head], pos[tail]);
        }
        let head = pos.len() - 1;
        tail += 1;
        while tail < pos.len() {
            dsu.union(pos[head], pos[tail]);
            tail += 1;
        }
        pos.clear();
    }
    let mut result = n as i64 * n as i64;
    for i in 1..2 * n {
        if dsu.find(i as i32) == i as i32 {
            if a[i % n] > 1 {
                result -= (dsu.sz[i] - 1).max(0);
            }
        }
    }
    result
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn brute(a: Vec<i32>, k: usize) -> i64 {
    let n = a.len();
    let mut dsu = DSU::new(vec![1; n * n]);
    for i1 in 0..n {
        for j1 in 0..n {
            let x = a[(j1 + n - i1) % n];
            for i2 in 0..n {
                for j2 in 0..n {
                    let y = a[(j2 + n - i2) % n];
                    if (i1 as i32 - i2 as i32).abs() + (j1 as i32 - j2 as i32).abs() <= k as i32
                        && gcd(x, y) > 1
                    {
                        let u = (i1 * n + j1) as i32;
                        let v = (i2 * n + j2) as i32;
                        dsu.union(u, v);
                    }
                }
            }
        }
    }
    let mut result = n as i64 * n as i64;
    for i in 0..n * n {
        if dsu.find(i as i32) == i as i32 {
            result -= dsu.sz[i] - 1;
        }
    }
    result
}

fn main() {
    tester::run_tests();

    let mut precalc = PreCalc::new();
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 500 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=20);
        let k = rng.gen_range(2..=2 * n);
        let a = (0..n).map(|_| rng.gen_range(1..=40)).collect::<Vec<_>>();
        let x = doit(a.clone(), k, &mut precalc);
        let y = brute(a.clone(), k);
        if x != y {
            eprintln!("Mismatch at {a:?} {k}  {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
