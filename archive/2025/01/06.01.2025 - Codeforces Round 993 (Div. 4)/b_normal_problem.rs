//{"name":"B. Normal Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\nqwq\nppppp\npppwwwqqq\nwqpqwpqwwqp\npqpqpqpq\n","output":"pwp\nqqqqq\npppwwwqqq\nqpwwpqwpqpw\npqpqpqpq\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNormalProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line();
    for ch in s.as_bytes().iter().rev() {
        match *ch {
            b'w' => out.put(b'w'),
            b'q' => out.put(b'p'),
            b'p' => out.put(b'q'),
            _ => (),
        }
    }
    out.put(b'\n')
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
