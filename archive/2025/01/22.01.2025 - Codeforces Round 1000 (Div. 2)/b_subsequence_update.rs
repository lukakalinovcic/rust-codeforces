//{"name":"B. Subsequence Update","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n2 1 1\n2 1\n3 2 3\n1 2 3\n3 1 3\n3 1 2\n4 2 3\n1 2 2 2\n5 2 5\n3 3 2 3 5\n6 1 3\n3 6 6 4 3 2\n","output":"1\n3\n6\n3\n11\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSubsequenceUpdate"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let l = input.read();
    let r = input.read();
    let mut a = vec![];
    let mut b = vec![];
    let mut c = vec![];
    for i in 1..=n {
        let x = input.read_int();
        if i < l {
            a.push(x);
        } else if i <= r {
            b.push(x);
        } else {
            c.push(x);
        }
    }
    a.sort();
    b.sort();
    c.sort();
    out.print_line(solve2(&b, &a).min(solve2(&b, &c)));
}

fn solve2(b: &[i32], a: &[i32]) -> i64 {
    let mut sum = 0 as i64;
    for i in 0..b.len() {
        sum += b[i] as i64;
    }
    let mut result = sum;
    for i in 0..b.len().min(a.len()) {
        sum -= b[b.len() - 1 - i] as i64;
        sum += a[i] as i64;
        result = result.min(sum);
    }
    result
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
