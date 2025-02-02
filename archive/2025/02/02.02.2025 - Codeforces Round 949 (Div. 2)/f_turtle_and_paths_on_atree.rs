//{"name":"F. Turtle and Paths on a Tree","group":"Codeforces - Codeforces Round 949 (Div. 2)","url":"https://codeforces.com/contest/1981/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n5\n3 2 2 1 1\n1 1 2 2\n5\n3 2 1 1 1\n1 1 2 2\n6\n1 2 1 2 1 3\n1 2 3 3 4\n7\n2 1 2 3 1 2 1\n1 1 2 2 3 3\n10\n1 2 2 1 4 2 3 1 2 1\n1 1 2 2 3 3 4 5 5\n","output":"4\n6\n6\n6\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTurtleAndPathsOnATree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

#[derive(Clone, Copy)]
struct Value {
    a: i32, // min a[i] for i in lo..=hi
    b: i32, // min a[i] + i for i in lo..=hi
}

impl Value {
    fn apply(&mut self, lo: i32, u: Update) {
        self.a = (self.a + u.x).min(u.m);
        self.b = (self.b + u.x).min(u.m + lo);
    }

    fn combine(&mut self, lt: Value, rt: Value) {
        self.a = lt.a.min(rt.a);
        self.b = lt.b.min(rt.b);
    }
}

impl Default for Value {
    fn default() -> Self {
        Self { a: INF, b: INF }
    }
}

#[derive(Clone, Copy)]
struct Update {
    x: i32,
    m: i32, // a[i] = (a[i] + x).min(m);
}

impl Update {
    fn compose(&mut self, u: Update) {
        self.x = self.x + u.x;
        self.m = (self.m + u.x).min(u.m);
    }
}

impl Default for Update {
    fn default() -> Self {
        Self { x: 0, m: INF }
    }
}

struct Node {
    ch: Option<(i32, i32)>,
    v: Value,
    u: Update,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            ch: None,
            v: Value::default(),
            u: Update::default(),
        }
    }
}

struct SegTreeForest {
    nodes: Vec<Node>,
    roots: Vec<i32>,
}

impl SegTreeForest {
    fn new(n: usize) -> Self {
        Self {
            nodes: vec![],
            roots: vec![-1; n],
        }
    }

    fn set_root(&mut self, u: usize, i: i32) {
        self.roots[u] = i;
    }

    fn get_root(&mut self, u: usize) -> i32 {
        self.roots[u]
    }

    fn get_value(&mut self, i: i32) -> Value {
        self.nodes[i as usize].v
    }

    fn new_node(&mut self) -> i32 {
        let x = self.nodes.len() as i32;
        self.nodes.push(Node::default());
        x
    }

    fn update_node(&mut self, i: i32, lo: i32, u: Update) {
        self.nodes[i as usize].v.apply(lo, u);
        self.nodes[i as usize].u.compose(u);
    }

    fn propagate(&mut self, i: i32, lo: i32, hi: i32) -> (i32, i32) {
        let (lt, rt) = match self.nodes[i as usize].ch {
            None => {
                let (lt, rt) = (self.new_node(), self.new_node());
                self.nodes[i as usize].ch = Some((lt, rt));
                (lt, rt)
            }
            Some((lt, rt)) => (lt, rt),
        };
        let mid = (lo + hi) / 2;
        let u = self.nodes[i as usize].u;
        self.update_node(lt, lo, u);
        self.update_node(rt, mid, u);
        self.nodes[i as usize].u = Update::default();
        (lt, rt)
    }

    fn combine(&mut self, i: i32, lt: i32, rt: i32) {
        let lt = self.nodes[lt as usize].v;
        let rt = self.nodes[rt as usize].v;
        self.nodes[i as usize].v.combine(lt, rt);
    }

    fn set_inf(&mut self, i: i32, lo: i32, hi: i32, p: i32) {
        if p < lo || p >= hi {
            return;
        }
        if hi - lo == 1 {
            self.nodes[i as usize].v.a = INF;
            self.nodes[i as usize].v.b = INF;
            return;
        }
        let (lt, rt) = self.propagate(i, lo, hi);
        let mid = (lo + hi) / 2;
        self.set_inf(lt, lo, mid, p);
        self.set_inf(rt, mid, hi, p);
        self.combine(i, lt, rt);
    }

    fn update_range(&mut self, i: i32, lo: i32, hi: i32, a: i32, b: i32, u: Update) {
        if a >= hi || b <= lo {
            return;
        }
        if lo >= a && hi <= b {
            self.update_node(i, lo, u);
            return;
        }
        let (lt, rt) = self.propagate(i, lo, hi);
        let mid = (lo + hi) / 2;
        self.update_range(lt, lo, mid, a, b, u);
        self.update_range(rt, mid, hi, a, b, u);
        self.combine(i, lt, rt);
    }

    fn query_a_plus_b(&mut self, i: i32, j: i32, lo: i32, hi: i32) -> i32 {
        if self.nodes[i as usize].ch.is_none() {
            return self.nodes[i as usize].v.a + self.nodes[j as usize].v.b;
        }
        if self.nodes[j as usize].ch.is_none() {
            return self.nodes[j as usize].v.a + self.nodes[i as usize].v.b;
        }
        let (i_lt, i_rt) = self.propagate(i, lo, hi);
        let (j_lt, j_rt) = self.propagate(j, lo, hi);
        let mid = (lo + hi) / 2;
        let x = self.query_a_plus_b(i_lt, j_lt, lo, mid);
        let y = self.query_a_plus_b(i_rt, j_rt, mid, hi);
        self.combine(i, i_lt, i_rt);
        self.combine(j, j_lt, j_rt);
        x.min(y)
    }

    fn merge(&mut self, i: i32, j: i32, lo: i32, hi: i32) -> i32 {
        if self.nodes[i as usize].ch.is_none() {
            let m = self.nodes[i as usize].v.a;
            self.update_node(j, lo, Update { x: 0, m });
            return j;
        }
        if self.nodes[j as usize].ch.is_none() {
            let m = self.nodes[j as usize].v.a;
            self.update_node(i, lo, Update { x: 0, m });
            return i;
        }
        let (i_lt, i_rt) = self.propagate(i, lo, hi);
        let (j_lt, j_rt) = self.propagate(j, lo, hi);
        let mid = (lo + hi) / 2;
        let (lt, rt) = (
            self.merge(i_lt, j_lt, lo, mid),
            self.merge(i_rt, j_rt, mid, hi),
        );
        self.nodes[i as usize].ch = Some((lt, rt));
        self.combine(i, lt, rt);
        return i;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input
        .read_int_vec(n)
        .into_iter()
        .map(|a| a.min(n as i32 + 1))
        .collect::<Vec<_>>();
    let mut children = vec![vec![]; n];
    for i in 1..n {
        let p = input.read_int() - 1;
        children[p as usize].push(i);
    }
    let mut forest = SegTreeForest::new(n);
    out.print_line(dfs(&children, 0, &a, &mut forest));
}

fn dfs(children: &Vec<Vec<usize>>, u: usize, a: &[i32], forest: &mut SegTreeForest) -> i32 {
    let n = children.len() as i32;
    match children[u].as_slice() {
        [] => {
            let i = forest.new_node();
            forest.update_range(i, 1, n + 2, 1, n + 2, Update { x: 0, m: 0 });
            forest.set_inf(i, 1, n + 2, a[u]);
            forest.set_root(u, i);
            0
        }
        [x] => {
            dfs(children, *x, a, forest);
            let i = forest.get_root(*x);
            forest.set_inf(i, 1, n + 2, a[u]);
            let minx = forest.get_value(i).b;
            forest.update_range(i, 1, n + 2, 1, n + 2, Update { x: 0, m: minx });
            forest.set_inf(i, 1, n + 2, a[u]);
            forest.set_root(u, i);
            minx
        }
        [x, y] => {
            dfs(children, *x, a, forest);
            dfs(children, *y, a, forest);
            let i_x = forest.get_root(*x);
            let i_y = forest.get_root(*y);
            forest.set_inf(i_x, 1, n + 2, a[u]);
            forest.set_inf(i_y, 1, n + 2, a[u]);
            let minx = forest.get_value(i_x).b;
            let miny = forest.get_value(i_y).b;
            let best = (minx + miny).min(forest.query_a_plus_b(i_x, i_y, 1, n + 2));
            forest.update_range(i_x, 1, n + 2, 1, n + 2, Update { x: miny, m: INF });
            forest.update_range(i_y, 1, n + 2, 1, n + 2, Update { x: minx, m: INF });
            let i = forest.merge(i_x, i_y, 1, n + 2);
            forest.update_range(i, 1, n + 2, 1, n + 2, Update { x: 0, m: best });
            forest.set_inf(i, 1, n + 2, a[u]);
            forest.set_root(u, i);
            best
        }
        _ => panic!("unexpected number of children"),
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
