//{"name":"C. Remove Exactly Two","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n4\n1 2\n2 3\n2 4\n7\n1 2\n1 3\n2 4\n4 5\n5 6\n5 7\n","output":"0\n2\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRemoveExactlyTwo"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut memo = vec![[[INF; 2]; 2]; n];
    out.print_line(rec(&adj, 0, n, 0, 2, &mut memo));
}

fn rec(
    adj: &Vec<Vec<usize>>,
    u: usize,
    p: usize,
    parent_alive: usize,
    to_remove: usize,
    memo: &mut [[[i32; 2]; 2]],
) -> i32 {
    if memo[u][parent_alive][to_remove - 1] != INF {
        return memo[u][parent_alive][to_remove - 1];
    }

    let mut result = -INF;
    if to_remove == 1 {
        let mut score = 0;
        for v in &adj[u] {
            let v = *v;
            if v == p {
                continue;
            }
            score += 1;
        }
        result = result.max(score);

        let add = if parent_alive == 0 { 1 } else { 0 };
        for v in &adj[u] {
            let v = *v;
            if v == p {
                continue;
            }
            result = result.max(add + rec(adj, v, u, 1, 1, memo));
        }
    } else if to_remove == 2 {
        // pick u
        let mut best_diff = -INF;
        let mut score = 0;
        for v in &adj[u] {
            let v = *v;
            if v == p {
                continue;
            }
            let a = 1;
            let b = rec(adj, v, u, 0, 1, memo);
            score += a;
            best_diff = best_diff.max(b - a);
        }
        score += best_diff;
        result = result.max(score);

        let add = if parent_alive == 0 { 1 } else { 0 };
        for v in &adj[u] {
            let v = *v;
            if v == p {
                continue;
            }
            result = result.max(add + rec(adj, v, u, 1, 2, memo));
        }

        let mut best1 = -INF;
        let mut best2 = -INF;
        for v in &adj[u] {
            let v = *v;
            if v == p {
                continue;
            }
            let x = rec(adj, v, u, 1, 1, memo);
            if x > best1 {
                best2 = best1;
                best1 = x;
            } else if x > best2 {
                best2 = x;
            }
        }
        result = result.max(add + best1 + best2);
    } else {
        panic!("Unexpected to_remove {to_remove}");
    }
    memo[u][parent_alive][to_remove - 1] = result;
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
