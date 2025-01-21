//{"name":"D. Inaccurate Subsequence Search","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n7 4 2\n4 1 2 3 4 5 6\n1 2 3 4\n7 4 3\n4 1 2 3 4 5 6\n1 2 3 4\n7 4 4\n4 1 2 3 4 5 6\n1 2 3 4\n11 5 3\n9 9 2 2 10 9 7 6 3 6 3\n6 9 7 8 10\n4 1 1\n4 1 5 6\n6\n","output":"4\n3\n2\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DInaccurateSubsequenceSearch"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 1000000;

struct PreCalc {
    cnt_a: Vec<i32>,
    cnt_b: Vec<i32>,
}

impl PreCalc {
    fn new() -> Self {
        Self {
            cnt_a: vec![0; MAX + 1],
            cnt_b: vec![0; MAX + 1],
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let cnt_a = &mut data.cnt_a;
    let cnt_b = &mut data.cnt_b;
    let n: usize = input.read();
    let m: usize = input.read();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(m);

    for b in &b {
        cnt_b[*b as usize] += 1;
    }

    let mut result = 0;
    let mut matching = 0;
    for i in 0..n + m {
        if i < n {
            let x = a[i] as usize;
            matching -= cnt_a[x].min(cnt_b[x]);
            cnt_a[x] += 1;
            matching += cnt_a[x].min(cnt_b[x]);
        }
        if i >= m {
            let x = a[i - m] as usize;
            matching -= cnt_a[x].min(cnt_b[x]);
            cnt_a[x] -= 1;
            matching += cnt_a[x].min(cnt_b[x]);
        }
        if i >= m - 1 && i < n && matching >= k {
            result += 1;
        }
    }
    out.print_line(result);

    for b in &b {
        cnt_b[*b as usize] -= 1;
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

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
