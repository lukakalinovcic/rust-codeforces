//{"name":"G1. Yunli's Subarray Queries (easy version)","group":"Codeforces - Codeforces Round 971 (Div. 4)","url":"https://codeforces.com/contest/2009/problem/G1","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n7 5 3\n1 2 3 2 1 2 3\n1 5\n2 6\n3 7\n8 4 2\n4 3 1 1 2 4 3 2\n3 6\n2 5\n5 4 2\n4 5 1 2 3\n1 4\n2 5\n","output":"2\n3\n2\n2\n2\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1YunlisSubarrayQueriesEasyVersion"}}}

use std::collections::BTreeSet;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn window_modes(a: Vec<i32>, k: usize) -> Vec<i32> {
    let mut result = vec![];
    let mut cnt = HashMap::<i32, i32>::new();
    let mut pq = BTreeSet::<(i32, i32)>::new();
    for i in 0..a.len() {
        {
            let a = a[i];
            let c = cnt.entry(a).or_insert(0);
            pq.remove(&(*c, a));
            *c += 1;
            pq.insert((*c, a));
        }
        if i >= k - 1 {
            result.push(pq.last().unwrap().0);

            let a = a[i - (k - 1)];
            let c = cnt.entry(a).or_insert(0);
            pq.remove(&(*c, a));
            *c -= 1;
            pq.insert((*c, a));
        }
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, a)| a - i as i32)
        .collect();
    let modes = window_modes(a, k);
    for (l, r) in input.read_int_pair_vec(q) {
        let (l, _r) = (l as usize - 1, r as usize - 1);
        out.print_line(k as i32 - modes[l]);
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
