//{"name":"D. Digital string maximization","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n19\n1709\n11555\n51476\n9876543210\n5891917899\n","output":"81\n6710\n33311\n55431\n9876543210\n7875567711\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDigitalStringMaximization"}}}

use std::collections::BTreeSet;
use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn encode(ch: u8, i: usize, offset: bool) -> (i32, usize) {
    ((ch - b'0') as i32 - if offset { i as i32 } else { 0 }, i)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut s = input.read_line().into_bytes();
    let n = s.len();
    for i in 0..n {
        let mut best = (b'0' - 1, 0);
        for j in 0..9 {
            if i + j >= n {
                break;
            }
            let cand = (s[i + j] - j as u8, -(j as i32));
            best = best.max(cand);
        }
        for j in (0..(-best.1) as usize).rev() {
            s[i + j + 1] = s[i + j];
        }
        s[i] = best.0;
    }
    out.print_line(String::from_utf8(s).unwrap());
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
