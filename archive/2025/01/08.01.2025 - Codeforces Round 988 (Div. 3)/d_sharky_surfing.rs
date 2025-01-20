//{"name":"D. Sharky Surfing","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/contest/2037/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 5 50\n7 14\n30 40\n2 2\n3 1\n3 5\n18 2\n22 32\n4 3 50\n4 6\n15 18\n20 26\n34 38\n1 2\n8 2\n10 2\n1 4 17\n10 14\n1 6\n1 2\n1 2\n16 9\n1 2 10\n5 9\n2 3\n2 2\n","output":"4\n-1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSharkySurfing"}}}

use std::collections::BTreeSet;
use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let target = input.read_int();
    let mut hurdles = VecDeque::from_iter(input.read_int_pair_vec(n).into_iter());
    let mut powerups = VecDeque::from_iter(input.read_int_pair_vec(m).into_iter());
    let mut available_powerups = BTreeSet::new();

    let mut power = 1;
    let mut took = 0;
    let mut powerup_index = 0;
    while !hurdles.is_empty() {
        let (hurdle_lo, hurdle_hi) = hurdles.pop_front().unwrap();
        let need_power = hurdle_hi - hurdle_lo + 2;
        while !powerups.is_empty() && powerups.front().unwrap().0 < hurdle_lo {
            let (_, powerup) = powerups.pop_front().unwrap();
            powerup_index += 1;
            available_powerups.insert((powerup, powerup_index));
        }
        while !available_powerups.is_empty() && power < need_power {
            let (powerup, _) = available_powerups.pop_last().unwrap();
            power += powerup;
            took += 1;
        }
        if power < need_power {
            out.print_line(-1);
            return;
        }
    }
    out.print_line(took);
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
