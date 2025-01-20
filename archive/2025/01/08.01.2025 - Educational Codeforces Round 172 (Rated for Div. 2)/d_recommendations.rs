//{"name":"D. Recommendations","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/contest/2042/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n3 8\n2 5\n4 5\n2\n42 42\n1 1000000000\n3\n42 42\n1 1000000000\n42 42\n6\n1 10\n3 10\n3 7\n5 7\n4 4\n1 2\n","output":"0\n0\n1\n999999999\n0\n0\n0\n0\n0\n2\n3\n2\n4\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRecommendations"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn doit(a: Vec<(i32, i32)>) -> Vec<i32> {
    let n = a.len();
    let mut a: Vec<_> = a
        .into_iter()
        .enumerate()
        .map(|(i, (lo, hi))| (lo, -hi, i))
        .collect();
    a.sort();
    let mut ends = BTreeSet::new();
    let mut result = vec![0; a.len()];
    for k in 0..n {
        let (lo, minus_hi, i) = a[k];
        let hi = -minus_hi;
        let first_end_after_hi = if k + 1 < n && a[k + 1].0 == lo && a[k + 1].1 == minus_hi {
            hi
        } else {
            match ends.range(hi..).next() {
                Some(end) => *end,
                None => hi,
            }
        };
        result[i] += first_end_after_hi - hi;
        ends.insert(hi);
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_pair_vec(n);
    let after = doit(a.clone());
    let a = a.into_iter().map(|(lo, hi)| (-hi, -lo)).collect();
    let before = doit(a);

    out.print_iter(
        after
            .into_iter()
            .zip(before.into_iter())
            .map(|(a, b)| a + b),
    );
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
