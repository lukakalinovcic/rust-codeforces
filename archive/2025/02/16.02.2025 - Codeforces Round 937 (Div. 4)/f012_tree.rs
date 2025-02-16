//{"name":"F. 0, 1, 2, Tree!","group":"Codeforces - Codeforces Round 937 (Div. 4)","url":"https://codeforces.com/contest/1950/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n2 1 3\n0 0 1\n0 1 1\n1 0 2\n1 1 3\n3 1 4\n8 17 9\n24 36 48\n1 0 0\n0 3 1\n","output":"2\n0\n1\n1\n-1\n3\n6\n-1\n-1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F012Tree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut a = input.read_int();
    let leaves = a + 1;
    let mut b = input.read_int();
    let c = input.read_int();
    if c != leaves {
        out.print_line(-1);
        return;
    }
    let mut h = 0;
    let mut p2 = 1;
    while a > 0 {
        a -= p2;
        p2 *= 2;
        h += 1;
    }
    b += a;
    if b > 0 {
        h += (b + leaves - 1) / (leaves);
    }
    out.print_line(h);
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
