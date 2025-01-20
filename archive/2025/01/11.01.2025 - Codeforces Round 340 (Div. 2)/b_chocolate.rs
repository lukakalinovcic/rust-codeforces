//{"name":"B. Chocolate","group":"Codeforces - Codeforces Round 340 (Div. 2)","url":"https://codeforces.com/contest/617/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 1 0\n","output":"1\n"},{"input":"5\n1 0 1 0 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BChocolate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut seen_one = false;
    let mut zeros = 0;
    let mut ways = 1 as i64;
    for a in input.read_int_vec(n) {
        if a == 0 {
            zeros += 1;
        } else {
            if seen_one {
                ways *= 1 + zeros;
            } else {
                seen_one = true;
            }
            zeros = 0;
        }
    }
    if !seen_one {
        out.print_line(0);
    } else {
        out.print_line(ways);
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
