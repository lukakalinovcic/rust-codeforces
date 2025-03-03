//{"name":"C. Left and Right Houses","group":"Codeforces - Codeforces Round 935 (Div. 3)","url":"https://codeforces.com/contest/1945/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3\n101\n6\n010111\n6\n011001\n3\n000\n3\n110\n3\n001\n4\n1100\n","output":"2\n3\n2\n3\n0\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLeftAndRightHouses"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_line().into_bytes();
    let mut left_happy = 0;
    let mut right_happy = s.iter().filter(|ch| **ch == b'1').count() as i32;
    let mut result = 1000000000;
    let mut best = 1000000000;
    if right_happy >= (n + 1) as i32 / 2 {
        best = n as i32;
        result = 0;
    }
    for i in 0..n {
        left_happy += if s[i] == b'0' { 1 } else { 0 };
        right_happy -= if s[i] == b'1' { 1 } else { 0 };
        let left_cnt = (i + 1) as i32;
        let right_cnt = (n - i - 1) as i32;
        if left_happy >= (left_cnt + 1) / 2 && right_happy >= (right_cnt + 1) / 2 {
            let cost = (n as i32 - 2 * (i + 1) as i32).abs();
            if cost < best {
                best = cost;
                result = i + 1;
            }
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
