//{"name":"D. Move Back at a Cost","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n2 1 3\n5\n1 2 2 1 4\n6\n1 2 3 6 5 4\n","output":"1 3 3\n1 1 3 3 5\n1 2 3 4 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMoveBackAtACost"}}}

use std::collections::BTreeSet;
use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 2000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut main = VecDeque::new();
    let mut rest = BTreeSet::new();
    for (i, x) in a.into_iter().enumerate() {
        while main.back().cloned().unwrap_or((-1, n)).0 > x {
            rest.insert(main.pop_back().unwrap());
        }
        main.push_back((x, i));
    }
    while main.back().cloned().unwrap_or((-1, n)).0
        > rest.first().cloned().unwrap_or((INF, n)).0 + 1
    {
        rest.insert(main.pop_back().unwrap());
    }
    out.print_iter(main.into_iter().map(|(x, _)| x));
    out.put(b' ');
    out.print_iter(rest.into_iter().map(|(x, _)| x + 1));
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
