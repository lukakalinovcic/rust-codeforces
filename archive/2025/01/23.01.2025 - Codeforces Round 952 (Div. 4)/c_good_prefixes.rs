//{"name":"C. Good Prefixes","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n0\n1\n1\n4\n1 1 2 0\n5\n0 1 2 1 4\n7\n1 1 0 3 5 2 12\n7\n1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 294967296\n10\n0 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 589934592\n","output":"1\n0\n3\n3\n4\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGoodPrefixes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut result = 0;
    let mut sum = 0;
    let mut mx = 0;
    for a in input.read_long_vec(n) {
        sum += a;
        mx = mx.max(a);
        if mx == sum - mx {
            result += 1;
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
