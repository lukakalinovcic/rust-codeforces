//{"name":"C. Find and Replace","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n7\nabacaba\n2\naa\n1\ny\n4\nbkpt\n6\nninfia\n6\nbanana\n10\ncodeforces\n8\ntestcase\n","output":"YES\nNO\nYES\nYES\nNO\nYES\nNO\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFindAndReplace"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn try_alternating(s: &[u8], start: i32) -> bool {
    let mut assigned = HashMap::new();
    for i in 0..s.len() {
        let digit = (start + i as i32) % 2;
        if let Some(assigned) = assigned.get(&s[i]) {
            if *assigned != digit {
                return false;
            }
        } else {
            assigned.insert(s[i], digit);
        }
    }
    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n: usize = input.read();
    let s = input.read_line().into_bytes();
    if try_alternating(&s, 0) || try_alternating(&s, 1) {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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
