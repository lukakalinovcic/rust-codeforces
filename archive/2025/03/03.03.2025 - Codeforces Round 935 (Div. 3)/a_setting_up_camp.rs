//{"name":"A. Setting up Camp","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 3\n1 4 1\n1 4 2\n1 1 1\n1 3 2\n19 7 18\n0 0 0\n7 0 0\n0 24 0\n1000000000 1000000000 1000000000\n","output":"3\n-1\n3\n-1\n3\n28\n0\n7\n8\n1666666667\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASettingUpCamp"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_int();
    let b = input.read_int();
    let c = input.read_int();
    if b % 3 != 0 && c < 3 - (b % 3) {
        out.print_line(-1);
    } else {
        out.print_line(a + (b + c + 2) / 3);
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
