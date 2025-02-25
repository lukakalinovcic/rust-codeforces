//{"name":"E. Eat the Chip","group":"Codeforces - Codeforces Round 920 (Div. 3)","url":"https://codeforces.com/contest/1921/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"12\n6 5 2 2 5 3\n4 1 2 1 4 1\n1 4 1 3 1 1\n5 5 1 4 5 2\n4 4 1 1 4 4\n10 10 1 6 10 8\n10 10 2 6 10 7\n10 10 9 1 8 1\n10 10 8 1 10 2\n10 10 1 1 2 1\n10 10 1 3 4 1\n10 10 3 1 1 1\n","output":"Alice\nBob\nDraw\nDraw\nDraw\nAlice\nDraw\nDraw\nBob\nAlice\nAlice\nDraw\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EEatTheChip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _h = input.read_int();
    let w = input.read_int();
    let ra = input.read_int();
    let ca = input.read_int();
    let rb = input.read_int();
    let cb = input.read_int();
    out.print_line(doit(w, ra, ca, rb, cb));
}

fn doit(w: i32, ra: i32, ca: i32, rb: i32, cb: i32) -> &'static str {
    if ra > rb {
        return "Draw";
    }
    let a_moves = (rb - ra + 1) / 2;
    let b_moves = (rb - ra) / 2;
    let a_lo = (ca - a_moves).max(1);
    let a_hi = (ca + a_moves).min(w);
    let b_lo = (cb - b_moves).max(1);
    let b_hi = (cb + b_moves).min(w);
    if (rb - ra) % 2 == 0 {
        if a_lo >= b_lo && a_hi <= b_hi {
            "Bob"
        } else {
            "Draw"
        }
    } else {
        if b_lo >= a_lo && b_hi <= a_hi {
            "Alice"
        } else {
            "Draw"
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
