//{"name":"G. Money Buys Less Happiness Now","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 3\n2 2 2\n6 5\n2 2 8 2 6 8\n6 4\n4 10 3 8 6 10\n2 1\n1 1\n4 1\n4 1 3 1\n4 2\n1 3 4 3\n","output":"2\n4\n3\n1\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMoneyBuysLessHappinessNow"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

const INF: i32 = 1000000000;

#[derive(Clone, Default)]
struct MinValue {
    value: i32,
    position: usize,
}

struct MySpec {}
impl LazySegTreeSpec<MinValue, i32> for &MySpec {
    fn identity(&self) -> MinValue {
        MinValue {
            value: INF,
            position: 0,
        }
    }

    fn op(&self, a: &MinValue, b: &MinValue) -> MinValue {
        if a.value <= b.value {
            a.clone()
        } else {
            b.clone()
        }
    }

    fn compose(&self, a: &i32, b: &i32) -> i32 {
        *a + *b
    }

    fn update(&self, t: &MinValue, u: &i32) -> MinValue {
        MinValue {
            value: t.value + u,
            position: t.position,
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let m = input.read();
    let x = input.read_int();
    let c = input.read_int_vec(m);

    let tree_spec = MySpec {};
    let mut candidate_tree = LazySegTree::new(&tree_spec, m);
    candidate_tree.init(|i| MinValue {
        value: i as i32 * x - c[i],
        position: i as usize,
    });
    let mut selected_tree = LazySegTree::new(&tree_spec, m);
    selected_tree.init(|i| MinValue {
        value: INF,
        position: i as usize,
    });

    let mut active = BTreeSet::new();
    for i in 0..m {
        if i as i32 * x >= c[i] {
            active.insert((c[i], i));
        }
    }

    eprintln!("{c:?}");
    let mut result = 0;
    while !active.is_empty() {
        let (ci, i) = active.pop_first().unwrap();
        if selected_tree.product(i, m).value < ci {
            continue;
        }
        result += 1;
        selected_tree.update(i, m, -ci);
        selected_tree.set(i, candidate_tree.get(i).clone());
        candidate_tree.update(i, m, -ci);
        loop {
            let &MinValue { value, position } = candidate_tree.all_product();
            if value < 0 {
                candidate_tree.set(
                    position,
                    MinValue {
                        value: INF,
                        position,
                    },
                );
                active.remove(&(c[position], position));
            } else {
                break;
            }
        }
    }
    out.print_line(result);
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
