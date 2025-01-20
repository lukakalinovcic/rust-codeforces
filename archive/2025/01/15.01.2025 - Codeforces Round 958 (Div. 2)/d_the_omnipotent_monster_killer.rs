//{"name":"D. The Omnipotent Monster Killer","group":"Codeforces - Codeforces Round 958 (Div. 2)","url":"https://codeforces.com/contest/1988/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1\n1000000000000\n5\n47 15 32 29 23\n1 2\n1 3\n2 4\n2 5\n7\n8 10 2 3 5 7 4\n1 2\n1 4\n3 2\n5 3\n6 2\n7 5\n","output":"1000000000000\n193\n57\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTheOmnipotentMonsterKiller"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let cost = input.read_long_vec(n);
    let mut adj = vec![vec![]; n];
    for (a, b) in input.read_int_pair_vec(n - 1) {
        let (a, b) = (a as usize - 1, b as usize - 1);
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut order = vec![0];
    let mut parent = vec![0; n];
    for i in 0..n {
        let u = order[i];
        for v in &adj[u] {
            let v = *v;
            if v != parent[u] {
                order.push(v);
                parent[v] = u;
            }
        }
    }
    const MAX_ROUNDS: usize = 20;
    const INF: i64 = 1000000000000000000;
    let mut dp = vec![[0; MAX_ROUNDS]; n];
    for i in (0..n).rev() {
        let u = order[i];
        let mut sum = [0; MAX_ROUNDS];
        for u_round in 0..MAX_ROUNDS {
            sum[u_round] = (u_round + 1) as i64 * cost[u];
            for v in &adj[u] {
                let v = *v;
                if v == parent[u] {
                    continue;
                }
                sum[u_round] += dp[v][u_round];
            }
        }
        let mut prefix_min = [0; MAX_ROUNDS + 1];
        prefix_min[0] = INF;
        for i in 0..MAX_ROUNDS {
            prefix_min[i + 1] = prefix_min[i].min(sum[i]);
        }
        let mut suffix_min = INF;
        for p_round in (0..MAX_ROUNDS).rev() {
            dp[u][p_round] = suffix_min.min(prefix_min[p_round]);
            suffix_min = suffix_min.min(sum[p_round]);
        }
    }
    out.print_line(dp[0].iter().fold(INF, |acc, x| acc.min(*x)));
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
