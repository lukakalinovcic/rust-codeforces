//{"name":"A. Phone Desktop","group":"Codeforces - Codeforces Round 946 (Div. 3)","url":"https://codeforces.com/contest/1974/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n1 1\n7 2\n12 4\n0 3\n1 0\n8 1\n0 0\n2 0\n15 0\n8 2\n0 9\n","output":"1\n1\n2\n2\n1\n1\n0\n1\n1\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APhoneDesktop"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_int();
    let y = input.read_int();
    let k = (y + 1) / 2;
    let x = x - 5 * 3 * k + y * 4;
    let k = if x >= 0 {
        k + (x + 5 * 3 - 1) / (5 * 3)
    } else {
        k
    };
    out.print_line(k);
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
