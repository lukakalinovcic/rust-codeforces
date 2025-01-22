//{"name":"E. Triple Operations","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 3\n2 4\n199999 200000\n19 84\n","output":"5\n6\n36\n263\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETripleOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_int();
    let r = input.read_int();
    let a = l as i64;
    let b = l as i64 + 1;
    let mut result = solve2(a, b).min(solve2(b, a));
    let l = l + 2;
    let mut c3 = 2;
    let mut p3 = 3;
    while p3 <= r {
        let lo = l.max(p3);
        let hi = r.min(p3 * 3 - 1);
        if lo <= hi {
            result += (hi - lo + 1) * c3;
        }
        c3 += 1;
        p3 *= 3;
    }
    out.print_line(result);
}

fn solve2(mut a: i64, mut b: i64) -> i32 {
    let mut moves = 0;
    while a > 0 {
        a /= 3;
        b *= 3;
        moves += 1;
    }
    moves + solve1(b)
}

fn solve1(mut a: i64) -> i32 {
    let mut moves = 0;
    while a > 0 {
        a /= 3;
        moves += 1;
    }
    moves
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
