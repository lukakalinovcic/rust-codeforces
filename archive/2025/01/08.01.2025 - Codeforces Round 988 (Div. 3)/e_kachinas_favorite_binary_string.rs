//{"name":"E. Kachina's Favorite Binary String","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/contest/2037/problem/E","interactive":true,"timeLimit":2000,"tests":[{"input":"2\n5\n\n4\n\n0\n\n1\n\n2\n\n2\n\n0\n","output":"? 1 5\n\n? 2 4\n\n? 4 5\n\n? 3 5\n\n! 01001\n\n? 1 2\n\n! IMPOSSIBLE\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKachinasFavoriteBinaryString"}}}

use std::io::Write;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn query(input: &mut Input, out: &mut Output, a: usize, b: usize) -> usize {
    out.print_line(format!("? {a} {b}"));
    out.flush();
    input.read()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut result = vec![b'1'; n];
    let mut prev = 0;
    for i in 1..n as usize {
        let curr = query(input, out, 1, i + 1);
        let delta = curr - prev;
        if prev > 0 {
            if delta == 0 {
                result[i] = b'0';
            } else {
                result[i] = b'1';
            }
        } else if curr > 0 {
            result[i] = b'1';
            for j in (i - delta)..i {
                result[j] = b'0';
            }
        }
        prev = curr;
    }
    if prev == 0 {
        out.print_line("! IMPOSSIBLE");
    } else {
        out.print_line(format!("! {}", String::from_utf8(result).unwrap()));
    }
    out.flush()
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
