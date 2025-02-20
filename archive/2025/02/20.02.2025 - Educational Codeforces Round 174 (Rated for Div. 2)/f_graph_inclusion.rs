//{"name":"F. Graph Inclusion","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"6 9\nA 2 3\nB 1 3\nA 2 1\nA 3 2\nB 5 6\nA 6 5\nA 3 4\nA 4 2\nA 4 3\n","output":"0\n1\n0\n1\n2\n1\n1\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FGraphInclusion"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

type Edge = (char, i32, i32);

struct DsuRollback {
    u: i32,
    v: i32,
    parent_v: i32,
    rank_u: i32,
}

struct DsuWithRollback {
    parent: Vec<i32>,
    rank: Vec<i32>,
    num_components: usize,
    rollbacks: Vec<DsuRollback>,
}

impl DsuWithRollback {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n as i32).collect(),
            rank: vec![0; n],
            num_components: n,
            rollbacks: vec![],
        }
    }

    fn find(&self, u: i32) -> i32 {
        if self.parent[u as usize] == u {
            u
        } else {
            self.find(self.parent[u as usize])
        }
    }

    fn union(&mut self, u: i32, v: i32) -> bool {
        let u = self.find(u) as usize;
        let v = self.find(v) as usize;
        if u == v {
            return false;
        }
        let (u, v) = if self.rank[u] < self.rank[v] {
            (v, u)
        } else {
            (u, v)
        };
        self.rollbacks.push(DsuRollback {
            u: u as i32,
            v: v as i32,
            parent_v: self.parent[v],
            rank_u: self.rank[u],
        });
        self.parent[v] = u as i32;
        if self.rank[u] == self.rank[v] {
            self.rank[u] += 1;
        }
        self.num_components -= 1;
        true
    }

    fn rollback(&mut self) {
        let DsuRollback {
            u,
            v,
            parent_v,
            rank_u,
        } = self.rollbacks.pop().unwrap();
        self.rank[u as usize] = rank_u;
        self.parent[v as usize] = parent_v;
        self.num_components += 1;
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let mut tree: Vec<Vec<Edge>> = vec![vec![]; 2 * q.next_power_of_two()];
    let mut active_since: HashMap<Edge, usize> = HashMap::new();
    for i in 0..q {
        let c = input.read_char();
        let x = input.read_int() - 1;
        let y = input.read_int() - 1;
        let e = (c, x.min(y), x.max(y));
        match active_since.entry(e) {
            Entry::Occupied(entry) => {
                let start = entry.remove();
                tree_add(&mut tree, 1, 0, q, start, i, e);
            }
            Entry::Vacant(entry) => {
                entry.insert(i);
            }
        }
    }
    for (e, start) in active_since.into_iter() {
        tree_add(&mut tree, 1, 0, q, start, q, e);
    }
    let mut g_a = DsuWithRollback::new(n);
    let mut g_ab = DsuWithRollback::new(n);
    tree_traverse(&tree, 1, 0, q, &mut g_a, &mut g_ab, out);
}

fn tree_add(
    tree: &mut Vec<Vec<Edge>>,
    i: usize,
    lo: usize,
    hi: usize,
    a: usize,
    b: usize,
    e: Edge,
) {
    if lo >= b || hi <= a {
        return;
    }
    if lo >= a && hi <= b {
        tree[i].push(e);
        return;
    }
    let mid = (lo + hi) / 2;
    tree_add(tree, 2 * i, lo, mid, a, b, e);
    tree_add(tree, 2 * i + 1, mid, hi, a, b, e);
}

fn tree_traverse(
    tree: &Vec<Vec<Edge>>,
    i: usize,
    lo: usize,
    hi: usize,
    g_a: &mut DsuWithRollback,
    g_ab: &mut DsuWithRollback,
    out: &mut Output,
) {
    let mut g_a_updates = 0;
    let mut g_ab_updates = 0;
    for &(c, x, y) in &tree[i] {
        if c == 'A' {
            if g_a.union(x, y) {
                g_a_updates += 1;
            }
        }
        if g_ab.union(x, y) {
            g_ab_updates += 1;
        }
    }

    if lo + 1 == hi {
        out.print_line(g_a.num_components - g_ab.num_components);
    } else {
        let mid = (lo + hi) / 2;
        tree_traverse(tree, 2 * i, lo, mid, g_a, g_ab, out);
        tree_traverse(tree, 2 * i + 1, mid, hi, g_a, g_ab, out);
    }

    for _ in 0..g_a_updates {
        g_a.rollback();
    }
    for _ in 0..g_ab_updates {
        g_ab.rollback();
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
