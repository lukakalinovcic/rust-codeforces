//{"name":"C. Penchick and BBQ Buns","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3\n12\n","output":"-1\n1 2 3 6 10 2 7 6 10 1 7 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPenchickAndBBQBuns"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn assign(a: usize, b: usize, out: &mut [i32], next: &mut i32) {
    out[a] = *next;
    out[b] = *next;
    *next += 1;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    if n % 2 == 1 && n < 27 {
        out.print_line(-1);
    } else {
        let mut result = vec![0; n];
        let mut next = 1;
        if n % 2 == 1 {
            result[0] = 1;
            assign(9, 25, &mut result, &mut next);
            assign(22, 26, &mut result, &mut next);
            assign(23, 24, &mut result, &mut next);
            for i in (1..8).step_by(2) {
                assign(i, i + 1, &mut result, &mut next);
            }
            for i in (10..21).step_by(2) {
                assign(i, i + 1, &mut result, &mut next);
            }
            for i in (27..n).step_by(2) {
                assign(i, i + 1, &mut result, &mut next);
            }
        } else {
            for i in (0..n).step_by(2) {
                assign(i, i + 1, &mut result, &mut next);
            }
        }
        out.print_iter(result.into_iter());
        out.put(b'\n');
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
