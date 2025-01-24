//{"name":"B. Cat, Fox and the Lonely Array","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n0\n3\n2 2 2\n3\n1 0 2\n5\n3 0 1 4 2\n5\n2 0 4 0 2\n7\n0 0 0 0 1 2 4\n8\n0 1 3 2 2 1 0 3\n","output":"1\n1\n3\n4\n4\n7\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCatFoxAndTheLonelyArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut result = 1;
    for bit in 0..30 {
        let mut prev = -1;
        for i in 0..n {
            let b = (a[i] >> bit) & 1;
            if b == 1 {
                let curr = i as i32;
                result = result.max(curr - prev);
                prev = curr;
            }
        }
        if prev != -1 {
            let curr = n as i32;
            result = result.max(curr - prev);
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
