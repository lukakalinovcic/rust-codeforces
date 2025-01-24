//{"name":"C. Nene's Magical Matrix","group":"Codeforces - Codeforces Round 939 (Div. 2)","url":"https://codeforces.com/contest/1956/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1\n2\n","output":"1 1\n1 1 1\n7 3\n1 1 1 2\n1 2 1 2\n2 1 1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNenesMagicalMatrix"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut sum = 0;
    for i in 1..=n {
        sum += i * (2 * i - 1);
    }
    out.print_line((sum, 2 * n));
    for i in (1..=n).rev() {
        for t in 1..=2 {
            out.print((t, i));
            out.print(" ");
            out.print_iter(1..=n);
            out.print_line("");
        }
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
