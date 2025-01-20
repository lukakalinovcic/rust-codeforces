//{"name":"D. Counting Pairs","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4 8 10\n4 6 3 6\n6 22 27\n4 9 6 3 4 5\n3 8 10\n3 2 1\n3 1 1\n2 3 4\n3 3 6\n3 2 1\n4 4 12\n3 3 2 1\n6 8 8\n1 1 2 2 2 3\n","output":"4\n7\n0\n0\n1\n5\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountingPairs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn lower_bound(a: &[i64], x: i64) -> usize {
    let (mut lo, mut hi) = (0 as usize, a.len());
    while lo < hi {
        let mid = (lo + hi) / 2;
        if a[mid] < x {
            lo = mid + 1
        } else {
            hi = mid;
        }
    }
    return lo;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, x, y) = (input.read_int(), input.read_long(), input.read_long());
    let mut a: Vec<_> = (0..n).map(|_| input.read_long()).collect();
    a.sort();
    let s: i64 = a.iter().sum();
    let mut result = 0;
    for i in 0..(n as usize) {
        let ii = lower_bound(&a, s - y - a[i]).max(i + 1);
        let jj = lower_bound(&a, s - x - a[i] + 1).max(i + 1);
        result += jj - ii;
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
