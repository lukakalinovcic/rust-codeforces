//{"name":"A. Array Coloring","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n8\n1 2 4 3 2 3 5 4\n2\n4 7\n3\n3 9 8\n2\n1 7\n5\n5 4 3 2 1\n4\n4 3 4 5\n2\n50 48\n","output":"YES\nNO\nYES\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AArrayColoring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut num_odd = 0;
    for a in input.read_int_vec(n) {
        if a % 2 == 1 {
            num_odd += 1;
        }
    }
    if num_odd % 2 == 1 {
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
