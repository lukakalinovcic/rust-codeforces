//{"name":"F1. Frequency Mismatch (Easy Version)","group":"Codeforces - Codeforces Round 940 (Div. 2) and CodeCraft-23","url":"https://codeforces.com/contest/1957/problem/F1","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n5 2 3 4 3\n1 2\n1 3\n2 4\n2 5\n3\n1 4 4 5 1\n2 3 2 3 1\n5 5 4 3 1\n","output":"1 5\n0\n1 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1FrequencyMismatchEasyVersion"}}}

use std::i32;

use algo_lib::graph::lca::LCA;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

type VV = [i32; NUM_HASHES];

#[derive(Clone)]
struct Node<V: Default + Copy> {
    ch: Option<(i32, i32)>,
    v: V,
}

struct PersistentSegTreeForest<V: Default + Copy> {
    nodes: Vec<Node<V>>,
}

impl<V: Default + Copy> PersistentSegTreeForest<V> {
    fn new() -> Self {
        Self { nodes: vec![] }
    }

    fn get_value(&self, i: Option<i32>) -> V {
        match i {
            None => V::default(),
            Some(i) => self.nodes[i as usize].v.clone(),
        }
    }

    fn left_child(&self, i: Option<i32>) -> Option<i32> {
        match i {
            None => None,
            Some(i) => match self.nodes[i as usize].ch {
                None => None,
                Some((lt, _rt)) => Some(lt),
            },
        }
    }

    fn right_child(&self, i: Option<i32>) -> Option<i32> {
        match i {
            None => None,
            Some(i) => match self.nodes[i as usize].ch {
                None => None,
                Some((_lt, rt)) => Some(rt),
            },
        }
    }

    fn new_node(&mut self, v: V, ch: Option<(i32, i32)>) -> i32 {
        let x = self.nodes.len() as i32;
        self.nodes.push(Node { ch, v });
        x
    }

    fn add_value(
        &mut self,
        i: Option<i32>,
        lo: usize,
        hi: usize,
        p: usize,
        v: V,
        adder: &impl Fn(V, V) -> V,
    ) -> i32 {
        if p < lo || p >= hi {
            return match i {
                None => self.new_node(V::default(), None),
                Some(i) => i,
            };
        }
        if hi - lo == 1 {
            return match i {
                None => self.new_node(v, None),
                Some(i) => self.new_node(adder(v, self.nodes[i as usize].v), None),
            };
        }
        let (lt, rt) = match i {
            None => (None, None),
            Some(i) => match self.nodes[i as usize].ch {
                None => (None, None),
                Some((lt, rt)) => (Some(lt), Some(rt)),
            },
        };
        let mid = (lo + hi) / 2;
        let lt = self.add_value(lt, lo, mid, p, v, adder);
        let rt = self.add_value(rt, mid, hi, p, v, adder);

        let lv = self.nodes[lt as usize].v;
        let rv = self.nodes[rt as usize].v;
        self.new_node((adder)(lv, rv), Some((lt, rt)))
    }
}

const MAXVAL: usize = 100000;
const NUM_HASHES: usize = 2;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let hash_params = pick_hash_params(1000000000);
    let mut h = vec![VV::default(); MAXVAL + 1];
    for j in 0..NUM_HASHES {
        h[0][j] = hash_params[j].0;
    }
    for i in 1..=MAXVAL {
        for j in 0..NUM_HASHES {
            h[i][j] =
                ((h[i - 1][j] as i64 * hash_params[j].0 as i64) % hash_params[j].1 as i64) as i32;
        }
    }
    let adder = move |a: VV, b: VV| {
        let mut c = VV::default();
        for i in 0..NUM_HASHES {
            c[i] = (a[i] + b[i]) % hash_params[i].1;
        }
        c
    };
    let suber = move |a: VV, b: VV| {
        let mut c = VV::default();
        for i in 0..NUM_HASHES {
            c[i] = (a[i] + hash_params[i].1 - b[i]) % hash_params[i].1;
        }
        c
    };

    let mut adj = vec![vec![]; n + 1];
    adj[0].push(1);
    for (u, v) in input.read_int_pair_vec(n - 1) {
        adj[u as usize].push(v);
        adj[v as usize].push(u);
    }

    let mut forest = PersistentSegTreeForest::new();
    let mut roots = vec![None; n + 1];
    dfs(&adj, 0, 0, &a, &h, &mut forest, &mut roots, &adder);

    let lca = LCA::new(&adj, 0);
    let q = input.read();
    for _ in 0..q {
        let u1 = input.read_int();
        let v1 = input.read_int();
        let u2 = input.read_int();
        let v2 = input.read_int();
        let k = input.read_size();

        let lca1 = lca.query(u1, v1);
        let lca2 = lca.query(u2, v2);
        let lca1_parent = lca.parent(lca1).unwrap();
        let lca2_parent = lca.parent(lca2).unwrap();
        let p1 = Path {
            u: roots[u1 as usize],
            v: roots[v1 as usize],
            lca: roots[lca1 as usize],
            lca_parent: roots[lca1_parent as usize],
        };
        let p2 = Path {
            u: roots[u2 as usize],
            v: roots[v2 as usize],
            lca: roots[lca2 as usize],
            lca_parent: roots[lca2_parent as usize],
        };
        let mut result = vec![];
        traverse(
            p1,
            p2,
            1,
            MAXVAL + 1,
            k,
            &forest,
            &adder,
            &suber,
            &mut result,
        );
        out.print(result.len());
        out.print(" ");
        out.print_iter(result.into_iter());
        out.print_line("");
    }
}

struct Path {
    u: Option<i32>,
    v: Option<i32>,
    lca: Option<i32>,
    lca_parent: Option<i32>,
}

impl Path {
    fn left(&self, forest: &PersistentSegTreeForest<VV>) -> Path {
        Path {
            u: forest.left_child(self.u),
            v: forest.left_child(self.v),
            lca: forest.left_child(self.lca),
            lca_parent: forest.left_child(self.lca_parent),
        }
    }

    fn right(&self, forest: &PersistentSegTreeForest<VV>) -> Path {
        Path {
            u: forest.right_child(self.u),
            v: forest.right_child(self.v),
            lca: forest.right_child(self.lca),
            lca_parent: forest.right_child(self.lca_parent),
        }
    }

    fn path_value(
        &self,
        forest: &PersistentSegTreeForest<VV>,
        adder: &impl Fn(VV, VV) -> VV,
        suber: &impl Fn(VV, VV) -> VV,
    ) -> VV {
        let u = forest.get_value(self.u);
        let v = forest.get_value(self.v);
        let lca = forest.get_value(self.lca);
        let lca_parent = forest.get_value(self.lca_parent);
        suber(adder(u, v), adder(lca, lca_parent))
    }
}

fn traverse(
    p1: Path,
    p2: Path,
    lo: usize,
    hi: usize,
    k: usize,
    forest: &PersistentSegTreeForest<VV>,
    adder: &impl Fn(VV, VV) -> VV,
    suber: &impl Fn(VV, VV) -> VV,
    out: &mut Vec<i32>,
) {
    if p1.path_value(forest, adder, suber) == p2.path_value(forest, adder, suber) {
        return;
    }
    if out.len() == k {
        return;
    }
    if hi - lo == 1 {
        out.push(lo as i32);
        return;
    }
    let mid = (lo + hi) / 2;
    traverse(
        p1.left(forest),
        p2.left(forest),
        lo,
        mid,
        k,
        forest,
        adder,
        suber,
        out,
    );
    traverse(
        p1.right(forest),
        p2.right(forest),
        mid,
        hi,
        k,
        forest,
        adder,
        suber,
        out,
    );
}

fn dfs(
    adj: &Vec<Vec<i32>>,
    u: i32,
    p: i32,
    a: &Vec<i32>,
    h: &Vec<VV>,
    forest: &mut PersistentSegTreeForest<VV>,
    roots: &mut Vec<Option<i32>>,
    adder: &impl Fn(VV, VV) -> VV,
) {
    for &v in &adj[u as usize] {
        if v == p {
            continue;
        }
        let i = a[v as usize - 1] as usize;
        roots[v as usize] =
            Some(forest.add_value(roots[u as usize], 1, MAXVAL + 1, i, h[i], adder));
        dfs(adj, v, u, a, h, forest, roots, adder);
    }
}

fn small_rand(prev: u16) -> u16 {
    let x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let h1 = x >> 16;
    let h2 = x & 0xffff;
    let curr = (h1 ^ h2) as u16;
    (prev << 8) ^ (prev >> 8) ^ curr ^ 0xcd15
}

fn is_prime(p: i32) -> bool {
    for d in 2.. {
        if p % d == 0 {
            return false;
        }
        if d * d > p {
            break;
        }
    }
    true
}

fn pick_hash_params(prime_lo: i32) -> [(i32, i32); NUM_HASHES] {
    let mut params = [(0, 0); NUM_HASHES];
    let mut r = small_rand(12345);
    let mut p = prime_lo + (r as i32) % 31337;
    let mut i = 0;
    while i < NUM_HASHES {
        if is_prime(p) {
            let b = p - 1337 - r as i32;
            params[i] = (b, p);
            i += 1;
            r = small_rand(r);
            p += (r as i32) % 31337;
        }
        p += 1;
    }
    params
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
