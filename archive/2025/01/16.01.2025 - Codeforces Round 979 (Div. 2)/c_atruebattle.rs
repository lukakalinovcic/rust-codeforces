//{"name":"C. A TRUE Battle","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n11\n3\n010\n12\n101111111100\n10\n0111111011\n8\n01000010\n","output":"YES\nNO\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CATRUEBattle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s = input.read_line().into_bytes();
    let mut longest_one_run = 0;
    let mut current_one_run = 0;
    let mut prev = b'0';
    for i in 0..n {
        if s[i] == b'1' {
            if prev == b'1' {
                current_one_run += 1;
                longest_one_run = longest_one_run.max(current_one_run);
            } else {
                current_one_run = 1;
                longest_one_run = longest_one_run.max(current_one_run);
            }
        } else {
            current_one_run = 0;
        }
        prev = s[i];
    }
    if longest_one_run >= 2 || s[0] == b'1' || s[n - 1] == b'1' {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
