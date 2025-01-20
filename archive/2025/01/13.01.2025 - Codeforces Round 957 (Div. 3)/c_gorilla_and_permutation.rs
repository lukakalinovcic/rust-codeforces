//{"name":"C. Gorilla and Permutation","group":"Codeforces - Codeforces Round 957 (Div. 3)","url":"https://codeforces.com/contest/1992/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 2 5\n3 1 3\n10 3 8\n","output":"5 3 4 1 2\n3 2 1\n10 9 8 4 7 5 6 1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGorillaAndPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let k = input.read_int();
    let mut result = vec![];
    for i in (k..=n).rev() {
        result.push(i);
    }
    for i in m + 1..k {
        result.push(i);
    }
    for i in 1..=m {
        result.push(i);
    }
    out.print_iter(result.into_iter());
    out.print("\n");
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
