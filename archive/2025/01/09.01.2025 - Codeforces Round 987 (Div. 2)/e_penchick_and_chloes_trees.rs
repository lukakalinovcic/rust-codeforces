//{"name":"E. Penchick and Chloe's Trees","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/contest/2031/problem/E","interactive":false,"timeLimit":3500,"tests":[{"input":"5\n6\n1 2 2 1 1\n15\n1 1 2 2 3 3 4 4 5 5 6 6 7 7\n5\n1 2 2 2\n7\n1 1 2 1 1 2\n10\n1 1 1 2 2 2 4 3 3\n","output":"2\n3\n3\n3\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPenchickAndChloesTrees"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve_kids(kids: &mut Vec<i32>) -> i32 {
    if kids.len() == 0 {
        return 0;
    } else if kids.len() == 1 {
        return 1 + kids[0];
    }
    let mut pq = BinaryHeap::new();
    for x in kids {
        pq.push(-*x);
    }
    while pq.len() > 1 {
        let a = -pq.pop().unwrap();
        let b = -pq.pop().unwrap();
        pq.push(-(1 + a.max(b)));
    }
    -pq.pop().unwrap()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n - 1);
    let mut kids = vec![vec![]; n];
    for i in (0..n).rev() {
        let d = solve_kids(&mut kids[i]);
        if i > 0 {
            kids[p[i - 1] as usize - 1].push(d);
        } else {
            out.print_line(d);
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
