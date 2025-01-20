//{"name":"E. Alice's Adventures in the Rabbit Hole","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 2\n1 3\n2 4\n3 5\n9\n1 2\n2 3\n4 5\n5 6\n7 8\n8 9\n2 4\n5 7\n","output":"1 499122177 499122177 0 0\n1 499122177 0 332748118 166374059 0 443664157 720954255 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAlicesAdventuresInTheRabbitHole"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 998244353;
const INF: i64 = 1000000;

fn mul(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 1 {
        mul(a, pow(a, n - 1))
    } else {
        let x = pow(a, n / 2);
        mul(x, x)
    }
}

fn inverse(a: i64) -> i64 {
    pow(a, MOD - 2)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
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
    let mut dist_from_leaf = vec![INF; n];
    let mut prob_up = vec![0; n];
    let mut prob_up_debug = vec![0.0; n];
    for i in (0..n).rev() {
        let u = order[i];
        for v in &adj[u] {
            let v = *v;
            if v != parent[u] {
                dist_from_leaf[u] = dist_from_leaf[u].min(1 + dist_from_leaf[v]);
            }
        }
        if dist_from_leaf[u] == INF {
            dist_from_leaf[u] = 0;
        }
        prob_up[u] = mul(dist_from_leaf[u], inverse(dist_from_leaf[u] + 1))
    }
    prob_up[0] = 1;
    prob_up_debug[0] = 1.0;
    let mut result = vec![0; n];
    result[0] = 1;
    for i in 1..n {
        let u = order[i];
        result[u] = mul(result[parent[u]], prob_up[u]);
    }

    out.print_iter(result.into_iter());
    out.print("\n");
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
