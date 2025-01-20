//{"name":"C. Concatenation of Arrays","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 4\n2 3\n3\n3 2\n4 3\n2 1\n5\n5 10\n2 3\n9 6\n4 1\n8 7\n1\n10 20\n","output":"2 3 1 4\n2 1 3 2 4 3\n4 1 2 3 5 10 8 7 9 6\n10 20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CConcatenationOfArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut p = input.read_int_pair_vec(n);
    p.sort_by_key(|(x, y)| x + y);
    let mut result = vec![];
    for i in 0..n {
        result.push(p[i].0);
        result.push(p[i].1);
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
