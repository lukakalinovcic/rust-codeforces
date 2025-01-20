//{"name":"D. Subtract Min Sort","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5\n1 2 3 4 5\n4\n4 3 2 1\n4\n4 5 2 3\n8\n4 5 4 5 4 5 4 5\n9\n9 9 8 2 4 4 3 5 3\n","output":"YES\nNO\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSubtractMinSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            out.print_line("NO");
            return;
        }
        let m = a[i].min(a[i + 1]);
        a[i] -= m;
        a[i + 1] -= m;
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
