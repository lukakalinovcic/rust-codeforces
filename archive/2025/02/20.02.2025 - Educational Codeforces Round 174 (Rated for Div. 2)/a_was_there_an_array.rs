//{"name":"A. Was there an Array?","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10\n0 1 0 0 0 0 1 1\n3\n1\n10\n0 1 0 1 1 0 0 1\n","output":"YES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWasThereAnArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let b = input.read_int_vec(n - 2);
    for i in 2..n - 2 {
        if b[i - 2] == 1 && b[i - 1] == 0 && b[i] == 1 {
            out.print_line("NO");
            return;
        }
    }
    out.print_line("YES");
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
