//{"name":"B. Variety is Discouraged","group":"Codeforces - Codeforces Round 1005 (Div. 2)","url":"https://codeforces.com/contest/2064/problem/B","interactive":false,"timeLimit":1500,"tests":[{"input":"3\n1\n1\n5\n1 1 1 1 1\n4\n2 1 3 2\n","output":"1 1\n0\n2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BVarietyIsDiscouraged"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut cnt = vec![0; n + 1];
    for i in 0..n {
        cnt[a[i] as usize] += 1;
    }
    let mut best = None;
    let mut start = None;
    for i in 0..n {
        if cnt[a[i] as usize] > 1 {
            start = None;
            continue;
        }
        let s = match start {
            None => i,
            Some(s) => s,
        };
        start = Some(s);
        best = match best {
            None => Some((s, i)),
            Some((a, b)) => {
                if i - s > b - a {
                    Some((s, i))
                } else {
                    Some((a, b))
                }
            }
        };
    }
    match best {
        None => out.print_line(0),
        Some((a, b)) => out.print_line((a + 1, b + 1)),
    }
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
