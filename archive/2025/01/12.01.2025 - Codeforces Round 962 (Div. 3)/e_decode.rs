//{"name":"E. Decode","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n0000\n01010101\n1100111001\n11000000111\n","output":"0\n130\n147\n70\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDecode"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: usize = 1000000007;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line().into_bytes();
    let n = s.len();
    let mut prefix_sum: usize = n;
    let mut sum_l_for_prefix_sum = vec![0; 2 * n + 1];
    let mut result = 0;
    for (i, ch) in s.into_iter().enumerate() {
        sum_l_for_prefix_sum[prefix_sum] += i + 1;
        sum_l_for_prefix_sum[prefix_sum] += MOD;
        if ch == b'0' {
            prefix_sum -= 1;
        } else {
            prefix_sum += 1;
        }
        result += (sum_l_for_prefix_sum[prefix_sum] * (n - i)) % MOD;
        result %= MOD;
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
