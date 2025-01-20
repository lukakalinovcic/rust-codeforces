//{"name":"A. Preparing for the Olympiad","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n3 2\n2 1\n1\n5\n8\n3\n1 1 1\n2 2 2\n6\n8 2 5 6 2 6\n8 2 7 4 3 4\n","output":"4\n5\n1\n16\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APreparingForTheOlympiad"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let mut result = 0;
    let a: Vec<i32> = (0..n).map(|_| input.read_int()).collect();
    let b: Vec<i32> = (0..n)
        .map(|_| input.read_int())
        .skip(1)
        .chain(Some(0).into_iter())
        .collect();
    for (aa, bb) in a.into_iter().zip(b.into_iter()) {
        result += 0.max(aa - bb);
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
