//{"name":"E. Novice's Mistake","group":"Codeforces - Codeforces Round 957 (Div. 3)","url":"https://codeforces.com/contest/1992/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n2\n3\n10\n","output":"3\n20 18\n219 216\n2218 2214\n1\n165 162\n1\n1262 2519\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENovicesMistake"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let mut a_digits = 1;
    let mut p10 = 10;
    let mut results = vec![];
    let (n_digits, p10) = if n < 10 {
        (1, 10)
    } else if n < 100 {
        (2, 100)
    } else {
        (3, 1000)
    };
    for a in 1..=10000 {
        for target_digits in 1..=6 {
            if n_digits * a < target_digits {
                break;
            }
            let mut x = 0;
            let mut x_digits = 0;
            while x_digits < target_digits {
                x = x * p10 + n;
                x_digits += n_digits;
                while x_digits > target_digits {
                    x /= 10;
                    x_digits -= 1;
                }
            }
            let b = n * a - x;
            if 1 <= b && b <= 10000.min(n * a) && x_digits == n_digits * a - b {
                results.push((a, b));
            }
        }
    }
    out.print_line(results.len());
    for (a, b) in results {
        out.print_line((a, b));
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
