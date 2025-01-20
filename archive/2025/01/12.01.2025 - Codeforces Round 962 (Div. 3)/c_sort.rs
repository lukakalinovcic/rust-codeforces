//{"name":"C. Sort","group":"Codeforces - Codeforces Round 962 (Div. 3)","url":"https://codeforces.com/contest/1996/problem/C","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n5 3\nabcde\nedcba\n1 5\n1 4\n3 3\n4 2\nzzde\nazbe\n1 3\n1 4\n6 3\nuwuwuw\nwuwuwu\n2 4\n1 3\n1 6\n","output":"0\n1\n0\n2\n2\n1\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_line().into_bytes();
    let b = input.read_line().into_bytes();
    let mut a_prefix_cnt = vec![[0; 26]; n + 1];
    let mut b_prefix_cnt = vec![[0; 26]; n + 1];
    for i in 0..n {
        a_prefix_cnt[i + 1] = a_prefix_cnt[i];
        b_prefix_cnt[i + 1] = b_prefix_cnt[i];
        a_prefix_cnt[i + 1][(a[i] - b'a') as usize] += 1;
        b_prefix_cnt[i + 1][(b[i] - b'a') as usize] += 1;
    }

    for (l, r) in input.read_int_pair_vec(q) {
        let (l, r) = (l as usize, r as usize);
        let mut diff = 0;
        for c in 0..26 {
            let a_cnt: i32 = a_prefix_cnt[r][c] - a_prefix_cnt[l - 1][c];
            let b_cnt: i32 = b_prefix_cnt[r][c] - b_prefix_cnt[l - 1][c];
            diff += (a_cnt - b_cnt).abs();
        }
        out.print_line(diff / 2);
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
