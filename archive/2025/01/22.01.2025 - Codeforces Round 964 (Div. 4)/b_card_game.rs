//{"name":"B. Card Game","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 8 2 6\n1 1 1 1\n10 10 2 2\n1 1 10 10\n3 8 7 2\n","output":"2\n0\n4\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCardGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a1 = input.read_int();
    let a2 = input.read_int();
    let b1 = input.read_int();
    let b2 = input.read_int();
    let mut result = 0;
    if a1 > b1 && a2 >= b2 || a1 >= b1 && a2 > b2 {
        result += 1;
    }
    if a1 > b2 && a2 >= b1 || a1 >= b2 && a2 > b1 {
        result += 1;
    }
    if a2 > b1 && a1 >= b2 || a2 >= b1 && a1 > b2 {
        result += 1;
    }
    if a2 > b2 && a1 >= b1 || a2 >= b2 && a1 > b1 {
        result += 1;
    }
    out.print_line(result);
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
