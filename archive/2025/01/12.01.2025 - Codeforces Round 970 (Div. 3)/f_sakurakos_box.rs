//{"name":"F. Sakurako's Box","group":"Codeforces - Codeforces Round 970 (Div. 3)","url":"https://codeforces.com/contest/2008/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n3 2 3\n4\n2 2 2 4\n5\n1 2 3 4 5\n","output":"7\n6\n500000012\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSakurakosBox"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 1000000007;

fn mul(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 1 {
        mul(a, pow(a, n - 1))
    } else {
        let x = pow(a, n / 2);
        mul(x, x)
    }
}

fn inverse(a: i64) -> i64 {
    pow(a, MOD - 2)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut sum = 0;
    let mut result = 0;
    for i in 0..n {
        result = (result + mul(sum, a[i])) % MOD;
        sum = (sum + a[i]) % MOD;
    }
    result = mul(result, inverse(n as i64));
    result = mul(result, inverse(n as i64 - 1));
    result = mul(result, 2);
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
