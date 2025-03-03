//{"name":"C. Creating Keys for StORages Has Become My Main Skill","group":"Codeforces - Codeforces Round 1006 (Div. 3)","url":"https://codeforces.com/contest/2072/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n1 69\n7 7\n5 7\n7 3\n8 7\n3 52\n9 11\n6 15\n2 3\n","output":"69\n6 0 3 4 1 2 5\n4 1 3 0 2\n0 1 2 3 2 1 0\n7 0 6 1 5 2 4 3\n0 52 0\n0 1 8 3 0 9 11 2 10\n4 0 3 8 1 2\n0 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCreatingKeysForStORagesHasBecomeMyMainSkill"}}}

use std::ops::Not;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let x = input.read_unsigned();
    let mut curr = 0;
    let mut result_or = 0;
    let mut result = vec![];
    while result.len() < n {
        if curr == x {
            result.push(x);
        } else {
            result.push(curr);
            curr = (curr + x.not() + 1) & x;
        }
        result_or |= *result.last().unwrap();
    }
    if result_or != x {
        *result.last_mut().unwrap() = x;
    }
    out.print_line(result);
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
