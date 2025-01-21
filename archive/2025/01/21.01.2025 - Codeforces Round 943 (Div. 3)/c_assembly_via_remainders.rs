//{"name":"C. Assembly via Remainders","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n2 4 1\n3\n1 1\n6\n4 2 5 1 2\n2\n500\n3\n1 5\n","output":"3 5 4 9\n2 5 11\n5 14 16 5 11 24\n501 500\n2 7 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAssemblyViaRemainders"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let x = input.read_int_vec(n - 1);
    let mut a = vec![1000];
    for x in x {
        a.push(a.last().cloned().unwrap() + x);
    }
    out.print_iter(a.into_iter());
    out.print_line("");
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
