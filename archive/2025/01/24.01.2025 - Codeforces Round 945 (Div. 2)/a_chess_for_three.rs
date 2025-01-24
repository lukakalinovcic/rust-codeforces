//{"name":"A. Chess For Three","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n0 0 0\n0 1 1\n1 1 1\n1 1 2\n3 3 3\n3 4 5\n1 1 10\n","output":"0\n1\n-1\n2\n-1\n6\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AChessForThree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut a = input.read_int();
    let mut b = input.read_int();
    let mut c = input.read_int();
    let mut draws = 0;
    while b >= 1 {
        c -= 1;
        b -= 1;
        draws += 1;

        let mut x = vec![a, b, c];
        x.sort();
        (a, b, c) = (x[0], x[1], x[2]);
    }
    if c % 2 == 1 {
        out.print_line(-1);
    } else {
        out.print_line(draws);
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
