//{"name":"C. Perform Operations to Maximize Score","group":"Codeforces - Codeforces Round 965 (Div. 2)","url":"https://codeforces.com/contest/1998/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n2 10\n3 3\n1 1\n3 10\n3 3 3\n0 0 0\n4 4\n2 1 5 1\n0 1 0 1\n5 4\n7 5 2 5 4\n0 0 1 0 1\n5 1\n5 15 15 2 11\n1 0 0 1 1\n5 2\n10 11 4 10 15\n1 1 0 1 0\n4 4\n1 1 2 5\n1 1 0 0\n2 1000000000\n1000000000 1000000000\n1 1\n","output":"16\n6\n8\n13\n21\n26\n8\n3000000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPerformOperationsToMaximizeScore"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    let b = input.read_int_vec(n);
    let mut max_b0 = None;
    let mut max_b1 = None;
    for i in 0..n {
        if b[i] == 0 {
            max_b0 = match max_b0 {
                None => Some(i),
                Some(j) => Some(if a[i] > a[j] { i } else { j }),
            }
        } else {
            max_b1 = match max_b1 {
                None => Some(i),
                Some(j) => Some(if a[i] > a[j] { i } else { j }),
            }
        }
    }
    let mut result = 0;
    if let Some(i) = max_b1 {
        let ai = a[i];
        let mut a = a.clone();
        let mut b = b.clone();
        a.remove(i);
        b.remove(i);
        result = result.max(ai + k + solve_median(a, b, 0));
    }
    if let Some(i) = max_b0 {
        let ai = a[i];
        let mut a = a.clone();
        let mut b = b.clone();
        a.remove(i);
        b.remove(i);
        result = result.max(ai + solve_median(a, b, k));
    }
    out.print_line(result);
}

fn solve_median(a: Vec<i64>, b: Vec<i32>, k: i64) -> i64 {
    let mut ab = a.into_iter().zip(b).collect::<Vec<_>>();
    ab.sort();
    ab.reverse();
    let n = ab.len();
    let mut lo = 0;
    let mut hi = 2000000000;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;

        let mut cnt_gte = 0;
        for i in 0..n {
            if ab[i].0 >= mid {
                cnt_gte += 1;
            }
        }
        let mut need = (n as i64 + 2) / 2;
        need -= cnt_gte;
        if need <= 0 {
            lo = mid;
            continue;
        }
        let mut cost = 0;
        for i in 0..n {
            if need > 0 && ab[i].1 == 1 && ab[i].0 < mid {
                cost += mid - ab[i].0;
                need -= 1;
            }
        }
        if need > 0 || cost > k {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }
    lo
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
