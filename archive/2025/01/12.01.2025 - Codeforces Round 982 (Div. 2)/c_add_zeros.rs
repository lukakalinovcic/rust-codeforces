//{"name":"C. Add Zeros","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n5\n2 4 6 2 5\n5\n5 4 4 5 1\n4\n6 8 2 3\n1\n1\n","output":"10\n11\n10\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CAddZeros"}}}

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut targets = HashMap::<i64, Vec<i64>>::new();
    for i in 0..n {
        let target_size = a[i] + i as i64;
        targets.entry(target_size).or_default().push(i as i64);
    }

    let mut result = 0;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back(a.len() as i64);
    while !q.is_empty() {
        let curr = q.pop_front().unwrap();
        result = result.max(curr);
        if let Some(targets) = targets.get(&curr) {
            for i in targets {
                let i = *i;
                if seen.insert(curr + i) {
                    q.push_back(curr + i);
                }
            }
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
