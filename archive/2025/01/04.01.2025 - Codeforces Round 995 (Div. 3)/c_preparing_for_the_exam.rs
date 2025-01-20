//{"name":"C. Preparing for the Exam","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n4 4 3\n1 2 3 4\n1 3 4\n5 4 3\n1 2 3 4\n1 3 4\n4 4 4\n1 2 3 4\n1 2 3 4\n2 2 1\n1 2\n2\n","output":"0100\n0000\n1111\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPreparingForTheExam"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, m, k) = (input.read_int(), input.read_int(), input.read_int());
    let a: Vec<_> = (0..m).map(|_| input.read_int()).collect();
    let q: Vec<_> = (0..k).map(|_| input.read_int()).collect();
    if k < n - 1 {
        for _ in 0..m {
            out.print('0');
        }
    } else if k == n {
        for _ in 0..m {
            out.print('1');
        }
    } else {
        let q: HashSet<_> = q.into_iter().collect();
        for aa in a.into_iter() {
            if q.contains(&aa) {
                out.print('0')
            } else {
                out.print('1')
            }
        }
    }
    out.put(b'\n');
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
