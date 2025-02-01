//{"name":"B. Turtle and an Infinite Sequence","group":"Codeforces - Codeforces Round 949 (Div. 2)","url":"https://codeforces.com/contest/1981/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n0 0\n0 1\n0 2\n1 0\n5 2\n10 1\n20 3\n1145 14\n19198 10\n","output":"0\n1\n3\n1\n7\n11\n23\n1279\n19455\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTurtleAndAnInfiniteSequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let lo = (n - m).max(0);
    let hi = n + m;
    let mut result = 0;
    for p in 0..=30 {
        if hi - lo >= (1 << p) || (hi >> p) & 1 == 1 || (lo >> p) & 1 == 1 {
            result |= 1 << p;
        }
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
