//{"name":"F. Sheriff's Defense","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1\n2 3 1\n1 2\n2 3\n3 1\n3 6 3\n1 2\n2 3\n3 1\n-2 -3 -1\n1 2\n2 3\n6 1\n5 -4 3 6 7 3\n4 1\n5 1\n3 5\n3 6\n1 2\n8 1\n3 5 2 7 8 5 -3 -4\n7 3\n1 8\n4 3\n3 5\n7 6\n8 7\n2 1\n","output":"3\n8\n0\n17\n26\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FSheriffsDefense"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let c = input.read_long();
    let gold = input.read_long_vec(n);
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
    let mut dp = vec![[[0, 0], [0, 0]]; n];
    for i in (0..n).rev() {
        let u = order[i];

        for u_strong in [0, 1] {
            let mut deltas = vec![];
            let mut init_sum = 0;
            for v in &adj[u] {
                let v = *v;
                if v == parent[u] {
                    continue;
                }
                init_sum += dp[v][u_strong][0]; // v not strong;
                deltas.push(dp[v][u_strong][1] - dp[v][u_strong][0]); // delta when we make v strong;
            }
            deltas.sort();
            deltas.reverse();
            for parent_strong in [0, 1] {
                let mut sum = init_sum;
                let mut u_gold = gold[u];
                if parent_strong == 1 {
                    u_gold -= c;
                }
                let mut best = sum + if u_strong == 1 { u_gold.max(0) } else { 0 };

                for delta in &deltas {
                    sum += *delta;
                    u_gold -= c;
                    best = best.max(sum + if u_strong == 1 { u_gold.max(0) } else { 0 });
                }
                dp[u][parent_strong][u_strong] = best;
            }
        }
    }
    out.print_line(dp[0][0][0].max(dp[0][0][1]));
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
