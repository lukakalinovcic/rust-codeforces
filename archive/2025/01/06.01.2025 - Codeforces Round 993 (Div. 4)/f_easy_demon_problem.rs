//{"name":"F. Easy Demon Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3 3 6\n-2 3 -3\n-2 2 -1\n-1\n1\n-2\n2\n-3\n3\n","output":"NO\nYES\nNO\nNO\nYES\nNO\n"},{"input":"5 5 6\n1 -2 3 0 0\n0 -2 5 0 -3\n4\n-3\n5\n2\n-1\n2\n","output":"YES\nYES\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEasyDemonProblem"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let q: usize = input.read();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);
    let s_a: i64 = a.iter().map(|i| *i).sum();
    let s_b: i64 = b.iter().map(|i| *i).sum();
    let mut a_cands = BTreeSet::new();
    for x in a {
        a_cands.insert(s_a - x);
    }
    let mut b_cands = BTreeSet::new();
    for x in b {
        b_cands.insert(s_b - x);
    }
    for _ in 0..q {
        let z = input.read_long();
        let mut result = false;
        for x in 1.. {
            if x * x > z.abs() {
                break;
            }
            if z % x != 0 {
                continue;
            }
            let y = z / x;
            if a_cands.contains(&x) && b_cands.contains(&y)
                || a_cands.contains(&y) && b_cands.contains(&x)
                || a_cands.contains(&-x) && b_cands.contains(&-y)
                || a_cands.contains(&-y) && b_cands.contains(&-x)
            {
                result = true;
                break;
            }
        }
        if result {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
