//{"name":"C. Permutation Counting","group":"Codeforces - Codeforces Round 942 (Div. 2)","url":"https://codeforces.com/contest/1972/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n1 10\n1\n2 4\n8 4\n3 4\n6 1 8\n3 9\n7 6 2\n5 3\n6 6 7 4 6\n9 7\n7 6 1 7 6 2 4 3 3\n10 10\n1 3 1 2 1 9 3 5 7 5\n9 8\n5 8 7 5 1 3 2 9 8\n","output":"11\n15\n15\n22\n28\n32\n28\n36\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPermutationCounting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    let mut mini = a[0];
    for i in 0..n {
        mini = mini.min(a[i]);
    }
    let mut lo: i64 = 0;
    let mut hi: i64 = mini + k;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        let mut cost = 0;
        for i in 0..n {
            cost += (mid - a[i]).max(0);
        }
        if cost <= k {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    let mut cost = 0;
    let mut rem = 0;
    for i in 0..n {
        cost += (lo - a[i]).max(0);
        if a[i] > lo {
            rem += 1;
        }
    }
    rem += k - cost;

    out.print_line(lo * n as i64 - n as i64 + 1 + rem);
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
