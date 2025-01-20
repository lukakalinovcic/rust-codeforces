//{"name":"C1. Adjust The Presentation (Easy Version)","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/C1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 2 0\n1 2 3 4\n1 1\n3 6 0\n1 2 3\n1 1 2 3 3 2\n4 6 0\n3 1 4 2\n3 1 1 2 3 4\n","output":"YA\nYA\nTIDAK\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1AdjustThePresentationEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m: usize = input.read();
    let _q: usize = input.read();
    let a = input.read_int_vec(n);
    let mut first = vec![m; n + 1];
    for i in 0..m {
        let b: usize = input.read();
        if first[b] == m {
            first[b] = i;
        }
    }
    for i in 1..n {
        if first[a[i] as usize] < first[a[i - 1] as usize] {
            out.print_line("TIDAK");
            return;
        }
    }
    out.print_line("YA");
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
