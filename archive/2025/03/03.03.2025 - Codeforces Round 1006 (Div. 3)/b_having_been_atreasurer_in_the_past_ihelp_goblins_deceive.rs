//{"name":"B. Having Been a Treasurer in the Past, I Help Goblins Deceive","group":"Codeforces - Codeforces Round 1006 (Div. 3)","url":"https://codeforces.com/contest/2072/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3\n--_\n5\n__-__\n9\n--__-_---\n4\n_--_\n10\n_-_-_-_-_-\n7\n_------\n1\n-\n2\n_-\n","output":"1\n0\n27\n2\n30\n9\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BHavingBeenATreasurerInThePastIHelpGoblinsDeceive"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_line().into_bytes();
    let mut a = 0;
    let mut b = 0;
    for i in 0..n {
        if s[i] == b'_' {
            a += 1 as i64;
        } else {
            b += 1 as i64;
        }
    }
    let b1 = b / 2;
    let b2 = b - b1;
    out.print_line(a * b1 * b2);
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
