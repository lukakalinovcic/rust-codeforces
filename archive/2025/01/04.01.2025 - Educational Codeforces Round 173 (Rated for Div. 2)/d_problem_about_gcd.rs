//{"name":"D. Problem about GCD","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 8 2\n4 8 3\n4 8 4\n5 7 6\n","output":"4 6\n-1 -1\n4 8\n6 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProblemAboutGCD"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (l, r, g) = (input.read_long(), input.read_long(), input.read_long());
    let mut l_cands = vec![];
    let mut i = ((l + g - 1) / g) * g;
    for _ in 0..25 {
        if i <= r {
            l_cands.push(i);
        }
        i += g;
    }
    let mut r_cands = vec![];
    let mut i = (r / g) * g;
    for _ in 0..25 {
        if i >= l {
            r_cands.push(i);
        }
        i -= g;
    }
    let mut result = None;
    for a in l_cands.iter() {
        let a = *a;
        for b in r_cands.iter() {
            let b = *b;
            if a > b || gcd(a, b) != g {
                continue;
            }
            result = match result {
                None => Some((a, b)),
                Some((r_a, r_b)) => {
                    let d = b - a;
                    let r_d = r_b - r_a;
                    if d > r_d || d == r_d && a < r_a {
                        Some((a, b))
                    } else {
                        Some((r_a, r_b))
                    }
                }
            };
        }
    }
    let (a, b) = result.unwrap_or((-1, -1));
    out.print_iter([a, b].into_iter());
    out.put(b'\n');
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
