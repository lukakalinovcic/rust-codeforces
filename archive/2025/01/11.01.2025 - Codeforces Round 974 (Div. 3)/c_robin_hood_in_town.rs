//{"name":"C. Robin Hood in Town","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1\n2\n2\n2 19\n3\n1 3 20\n4\n1 2 3 4\n5\n1 2 3 4 5\n6\n1 2 1 1 1 25\n","output":"-1\n-1\n0\n15\n16\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobinHoodInTown"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_long_vec(n);
    a.sort();
    if n < 3 {
        out.print_line(-1);
        return;
    }
    let total: i64 = a.iter().sum();
    let x = a[n / 2];
    let target = 2 * x * (n as i64) + 1;
    out.print_line((target - total).max(0));
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
