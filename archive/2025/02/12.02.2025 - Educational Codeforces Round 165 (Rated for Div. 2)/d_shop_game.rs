//{"name":"D. Shop Game","group":"Codeforces - Educational Codeforces Round 165 (Rated for Div. 2)","url":"https://codeforces.com/contest/1969/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 0\n2 1\n1 2\n4 1\n1 2 1 4\n3 3 2 3\n4 2\n2 1 1 1\n4 2 3 2\n6 2\n1 3 4 9 1 3\n7 6 8 10 6 8\n","output":"1\n1\n0\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DShopGame"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let mut d = vec![];
    for i in 0..n {
        if b[i] > a[i] {
            d.push((b[i], a[i], b[i] - a[i]));
        }
    }
    d.sort();
    d.reverse();
    let n = d.len();
    let mut suffix_sum = vec![0; n + 1];
    for i in (0..n).rev() {
        suffix_sum[i] = suffix_sum[i + 1] + d[i].2 as i64;
    }
    let mut result = 0;
    let mut sum_k = 0;
    let mut selected = BinaryHeap::new();
    for i in 0..n {
        if i >= k {
            result = result.max(suffix_sum[i] - sum_k);
        }
        sum_k += d[i].1 as i64;
        selected.push(d[i].1 as i64);
        if i >= k {
            sum_k -= selected.pop().unwrap();
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
