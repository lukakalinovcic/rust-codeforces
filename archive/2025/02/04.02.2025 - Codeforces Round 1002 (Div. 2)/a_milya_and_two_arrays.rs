//{"name":"A. Milya and Two Arrays","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n1 2 1 2\n1 2 1 2\n6\n1 2 3 3 2 1\n1 1 1 1 1 1\n3\n1 1 1\n1 1 1\n6\n1 52 52 3 1 3\n59 4 3 59 3 4\n4\n100 1 100 1\n2 2 2 2\n","output":"YES\nYES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMilyaAndTwoArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    let mut b = input.read_int_vec(n);
    a.sort();
    a.dedup();
    b.sort();
    b.dedup();
    if a.len() * b.len() >= 3 {
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
