//{"name":"E. Long Inversions","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n5\n00100\n5\n01000\n7\n1011101\n3\n000\n2\n10\n","output":"3\n2\n4\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ELongInversions"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s = input
        .read_line()
        .into_bytes()
        .into_iter()
        .map(|c| (c - b'0') as i32)
        .collect::<Vec<_>>();
    for k in (1..=n).rev() {
        if check(&s, k) {
            out.print_line(k);
            break;
        }
    }
}

fn check(s: &[i32], k: usize) -> bool {
    let n = s.len();
    let mut flip = 0;
    let mut flip_end = vec![0; n + 1];
    for i in 0..n {
        flip ^= flip_end[i];
        let x = s[i] ^ flip;
        if x == 0 {
            if i + k > n {
                return false;
            }
            flip ^= 1;
            flip_end[i + k] = 1;
        }
    }
    true
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
