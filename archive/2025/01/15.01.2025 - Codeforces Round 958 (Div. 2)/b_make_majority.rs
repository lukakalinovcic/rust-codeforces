//{"name":"B. Make Majority","group":"Codeforces - Codeforces Round 958 (Div. 2)","url":"https://codeforces.com/contest/1988/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"5\n1\n0\n1\n1\n2\n01\n9\n100000001\n9\n000011000\n","output":"No\nYes\nNo\nYes\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMakeMajority"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s = input.read_line().into_bytes();
    let mut prev = b'2';
    let mut c0 = 0;
    let mut c1 = 0;
    for i in 0..n {
        if s[i] == b'1' {
            c1 += 1;
        } else if prev != b'0' {
            c0 += 1;
        }
        prev = s[i];
    }
    if c0 >= c1 {
        out.print_line("No");
    } else {
        out.print_line("Yes");
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
