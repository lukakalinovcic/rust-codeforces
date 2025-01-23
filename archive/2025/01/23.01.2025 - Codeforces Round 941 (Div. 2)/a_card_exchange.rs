//{"name":"A. Card Exchange","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5 3\n4 1 1 4 4\n1 10\n7\n7 2\n4 2 1 100 5 2 3\n10 4\n1 1 1 1 1 1 1 1 1 1\n5 2\n3 8 1 48 7\n6 2\n10 20 30 10 20 40\n6 3\n10 20 30 10 20 40\n","output":"2\n1\n1\n3\n5\n1\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACardExchange"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k: usize = input.read();
    let mut a = input.read_int_vec(n);
    a.sort();
    for i in k - 1..n {
        if a[i] == a[i - (k - 1)] {
            out.print_line(k - 1);
            return;
        }
    }
    out.print_line(n);
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
