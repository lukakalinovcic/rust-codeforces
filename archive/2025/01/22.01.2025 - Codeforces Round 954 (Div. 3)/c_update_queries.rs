//{"name":"C. Update Queries","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 2\na\n1 1\ncb\n4 4\nmeow\n1 2 1 4\nzcwz\n7 4\nabacaba\n1 3 5 7\ndamn\n7 10\ntraktor\n7 6 5 4 3 2 1 6 4 2\ncodeforces\n","output":"b\ncwoz\nabdcmbn\nccdeefo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CUpdateQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n: usize = input.read();
    let m = input.read();
    let mut s = input.read_line().into_bytes();
    let mut p = input.read_int_vec(m);
    let mut c = input.read_line().into_bytes();
    p.sort();
    p.dedup();
    c.sort();
    for i in 0..p.len() {
        s[p[i] as usize - 1] = c[i];
    }
    out.print_line(String::from_utf8(s).unwrap());
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
