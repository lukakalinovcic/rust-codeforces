//{"name":"C. Clock and Strings","group":"Codeforces - Codeforces Round 944 (Div. 4)","url":"https://codeforces.com/contest/1971/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"15\n2 9 10 6\n3 8 9 1\n1 2 3 4\n5 3 4 12\n1 8 2 10\n3 12 11 8\n9 10 12 1\n12 1 10 2\n3 12 6 9\n1 9 8 4\n6 7 9 12\n7 12 9 6\n10 12 11 1\n3 9 6 12\n1 4 3 5\n","output":"YES\nNO\nNO\nYES\nYES\nNO\nNO\nNO\nNO\nNO\nNO\nYES\nYES\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CClockAndStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int();
    let b = input.read_int();
    let (a, b) = (a.min(b), a.max(b));
    let c = input.read_int();
    let d = input.read_int();
    let c_side = c > a && c < b;
    let d_side = d > a && d < b;
    if c_side == d_side {
        out.print_line("NO");
    } else {
        out.print_line("YES");
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
