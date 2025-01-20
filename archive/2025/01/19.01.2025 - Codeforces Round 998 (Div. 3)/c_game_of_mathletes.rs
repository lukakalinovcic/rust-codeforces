//{"name":"C. Game of Mathletes","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n1 2 3 2\n8 15\n1 2 3 4 5 6 7 8\n6 1\n1 1 1 1 1 1\n16 9\n3 1 4 1 5 9 2 6 5 3 5 8 9 7 9 3\n","output":"2\n1\n0\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CGameOfMathletes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_int();
    let mut a = input.read_int_vec(n);
    a.sort();
    let mut i = 0;
    let mut result = 0;
    for j in (0..n).rev() {
        while i < j && a[i] + a[j] < k {
            i += 1;
        }
        if i < j && a[i] + a[j] == k {
            result += 1;
            i += 1;
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
