//{"name":"E. Unique Array","group":"Codeforces - Educational Codeforces Round 165 (Rated for Div. 2)","url":"https://codeforces.com/contest/1969/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n2 1 2\n4\n4 4 4 4\n5\n3 1 2 1 2\n5\n1 3 2 1 2\n","output":"0\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EUniqueArray"}}}

use std::collections::HashMap;
use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut m = n as i32 + 1;
    let a = input.read_int_vec(n);
    let mut prev: HashMap<i32, usize> = HashMap::new();
    let mut active = VecDeque::new();
    let mut result = 0;
    eprintln!("sovling {a:?}");
    for i in 0..n {
        let mut curr = a[i];
        let p = prev.get(&curr).cloned();
        match p {
            None => {}
            Some(p) => {
                while !active.is_empty() && *active.front().unwrap() <= p {
                    active.pop_front();
                }
                if active.is_empty() {
                    result += 1;
                    curr = m;
                    m = m + 1;
                } else {
                }
            }
        }
        active.push_back(i);
        prev.insert(curr, i);
        eprintln!("{active:?} {result}");
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
