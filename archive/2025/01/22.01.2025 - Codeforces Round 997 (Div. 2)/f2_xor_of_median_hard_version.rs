//{"name":"F2. Xor of Median (Hard Version)","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 3\n10\n2 3\n11\n5 1\n11101\n7 9\n1101011\n17 34\n11001010001010010\n1 1000000000\n1\n","output":"3\n2\n0\n8\n32\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2XorOfMedianHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _k: usize = input.read();
    let m = input.read();
    let mut ones: usize = 0;
    for c in input.read_line().into_bytes() {
        if c == b'1' {
            ones += 1;
        }
    }
    let n = (ones).next_power_of_two().trailing_zeros();
    let mut dp = vec![0; 1 << n];
    for groups in 1..=ones {
        let t = (ones - groups) & ((groups - 1) / 2) == 0;
        dp[groups - 1] = if t { 1 } else { 0 };
    }
    for i in (1..1 << n).rev() {
        let mut j = (i - 1) & i;
        loop {
            dp[i] ^= dp[j];
            if j == 0 {
                break;
            }
            j = (j - 1) & i;
        }
    }
    let mut result = 0;
    for mask in 0..1 << n {
        if dp[mask] == 0 || mask >= m {
            continue;
        }
        let mut k = m >> n;
        if ((k << n) | mask) >= m {
            k -= 1;
        }
        if k % 2 == 0 {
            result ^= mask;
        }
        result ^= match k % 4 {
            0 => k << n,
            1 => 1 << n,
            2 => (k + 1) << n,
            3 => 0,
            _ => panic!("unexpected modulo"),
        };
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
