//{"name":"G. Penacony","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n8 3\n1 8\n2 7\n4 5\n13 4\n1 13\n2 12\n3 11\n4 10\n10 2\n2 3\n3 4\n10 4\n3 8\n5 10\n2 10\n4 10\n4 1\n1 3\n5 2\n3 5\n1 4\n5 2\n2 5\n1 3\n","output":"4\n7\n2\n7\n2\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GPenacony"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

const INF: i32 = 1000000000;

#[derive(Clone, Default)]
struct MinValueCnt {
    value: i32,
    count: i32,
}

struct MySpec {}
impl LazySegTreeSpec<MinValueCnt, i32> for &MySpec {
    fn identity(&self) -> MinValueCnt {
        MinValueCnt {
            value: INF,
            count: 0,
        }
    }

    fn op(&self, a: &MinValueCnt, b: &MinValueCnt) -> MinValueCnt {
        if a.value < b.value {
            a.clone()
        } else if b.value < a.value {
            b.clone()
        } else {
            MinValueCnt {
                value: a.value,
                count: a.count + b.count,
            }
        }
    }

    fn compose(&self, a: &i32, b: &i32) -> i32 {
        *a + *b
    }

    fn update(&self, t: &MinValueCnt, u: &i32) -> MinValueCnt {
        MinValueCnt {
            value: t.value + u,
            count: t.count,
        }
    }
}

fn doit(n: usize, e: Vec<(i32, i32)>) -> i32 {
    let spec = MySpec {};
    let mut tree = LazySegTree::new(&spec, 2 * n);
    for i in 0..2 * n {
        tree.set(i, MinValueCnt { value: 0, count: 1 });
    }
    let mut e1 = e;
    let mut e2: Vec<_> = e1.clone().into_iter().map(|(a, b)| (b, a)).collect();
    e1.sort();
    e2.sort();
    for (a, b) in &e1 {
        let (a, b) = (*a as usize, *b as usize);
        tree.update(a, b, 1);
    }
    let min = tree.product(1, n);
    let mut best = if min.value == 0 { min.count } else { 0 };
    let mut j1 = 0;
    let mut j2 = 0;
    for i in 1..n {
        while j1 < e1.len() && e1[j1].0 == i as i32 {
            let (u, v) = e1[j1];
            j1 += 1;
            let (a, b) = (u as usize, v as usize);
            tree.update(a, b, -1);

            let (a, b) = (v as usize, u as usize + n);
            tree.update(a, b, 1);
        }
        while j2 < e2.len() && e2[j2].0 == i as i32 {
            let (u, v) = e2[j2];
            j2 += 1;
            let (a, b) = (u as usize, v as usize + n);
            tree.update(a, b, -1);

            let (a, b) = (v as usize + n, u as usize + n);
            tree.update(a, b, 1);
        }
        let min = tree.product(i + 1, i + n);
        best = best.max(if min.value == 0 { min.count } else { 0 });
    }
    n as i32 - 1 - best
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let e = input.read_int_pair_vec(m);
    out.print_line(doit(n, e));
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

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

fn brute(n: usize, e: Vec<(i32, i32)>) -> i32 {
    let mut best = 0;
    for i in 0..n {
        let mut covered = vec![false; n - 1];
        for (a, b) in &e {
            let (a, b) = (*a as usize, *b as usize);
            let (u, v) = if i < a {
                (a, b - 1)
            } else if i >= a && i < b {
                (b, a + n - 1)
            } else {
                (a + n, b + n - 1)
            };
            for j in u..=v {
                covered[j - i - 1] = true;
            }
        }
        let mut cnt_uncovered = 0;
        for b in covered {
            if b == false {
                cnt_uncovered += 1;
            }
        }
        best = best.max(cnt_uncovered);
    }
    n as i32 - 1 - best
}
fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..20);
        let m = rng.gen_range(1..20);
        let e: Vec<_> = (0..m)
            .map(|_| {
                let a = rng.gen_range(1..n);
                let b = rng.gen_range(a + 1..=n);
                (a as i32, b as i32)
            })
            .collect();
        let a = doit(n, e.clone());
        let b = brute(n, e.clone());
        if a != b {
            eprintln!("Mismatch at {n} {m} {e:?}");
            break;
        }
    }
}
//END MAIN
