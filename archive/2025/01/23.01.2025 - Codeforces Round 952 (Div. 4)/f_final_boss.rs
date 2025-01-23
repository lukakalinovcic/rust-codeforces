//{"name":"F. Final Boss","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3 2\n2 1\n2 1\n5 2\n2 1\n2 1\n50 3\n5 6 7\n5 6 7\n50 3\n2 2 2\n3 3 3\n90000 2\n200000 200000\n1 1\n100000 1\n1\n200000\n6 7\n3 2 3 2 3 1 2\n6 5 9 5 10 7 7\n21 6\n1 1 1 1 1 1\n5 5 8 10 7 6\n","output":"1\n3\n15\n25\n1\n19999800001\n1\n21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FFinalBoss"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut h = input.read_int();
    let n = input.read();
    let a = input.read_int_vec(n);
    let c = input.read_int_vec(n);

    let mut pq = BinaryHeap::new();
    for i in 0..n {
        pq.push((-1 as i64, i));
    }
    loop {
        let (t, i) = pq.pop().unwrap();
        let t = -t;
        h -= a[i];
        if h <= 0 {
            out.print_line(t);
            break;
        }
        pq.push((-(t + c[i] as i64), i));
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
