//{"name":"B1. The Strict Teacher (Easy Version)","group":"Codeforces - Codeforces Round 972 (Div. 2)","url":"https://codeforces.com/contest/2005/problem/B1","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n10 2 1\n1 4\n2\n8 2 1\n3 6\n1\n8 2 1\n3 6\n8\n","output":"1\n2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B1TheStrictTeacherEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read();
    let q = input.read();
    let mut b = input.read_int_vec(m);
    b.sort();
    let mut result = vec![];
    for a in input.read_int_vec(q) {
        let (Ok(i) | Err(i)) = b.binary_search(&a);
        if i > 0 && i < b.len() {
            let x = b[i - 1];
            let y = b[i];
            result.push((y - x) / 2);
        } else if i > 0 {
            let x = b[i - 1];
            result.push(n - x);
        } else {
            let y = b[i];
            result.push(y - 1);
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
