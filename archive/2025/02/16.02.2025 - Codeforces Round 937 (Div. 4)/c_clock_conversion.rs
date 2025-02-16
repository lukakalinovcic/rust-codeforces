//{"name":"C. Clock Conversion","group":"Codeforces - Codeforces Round 937 (Div. 4)","url":"https://codeforces.com/contest/1950/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n09:41\n18:06\n12:14\n00:59\n00:00\n14:34\n01:01\n19:07\n11:59\n12:00\n21:37\n","output":"09:41 AM\n06:06 PM\n12:14 PM\n12:59 AM\n12:00 AM\n02:34 PM\n01:01 AM\n07:07 PM\n11:59 AM\n12:00 PM\n09:37 PM\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CClockConversion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line().into_bytes();
    let hh = 10 * (s[0] - b'0') as i32 + (s[1] - b'0') as i32;
    let mm = 10 * (s[3] - b'0') as i32 + (s[4] - b'0') as i32;
    if hh >= 12 {
        out.print_line(format!("{:02}:{mm:02} PM", (hh + 11) % 12 + 1));
    } else {
        out.print_line(format!("{:02}:{mm:02} AM", (hh + 11) % 12 + 1));
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
