//{"name":"D. Odd Queries","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5 5\n2 2 1 3 2\n2 3 3\n2 3 4\n1 5 5\n1 4 9\n2 4 3\n10 5\n1 1 1 1 1 1 1 1 1 1\n3 8 13\n2 5 10\n3 8 10\n1 10 2\n1 9 100\n","output":"YES\nYES\nYES\nNO\nYES\nNO\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DOddQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let mut partial = vec![0; n + 1];
    for (i, x) in input.read_long_vec(n).into_iter().enumerate() {
        partial[i + 1] = partial[i] + x;
    }
    for _ in 0..q {
        let l: usize = input.read();
        let r: usize = input.read();
        let k = input.read_long();
        let new_sum = partial[n] - partial[r] + partial[l - 1] + (r - l + 1) as i64 * k;
        if new_sum % 2 == 1 {
            out.print_line("YES");
        } else {
            out.print_line("NO");
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
