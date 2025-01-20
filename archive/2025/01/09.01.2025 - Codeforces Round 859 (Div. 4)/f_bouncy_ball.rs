//{"name":"F. Bouncy Ball","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 7 1 7 2 4 DL\n5 7 1 7 3 2 DL\n3 3 1 3 2 2 UR\n2 4 2 1 2 2 DR\n4 3 1 1 1 3 UL\n6 4 1 2 3 4 DR\n","output":"3\n-1\n1\n-1\n4\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBouncyBall"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, m) = (input.read_int(), input.read_int());
    let (mut r, mut c) = (input.read_int(), input.read_int());
    let (r_goal, c_goal) = (input.read_int(), input.read_int());
    let d = input.read_line();
    let (mut dr, mut dc) = match d.as_str() {
        "DR" => (1, 1),
        "DL" => (1, -1),
        "UR" => (-1, 1),
        "UL" => (-1, -1),
        _ => (0, 0),
    };
    let mut seen = HashSet::new();
    let mut bounces = 0;
    for _ in 0.. {
        if (r, c) == (r_goal, c_goal) {
            out.print_line(bounces);
            return;
        } else if seen.contains(&(r, c, dr, dc)) {
            out.print_line(-1);
            return;
        } else {
            seen.insert((r, c, dr, dc));
        }
        let mut add_bounce = false;
        if r + dr < 1 || r + dr > n {
            dr = -dr;
            add_bounce = true;
        }
        if c + dc < 1 || c + dc > m {
            dc = -dc;
            add_bounce = true;
        }
        if add_bounce {
            bounces += 1;
        }
        r += dr;
        c += dc;
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
