//{"name":"E1. The Game (Easy Version)","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/E1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n4\n2 2 4 3\n1 2\n1 3\n2 4\n5\n1 2 3 4 5\n1 2\n2 3\n3 4\n4 5\n3\n1 2 3\n1 2\n1 3\n5\n3 1 3 4 5\n1 2\n2 3\n3 4\n4 5\n10\n1 2 3 2 4 3 3 4 4 3\n1 4\n4 6\n7 4\n6 9\n6 5\n7 8\n1 2\n2 3\n2 10\n","output":"2\n0\n2\n2\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1TheGameEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

#[derive(Debug)]
struct DfsContext {
    t: i32,
    enter: Vec<i32>,
    exit: Vec<i32>,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let w = input.read_int_vec(n);
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut ctx = DfsContext {
        t: 0,
        enter: vec![0; n],
        exit: vec![0; n],
    };
    dfs(&adj, 0, n, &mut ctx);

    let mut x = (0..n).map(|i| (w[i], i)).collect::<Vec<_>>();
    x.sort();
    x.reverse();
    let mut j = 0;
    let tree_spec = MySpec {};
    let mut active = LazySegTree::new(&tree_spec, 2 * n);
    for i in 0..n {
        while j < i && x[j].0 > x[i].0 {
            active.set(ctx.enter[x[j].1] as usize, 1);
            j += 1;
        }
        let remaining =
            j as i32 - active.product(ctx.enter[x[i].1] as usize, ctx.exit[x[i].1] as usize);
        if remaining > 0 {
            out.print_line(x[i].1 + 1);
            return;
        }
    }
    out.print_line(0);
}

fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, ctx: &mut DfsContext) {
    ctx.enter[u] = ctx.t;
    ctx.t += 1;
    for v in &adj[u] {
        let v = *v;
        if v == p {
            continue;
        }
        dfs(&adj, v, u, ctx);
    }
    ctx.exit[u] = ctx.t;
    ctx.t += 1;
}

struct MySpec {}
impl LazySegTreeSpec<i32, ()> for &MySpec {
    fn identity(&self) -> i32 {
        0
    }

    fn op(&self, a: &i32, b: &i32) -> i32 {
        a + b
    }

    fn compose(&self, _old: &(), _new: &()) -> () {
        ()
    }

    fn update(&self, t: &i32, _u: &()) -> i32 {
        *t
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
