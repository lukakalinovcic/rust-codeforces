//{"name":"E. Broken Queries","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n8\n\n0\n\n0\n\n1\n\n0\n\n4\n\n1\n\n0\n","output":"? 3 5\n\n? 1 8\n\n? 4 8\n\n? 3 8\n\n! 6\n\n? 3 3\n\n? 3 4\n\n! 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBrokenQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn query(input: &mut Input, out: &mut Output, a: i32, b: i32) -> i32 {
    if a < 1 {
        out.print_line(format!("? 1 {}", b - a + 1));
    } else {
        out.print_line(format!("? {a} {b}"));
    }
    out.flush();
    input.read_int()
}

fn search(input: &mut Input, out: &mut Output, lo: i32, hi: i32, right: i32, expected: i32) -> i32 {
    if lo == hi {
        lo
    } else {
        let mid = (lo + hi) / 2;
        if query(input, out, right - mid + 1, right) == expected {
            search(input, out, mid + 1, hi, right, expected)
        } else {
            search(input, out, lo, mid, right, expected)
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let t = n / 3;
    let a = query(input, out, 1, t);
    let b = query(input, out, t + 1, t + t);
    let c = query(input, out, t + t + 1, t + t + t);
    let result = if a + b + c >= 2 {
        search(input, out, 2, t, if a == 0 { t + t } else { t }, 0)
    } else if a == 1 {
        search(input, out, t, n - 1, t, 1)
    } else if b == 1 {
        search(input, out, t, n - 1, t + t, 1)
    } else if c == 1 {
        search(input, out, t, n - 1, t + t + t, 1)
    } else {
        search(input, out, t, n - 1, n, 1)
    };
    out.print_line(format!("! {result}"));

    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);
    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);
    run(input, output);
}

//END MAIN
