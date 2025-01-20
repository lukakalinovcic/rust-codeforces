//{"name":"B. Scale","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n0000\n0000\n0000\n0000\n6 3\n000111\n000111\n000111\n111000\n111000\n111000\n6 2\n001100\n001100\n111111\n111111\n110000\n110000\n8 1\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n","output":"0\n01\n10\n010\n111\n100\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n11111111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BScale"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let a: Vec<_> = (0..n).map(|_| input.read_line().into_bytes()).collect();
    for row in a.into_iter().step_by(k) {
        out.print_line(String::from_utf8(row.into_iter().step_by(k).collect()).unwrap());
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
