//{"name":"H. Hard Demon Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/H","interactive":false,"timeLimit":3500,"tests":[{"input":"2\n4 3\n1 5 2 4\n4 9 5 3\n4 5 2 3\n1 5 5 2\n1 1 4 4\n2 2 3 3\n1 2 4 3\n3 3\n1 2 3\n4 5 6\n7 8 9\n1 1 1 3\n1 3 3 3\n2 2 2 2\n","output":"500 42 168\n14 42 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HHardDemonProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let q: usize = input.read();
    let mut a = vec![vec![0; n + 1]; n + 1];
    let mut s = vec![vec![0; n + 1]; n + 1];
    let mut rws = vec![vec![0; n + 1]; n + 1];
    let mut pws = vec![vec![0; n + 1]; n + 1];
    for r in 1..=n {
        for c in 1..=n {
            a[r][c] = input.read_long();
            s[r][c] = a[r][c] + s[r - 1][c] + s[r][c - 1] - s[r - 1][c - 1];
            let rw = r as i64 * a[r][c];
            rws[r][c] = rw + rws[r - 1][c] + rws[r][c - 1] - rws[r - 1][c - 1];
            let p = ((r - 1) * n + c) as i64;
            let pw = p * a[r][c];
            pws[r][c] = pw + pws[r - 1][c] + pws[r][c - 1] - pws[r - 1][c - 1];
        }
    }
    let mut results = vec![];
    for _ in 0..q {
        let r1: usize = input.read();
        let c1: usize = input.read();
        let r2: usize = input.read();
        let c2: usize = input.read();
        let p = ((r1 - 1) * n + c1) as i64;
        let box_sum = s[r2][c2] - s[r1 - 1][c2] - s[r2][c1 - 1] + s[r1 - 1][c1 - 1];
        let box_rws = rws[r2][c2] - rws[r1 - 1][c2] - rws[r2][c1 - 1] + rws[r1 - 1][c1 - 1];
        let adjusted_box_rws = box_rws - (r1 as i64) * box_sum;
        let box_pws = pws[r2][c2] - pws[r1 - 1][c2] - pws[r2][c1 - 1] + pws[r1 - 1][c1 - 1];
        let result = box_pws
            - (p as i64 - 1) * box_sum
            - (n as i64 - c2 as i64 + c1 as i64 - 1) * adjusted_box_rws;
        results.push(result);
    }
    out.print_iter(results.into_iter());
    out.put(b'\n');
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
