//{"name":"A. Circuit","group":"Codeforces - Codeforces Round 983 (Div. 2)","url":"https://codeforces.com/contest/2032/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1\n0 0\n1\n0 1\n1\n1 1\n3\n0 0 1 0 1 0\n3\n0 1 1 1 0 0\n","output":"0 0\n1 1\n0 0\n0 2\n1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACircuit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let mut zeroes = 0;
    let mut ones = 0;
    for a in input.read_int_vec(2 * n) {
        if a == 0 {
            zeroes += 1;
        } else {
            ones += 1;
        }
    }
    let mini = ones % 2;
    let maxi = ones.min(zeroes);
    out.print_line(format!("{mini} {maxi}"));
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
