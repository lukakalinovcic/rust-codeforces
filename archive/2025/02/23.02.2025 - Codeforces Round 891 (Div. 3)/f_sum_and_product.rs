//{"name":"F. Sum and Product","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n3\n1 3 2\n4\n3 2\n5 6\n3 1\n5 5\n4\n1 1 1 1\n1\n2 1\n6\n1 4 -2 3 3 3\n3\n2 -8\n-1 -2\n7 12\n","output":"1 1 0 0\n6\n1 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSumAndProduct"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    for a in a {
        *cnt.entry(a).or_default() += 1;
    }
    let q = input.read_size();
    out.print_iter(input.read_long_pair_vec(q).into_iter().map(|(x, y)| {
        match sqrt(x * x - 4 * y) {
            None => 0,
            Some(c) => {
                let t = x - c;
                if t % 2 != 0 {
                    0
                } else {
                    let a = t / 2;
                    let b = x - a;
                    let ca = cnt.get(&a).cloned().unwrap_or_default();
                    let cb = cnt.get(&b).cloned().unwrap_or_default();
                    if a == b {
                        ca * (ca - 1) / 2
                    } else {
                        ca * cb
                    }
                }
            }
        }
    }));
    out.print_line("");
}

fn sqrt(x: i64) -> Option<i64> {
    if x < 0 {
        return None;
    }
    let mut lo: i64 = 0;
    let mut hi: i64 = 2000000000;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if mid * mid < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    if lo * lo == x {
        Some(lo)
    } else {
        None
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
