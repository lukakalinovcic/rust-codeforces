//{"name":"A. Rectangle Arrangement","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 5\n2 4\n3 3\n4 2\n5 1\n3\n2 2\n1 1\n1 2\n1\n3 2\n3\n100 100\n100 100\n100 100\n4\n1 4\n2 3\n1 5\n3 2\n","output":"20\n8\n10\n400\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARectangleArrangement"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut maxw = 0;
    let mut maxh = 0;
    for (w, h) in input.read_int_pair_vec(n) {
        maxw = maxw.max(w);
        maxh = maxh.max(h);
    }
    out.print_line(2 * maxw + 2 * maxh);
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
