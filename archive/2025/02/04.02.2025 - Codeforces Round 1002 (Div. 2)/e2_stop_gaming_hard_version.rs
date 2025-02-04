//{"name":"E2. Stop Gaming (Hard Version)","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/E2","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 2\n2 6\n3 4\n1 2\n7 8\n1 5\n5 4 1 2 3\n5 4 3 2 1\n3 3\n1 2 3\n4 5 6\n7 8 9\n11 1 2\n12 3 4\n13 5 6\n4 4\n1 2 3 4\n5 6 7 8\n9 10 11 12\n13 14 15 16\n17 1 2 3\n4 18 5 6\n7 19 8 20\n9 21 22 10\n","output":"3\n1 1\n2 8\n2 7\n5\n1 1\n1 2\n1 3\n1 4\n1 5\n3\n1 11\n2 12\n3 13\n6\n3 20\n2 18\n3 19\n4 22\n4 21\n1 17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2StopGamingHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

#[derive(Clone, Debug)]
enum Event {
    TookA(i32),
    TookB(i32, i32),
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n * m);
    let b = input.read_int_vec(n * m);
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    let mut events = vec![Event::TookA(0); n * m];
    while j < n * m {
        let row = j / m;
        let row_start = row * m;
        if b[j] == a[i] {
            if i < row_start {
                events[j] = Event::TookA((row_start - i) as i32);
                i += 1;
                j += 1;
                continue;
            }
            let rem = (row + 1) * m - j;
            let mut ok = true;
            for k in 1..rem {
                if b[j + k] != a[i + k] {
                    ok = false;
                    break;
                }
            }
            if ok {
                for _ in 0..rem {
                    events[j] = Event::TookA(0);
                    j += 1;
                }
                i += rem;
                continue;
            }
        }
        events[j] = Event::TookB((row_start - i) as i32, b[j]);
        j += 1;
        result += 1;
    }
    let tree_spec = MySpec {};
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = m;
    }
    let mut tree = LazySegTree::new(&tree_spec, n);
    tree.init(|i| MinValueMaxPosition {
        value: match events[i * m + p[i] - 1] {
            Event::TookA(shifts) => shifts,
            Event::TookB(shifts, _) => shifts,
        },
        position: i as i32,
    });
    let mut ops = vec![];
    for _ in 0..n * m {
        let i = tree.all_product().position as usize;
        if let Event::TookB(_, val) = events[i * m + p[i] - 1] {
            ops.push((i + 1, val));
            tree.update(i + 1, n, -1);
        }
        let delta = match events[i * m + p[i] - 1] {
            Event::TookA(shifts) => shifts,
            Event::TookB(shifts, _) => shifts,
        };
        p[i] -= 1;
        let value = if p[i] == 0 {
            INF
        } else {
            match events[i * m + p[i] - 1] {
                Event::TookA(shifts) => shifts - delta,
                Event::TookB(shifts, _) => shifts - delta,
            }
        };
        tree.set(
            i,
            MinValueMaxPosition {
                value: value,
                position: i as i32,
            },
        );
    }
    out.print_line(result);
    for op in ops {
        out.print_line(op);
    }
}

const INF: i32 = 1000000000;

#[derive(Clone, Default)]
struct MinValueMaxPosition {
    value: i32,
    position: i32,
}

struct MySpec {}
impl LazySegTreeSpec<MinValueMaxPosition, i32> for &MySpec {
    fn identity(&self) -> MinValueMaxPosition {
        MinValueMaxPosition {
            value: INF,
            position: 0,
        }
    }

    fn op(&self, a: &MinValueMaxPosition, b: &MinValueMaxPosition) -> MinValueMaxPosition {
        if a.value < b.value {
            a.clone()
        } else if b.value < a.value {
            b.clone()
        } else {
            MinValueMaxPosition {
                value: a.value,
                position: a.position.max(b.position),
            }
        }
    }

    fn compose(&self, a: &i32, b: &i32) -> i32 {
        *a + *b
    }

    fn update(&self, t: &MinValueMaxPosition, u: &i32) -> MinValueMaxPosition {
        MinValueMaxPosition {
            value: t.value + u,
            position: t.position,
        }
    }
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

fn main() {
    tester::run_tests();
}
//END MAIN
