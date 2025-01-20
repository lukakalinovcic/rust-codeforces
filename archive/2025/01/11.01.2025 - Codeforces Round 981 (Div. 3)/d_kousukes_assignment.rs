//{"name":"D. Kousuke's Assignment","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n2 1 -3 2 1\n7\n12 -4 4 43 -3 -5 8\n6\n0 -4 0 3 0 1\n","output":"1\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKousukesAssignment"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut dp = vec![0; n + 1];
    let mut prev = HashMap::new();
    let mut sum = 0;
    prev.insert(sum, 0);
    for (i, a) in input.read_long_vec(n).into_iter().enumerate() {
        let i = i + 1;
        sum += a;

        dp[i] = dp[i - 1];
        if let Some(j) = prev.get(&sum) {
            let j = *j;
            dp[i] = dp[i].max(1 + dp[j]);
        }

        prev.insert(sum, i);
    }
    out.print_line(dp[n]);
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
