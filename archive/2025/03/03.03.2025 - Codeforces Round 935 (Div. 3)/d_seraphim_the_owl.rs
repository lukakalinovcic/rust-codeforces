//{"name":"D. Seraphim the Owl","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 2\n7 3 6 9\n4 3 8 5\n6 2\n6 9 7 1 8 3\n5 8 8 1 4 1\n7 7\n7 2 9 2 6 5 9\n9 1 10 7 1 4 9\n2 1\n2 3\n1 1\n","output":"14\n22\n9\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSeraphimTheOwl"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut best = 0;
    let mut dp = vec![0; n + 1];
    for i in (0..n).rev() {
        dp[i] = best + a[i] as i64;
        best += b[i] as i64;
        best = best.min(dp[i]);
    }
    let mut result = dp[0];
    for i in 0..m {
        result = result.min(dp[i]);
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
