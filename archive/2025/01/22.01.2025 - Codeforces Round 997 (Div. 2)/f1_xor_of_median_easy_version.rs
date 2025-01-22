//{"name":"F1. Xor of Median (Easy Version)","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 3\n10\n2 3\n11\n5 1\n11101\n7 9\n1101011\n17 34\n11001010001010010\n1 500\n1\n","output":"3\n2\n0\n8\n32\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1XorOfMedianEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _k: usize = input.read();
    let m = input.read();
    let mut ones = 0;
    for c in input.read_line().into_bytes() {
        if c == b'1' {
            ones += 1;
        }
    }
    let mut choose = vec![vec![0; m]; m];
    for i in 0..m {
        choose[i][0] = 1;
        choose[i][i] = 1;
        for j in 1..i {
            choose[i][j] = choose[i - 1][j - 1] ^ choose[i - 1][j];
        }
    }
    let mut dp = vec![vec![0; ones + 1]; ones + 1];
    dp[1][1] = 1;
    for i in 2..=ones {
        for j in 1..=i {
            dp[i][j] = (dp[i - 1][j] * j + dp[i - 1][j - 1]) % 2;
        }
    }

    let mut result = 0;
    for x in 1..m {
        for groups in 1..=ones.min(x + 1) {
            result ^= x * dp[ones][groups] * choose[x][groups - 1];
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
