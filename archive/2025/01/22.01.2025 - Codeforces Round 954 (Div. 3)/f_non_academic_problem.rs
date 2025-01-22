//{"name":"F. Non-academic Problem","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2 1\n1 2\n3 3\n1 2\n2 3\n1 3\n5 5\n1 2\n1 3\n3 4\n4 5\n5 3\n6 7\n1 2\n1 3\n2 3\n3 4\n4 5\n4 6\n5 6\n5 5\n1 2\n1 3\n2 3\n2 4\n3 5\n10 12\n1 2\n1 3\n2 3\n2 4\n4 5\n5 6\n6 7\n7 4\n3 8\n8 9\n9 10\n10 8\n","output":"0\n3\n4\n6\n6\n21\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNonAcademicProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(m) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut ctx = DfsContext::new(n);
    dfs(&adj, 0, n, &mut ctx);
    let mut result = 0;
    for i in 1..n {
        if ctx.lowlink[i] == ctx.discover[i] {
            result = result.max(ctx.size[i] as i64 * (n as i32 - ctx.size[i]) as i64);
        }
    }
    let result = n as i64 * (n as i64 - 1) / 2 - result;
    out.print_line(result);
}

#[derive(Debug)]
struct DfsContext {
    t: i32,
    discover: Vec<i32>,
    lowlink: Vec<i32>,
    size: Vec<i32>,
}

impl DfsContext {
    fn new(n: usize) -> Self {
        Self {
            t: 0,
            discover: vec![-1; n],
            lowlink: vec![0; n],
            size: vec![0; n],
        }
    }
}

fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, ctx: &mut DfsContext) {
    ctx.discover[u] = ctx.t;
    ctx.t += 1;
    ctx.lowlink[u] = ctx.discover[u];
    ctx.size[u] = 1;
    for v in &adj[u] {
        let v = *v;
        if v == p {
            continue;
        }
        if ctx.discover[v] == -1 {
            dfs(adj, v, u, ctx);
            ctx.size[u] += ctx.size[v];
            ctx.lowlink[u] = ctx.lowlink[u].min(ctx.lowlink[v]);
        } else {
            ctx.lowlink[u] = ctx.lowlink[u].min(ctx.discover[v]);
        }
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
