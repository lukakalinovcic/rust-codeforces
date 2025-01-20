//{"name":"B. Intercepted Inputs","group":"Codeforces - Codeforces Round 988 (Div. 3)","url":"https://codeforces.com/contest/2037/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n1 1 2\n11\n3 3 4 5 6 7 8 9 9 10 11\n8\n8 4 8 3 8 2 8 1\n6\n2 1 4 5 3 3\n8\n1 2 6 3 8 5 5 3\n","output":"1 1\n3 3\n2 3\n4 1\n1 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInterceptedInputs"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let k = input.read_int();
    let a = input.read_int_vec(k as usize);
    let mut cnt = HashMap::new();
    for x in a {
        let entry = cnt.entry(x).or_insert(0);
        *entry = *entry + 1;
    }
    for a in 1..k {
        if (k - 2) % a == 0 {
            let b = (k - 2) / a;
            if a == b && cnt.get(&a).cloned().unwrap_or_default() >= 2 {
                out.print_line(format!("{a} {b}"));
                return;
            } else if a != b
                && cnt.get(&a).cloned().unwrap_or_default() >= 1
                && cnt.get(&b).cloned().unwrap_or_default() >= 1
            {
                out.print_line(format!("{a} {b}"));
                return;
            }
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
