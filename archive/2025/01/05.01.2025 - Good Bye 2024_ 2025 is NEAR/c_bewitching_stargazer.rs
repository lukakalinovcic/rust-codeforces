//{"name":"C. Bewitching Stargazer","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 2\n11 3\n55 13\n5801 6\n8919 64\n8765432 1\n","output":"12\n18\n196\n1975581\n958900\n38416403456028\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBewitchingStargazer"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn rec(n: i32, k: i32) -> (i64, i32) {
    if n < k {
        return (0, 0);
    }
    let (middle, left_half) = if n % 2 == 0 {
        ((0, 0), rec(n / 2, k))
    } else {
        ((((n + 1) / 2) as i64, 1), rec(n / 2, k))
    };
    let right_half = (
        left_half.0 + ((n + 1) / 2) as i64 * left_half.1 as i64,
        left_half.1,
    );
    (
        middle.0 + left_half.0 + right_half.0,
        middle.1 + left_half.1 + right_half.1,
    )
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let k = input.read_int();
    let (result, _) = rec(n, k);
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
