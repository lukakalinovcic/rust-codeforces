//{"name":"D. Attribute Checks","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/D","interactive":false,"timeLimit":2500,"tests":[{"input":"10 5\n0 1 0 2 0 -3 0 -4 0 -5\n","output":"3\n"},{"input":"3 1\n1 -1 0\n","output":"0\n"},{"input":"9 3\n0 0 1 0 2 -3 -2 -2 1\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAttributeChecks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut diff = vec![0; m + 2];
    let mut k: i32 = 0;
    for r in input.read_int_vec(n) {
        if r == 0 {
            let mut sum = 0;
            let mut prev = 0;
            let mut curr = 0;
            let mut next;
            for i in 0..=k {
                sum += diff[i as usize];
                next = sum;
                curr = curr.max(sum);
                diff[i as usize] = curr - prev;
                prev = curr;
                curr = next;
            }
            diff[(k + 1) as usize] = curr - prev;
            k += 1;
        } else if r > 0 {
            if r <= k {
                diff[r as usize] += 1;
                diff[(k + 1) as usize] -= 1;
            }
        } else {
            if -r <= k {
                diff[0] += 1;
                diff[(k + r + 1) as usize] -= 1;
            }
        }
    }
    let mut result = 0;
    let mut sum = 0;
    for i in 0..=m {
        sum += diff[i];
        result = result.max(sum);
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
