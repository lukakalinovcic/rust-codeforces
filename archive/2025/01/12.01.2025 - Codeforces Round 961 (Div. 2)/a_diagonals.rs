//{"name":"A. Diagonals","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n1 0\n2 2\n2 3\n2 4\n10 50\n100 239\n3 9\n","output":"0\n1\n2\n3\n6\n3\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADiagonals"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let mut k = input.read_int();
    if k == 0 {
        out.print_line(0);
        return;
    }
    for i in (0..2 * n - 1).rev() {
        let x = (i + 2) / 2;
        k -= x;
        if k <= 0 {
            out.print_line(2 * n - i - 1);
            return;
        }
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
