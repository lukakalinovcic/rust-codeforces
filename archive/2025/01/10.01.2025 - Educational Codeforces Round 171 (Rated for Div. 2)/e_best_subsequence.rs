//{"name":"E. Best Subsequence","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 0\n4\n1 0 1 2\n1\n5\n8\n7 1 48 14 13 8 7 6\n","output":"3\n2\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBestSubsequence"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn dfs(
    adj: &Vec<Vec<usize>>,
    i: usize,
    seen: &mut Vec<bool>,
    matched: &mut Vec<Option<usize>>,
) -> bool {
    if seen[i] {
        return false;
    }
    seen[i] = true;
    for j in &adj[i] {
        let j = *j;
        let ok = match matched[j] {
            None => true,
            Some(k) => k != i && dfs(adj, k, seen, matched),
        };
        if ok {
            matched[j] = Some(i);
            return true;
        }
    }
    false
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = input.read_long_vec(n);
    let mut adj = vec![vec![]; n];
    for i in 0..n {
        for j in 0..60 {
            if ((a[i] >> j) & 1) == 1 {
                adj[i].push(j as usize);
            }
        }
    }
    let mut max_matching = 0;
    let mut matched = vec![None; 60];
    for i in 0..n {
        let mut seen = vec![false; n];
        if dfs(&adj, i, &mut seen, &mut matched) {
            max_matching += 1;
        }
    }
    out.print_line(n - max_matching)
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
