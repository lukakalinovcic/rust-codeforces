//{"name":"C. Swap Columns and Find a Path","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1\n-10\n5\n3\n1 2 3\n10 -5 -3\n4\n2 8 5 3\n1 10 3 4\n","output":"-5\n16\n29\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSwapColumnsAndFindAPath"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut a_sum = 0;
    let mut b_sum = 0;
    for i in 0..n {
        if a[i] >= b[i] {
            a_sum += a[i];
        } else {
            b_sum += b[i];
        }
    }
    let mut result = -1000000000;
    for i in 0..n {
        let sum = a_sum + b_sum + if a[i] >= b[i] { b[i] } else { a[i] };
        result = result.max(sum);
    }
    out.print_line(result);
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
