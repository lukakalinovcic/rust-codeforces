//{"name":"B. Square or Not","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n11\n4\n1111\n9\n111101111\n9\n111111111\n12\n111110011111\n","output":"No\nYes\nYes\nNo\nNo\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSquareOrNot"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n_sq: usize = input.read();
    let s = input.read_line().into_bytes();
    let mut n = 1;
    while n * n < n_sq {
        n += 1;
    }
    if n * n != n_sq {
        out.print_line("No");
        return;
    }
    for r in 0..n {
        for c in 0..n {
            let i = r * n + c;
            let expected = if r == 0 || c == 0 || r == n - 1 || c == n - 1 {
                b'1'
            } else {
                b'0'
            };
            if s[i] != expected {
                out.print_line("No");
                return;
            }
        }
    }
    out.print_line("Yes");
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
