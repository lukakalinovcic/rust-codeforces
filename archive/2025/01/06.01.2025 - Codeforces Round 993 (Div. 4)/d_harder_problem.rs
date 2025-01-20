//{"name":"D. Harder Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n4\n1 1 1 2\n8\n4 5 5 5 1 1 2 1\n10\n1 1 2 2 1 1 3 3 1 1\n","output":"1 2\n1 1 2 2\n4 5 5 1 1 2 2 3\n1 8 2 2 1 3 3 9 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DHarderProblem"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = input.read_int_vec(n);
    let mut c = vec![0; n + 1];
    for x in a.iter() {
        let x = *x;
        c[x as usize] += 1;
    }
    let mut free = VecDeque::new();
    for i in 1..=n {
        if c[i] == 0 {
            free.push_back(i as i32);
        }
    }
    let mut c = vec![0; n + 1];
    let mut result = vec![];
    for x in a.iter() {
        let x = *x;
        c[x as usize] += 1;
        if c[x as usize] == 1 {
            result.push(x);
        } else {
            let y = free.pop_front().unwrap();
            result.push(y);
        }
    }
    out.print_iter(result.into_iter());
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
