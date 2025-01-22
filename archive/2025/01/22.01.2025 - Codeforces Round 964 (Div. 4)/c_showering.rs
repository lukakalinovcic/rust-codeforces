//{"name":"C. Showering","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 3 10\n3 5\n6 8\n9 10\n3 3 10\n1 2\n3 5\n6 7\n3 3 10\n1 2\n3 5\n6 8\n3 4 10\n1 2\n6 7\n8 9\n","output":"YES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CShowering"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s = input.read_int();
    let m = input.read_int();
    let mut a = vec![vec![(0, 0)], input.read_int_pair_vec(n), vec![(m, m)]].concat();
    for i in 1..=n + 1 {
        if a[i].0 - a[i - 1].1 >= s {
            out.print_line("YES");
            return;
        }
    }
    out.print_line("NO");
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
