//{"name":"G. Library of Magic","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/G","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n6\n\n0\n\n2\n\n3\n\n5\n\n3\n","output":"xor 1 1\n\nxor 2 2\n\nxor 3 3\n\nxor 4 6\n\nans 2 3 5\n\nans 1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GLibraryOfMagic"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn query(input: &mut Input, out: &mut Output, n: i64, lo: i64, hi: i64) -> i64 {
    let (a, b) = (lo.max(1), (hi - 1).min(n));
    if a > b {
        return 0;
    }
    out.print_line(format!("xor {a} {b}"));
    out.flush();
    input.read()
}

fn search1(input: &mut Input, out: &mut Output, n: i64, lo: i64, hi: i64) -> i64 {
    if hi - lo == 1 {
        lo
    } else {
        let mid = (lo + hi) / 2;
        if query(input, out, n, mid, hi) != 0 {
            search1(input, out, n, mid, hi)
        } else {
            search1(input, out, n, lo, mid)
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let mut hi = 4;
    while hi <= n {
        hi *= 2;
    }
    let a = search1(input, out, n, 0, hi);
    let b = if query(input, out, n, a + 1, n + 1) != 0 {
        search1(input, out, n, a + 1, n + 1)
    } else {
        search1(input, out, n, 1, a)
    };
    let (a, b) = (a.min(b), b.max(a));
    let c = query(input, out, n, 1, n + 1) ^ a ^ b;
    out.print_line(format!("ans {a} {b} {c}"));
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
