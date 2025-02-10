//{"name":"E. Skibidus and Rizz","group":"Codeforces - Codeforces Round 1003 (Div. 4)","url":"https://codeforces.com/contest/2065/problem/E","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n1 2 1\n4 3 2\n2 4 3\n8 3 2\n5 0 4\n5 0 5\n","output":"101\n0100101\n011011\n-1\n-1\n00000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESkibidusAndRizz"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let zeros = input.read_size();
    let ones = input.read_size();
    let k = input.read_size();
    match doit(zeros, ones, k) {
        None => out.print_line(-1),
        Some(result) => out.print_line(String::from_utf8(result).unwrap()),
    }
}

fn doit(zeros: usize, ones: usize, k: usize) -> Option<Vec<u8>> {
    let (n, m, c) = if zeros > ones {
        (zeros, ones, b'0')
    } else {
        (ones, zeros, b'1')
    };
    if k > n {
        return None;
    }
    if k == 1 {
        if n == m {
            Some(alternating(n + m, c))
        } else if n == m + 1 {
            Some(alternating(n + m, c))
        } else {
            None
        }
    } else {
        let n = n - k;
        if n > m {
            None
        } else if m - n <= k {
            Some(vec![vec![c; k], alternating(n + n, c ^ 1), vec![c ^ 1; m - n]].concat())
        } else {
            None
        }
    }
}

fn alternating(n: usize, first: u8) -> Vec<u8> {
    let mut c = first;
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = c;
        c ^= 1;
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
