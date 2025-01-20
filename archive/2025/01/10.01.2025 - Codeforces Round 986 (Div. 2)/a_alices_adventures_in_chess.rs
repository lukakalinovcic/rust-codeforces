//{"name":"A. Alice's Adventures in \"Chess\"","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2 2 2\nNE\n3 2 2\nNNE\n6 2 1\nNNEESW\n6 10 10\nNNEESW\n3 4 2\nNEE\n4 5 5\nNEWS\n","output":"YES\nNO\nYES\nYES\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAlicesAdventuresInChess"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn deltas(ch: u8) -> (i32, i32) {
    match ch {
        b'N' => (0, 1),
        b'E' => (1, 0),
        b'S' => (0, -1),
        b'W' => (-1, 0),
        _ => panic!("unexpected char"),
    }
}

fn num_cycles(target: i32, cycle_len: i32) -> Option<i32> {
    if cycle_len == 0 || target % cycle_len != 0 {
        None
    } else {
        Some(target / cycle_len)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n: usize = input.read();
    let mut x = input.read_int();
    let mut y = input.read_int();
    let s = input.read_line().into_bytes();
    let mut cycle_dx = 0;
    let mut cycle_dy = 0;
    for ch in s.iter() {
        let (dx, dy) = deltas(*ch);
        cycle_dx += dx;
        cycle_dy += dy;
    }
    for ch in s {
        let done = match (num_cycles(x, cycle_dx), num_cycles(y, cycle_dy)) {
            (None, None) => x == 0 && y == 0,
            (None, Some(cycles_y)) => (x == 0 && cycle_dx == 0) && cycles_y >= 0,
            (Some(cycles_x), None) => cycles_x >= 0 && (y == 0 && cycle_dy == 0),
            (Some(cycles_x), Some(cycles_y)) => cycles_x == cycles_y && cycles_x >= 0,
        };
        if done {
            out.print_line("YES");
            return;
        }
        let (dx, dy) = deltas(ch);
        x -= dx;
        y -= dy;
    }
    out.print_line("NO");
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
