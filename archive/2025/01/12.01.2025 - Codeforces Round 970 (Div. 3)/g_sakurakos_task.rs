//{"name":"G. Sakurako's Task","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 3\n3\n2 10\n1 1\n3 1\n1 2 3\n3 2\n1 2 4\n4 5\n2 2 2 16\n4 5\n2 2 2 3\n","output":"2\n11\n3\n4\n8\n8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSakurakosTask"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_long();
    let a = input.read_long_vec(n);
    if n == 1 {
        if k <= a[0] {
            out.print_line(k - 1);
        } else {
            out.print_line(k);
        }
        return;
    }
    let mut g = 0;
    for x in a.iter() {
        g = gcd(g, *x);
    }
    let n = n as i64;
    for i in 0..n {
        if i * (g - 1) >= k {
            let rem = k - (i - 1) * (g - 1);
            out.print_line((i - 1) * g + rem);
            return;
        }
    }
    let rem = k - (n - 1) * (g - 1);
    out.print_line((n - 1) * g + rem);
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
