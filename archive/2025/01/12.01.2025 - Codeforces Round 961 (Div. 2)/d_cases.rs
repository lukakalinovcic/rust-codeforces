//{"name":"D. Cases","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 5 1\nABCDE\n3 1 2\nAAA\n3 2 2\nAAB\n10 2 2\nABABABABAB\n4 4 4\nDCBA\n1 17 1\nQ\n9 3 2\nABCABCABC\n","output":"5\n1\n2\n1\n1\n1\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCases"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let c = input.read();
    let k = input.read();
    let s = input.read_line().into_bytes();
    let mut prefix = vec![[0; 18]; n + 1];
    let mut bad = vec![false; 1 << c as i32];
    bad[1 << (s[n - 1] - b'A') as usize] = true;
    for i in 0..n {
        if i >= k {
            let mut mask = 0;
            for j in 0..c {
                if prefix[i][j] - prefix[i - k][j] > 0 {
                    mask |= 1 << j;
                }
            }
            bad[mask] = true;
        }
        prefix[i + 1] = prefix[i];
        prefix[i + 1][(s[i] - b'A') as usize] += 1;
    }
    let mut result = c;
    for mask in 0..(1 << c) {
        if bad[mask] {
            for j in 0..c {
                bad[mask | (1 << j)] = true;
            }
        } else {
            let mut cnt = 0;
            for j in 0..c {
                if (mask >> j) & 1 == 0 {
                    cnt += 1;
                }
            }
            result = result.min(cnt);
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
