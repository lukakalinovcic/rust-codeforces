//{"name":"F. Color Rows and Columns","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n1 4\n6 3\n1 5\n4 4\n5 10\n1 1\n1 1\n1 1\n1 1\n1 1\n2 100\n1 2\n5 6\n3 11\n2 2\n3 3\n4 4\n3 25\n9 2\n4 3\n8 10\n4 18\n5 4\n8 5\n8 3\n6 2\n","output":"12\n14\n5\n-1\n17\n80\n35\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FColorRowsAndColumns"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let mut dp = vec![INF; k + 1];
    dp[0] = 0;
    for (a, b) in input.read_int_pair_vec(n) {
        let mut dp_next = dp.clone();
        let (mut a, mut b) = (a.min(b), a.max(b));
        let mut cost = 0;
        let mut points = 0;
        while (a, b) != (0, 0) {
            cost += a;
            points += 1;
            for i in (points..=k).rev() {
                dp_next[i] = dp_next[i].min(dp[i - points] + cost);
            }
            (a, b) = (a.min(b - 1), a.max(b - 1));
        }
        dp = dp_next;
    }
    if dp[k] == INF {
        out.print_line(-1);
    } else {
        out.print_line(dp[k]);
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
