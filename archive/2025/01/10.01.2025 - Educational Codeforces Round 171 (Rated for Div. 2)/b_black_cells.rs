//{"name":"B. Black Cells","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n1\n7\n3\n2 4 9\n5\n1 5 8 10 13\n","output":"1\n1\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBlackCells"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = i64::MAX;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut prefix_result = vec![INF; n + 1];
    prefix_result[0] = 0;
    for i in (1..n).step_by(2) {
        prefix_result[i + 1] = prefix_result[i - 1].max(a[i] - a[i - 1]);
    }
    let mut suffix_result = 0;
    let mut result = prefix_result[n];
    for i in (0..n).rev().step_by(2) {
        result = result.min(prefix_result[i].max(1.max(suffix_result)));
        if i > 0 {
            suffix_result = suffix_result.max(a[i] - a[i - 1]);
            result = result.min(prefix_result[i - 1].max(suffix_result));
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
