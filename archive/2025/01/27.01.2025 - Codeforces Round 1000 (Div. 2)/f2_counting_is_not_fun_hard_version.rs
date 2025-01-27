//{"name":"F2. Counting Is Not Fun (Hard Version)","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n2 5\n1 6\n3 4\n4\n1 6\n7 8\n2 3\n4 5\n6\n2 3\n1 6\n7 8\n9 12\n10 11\n4 5\n","output":"5 1 1 1\n14 2 2 1 1\n132 42 5 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2CountingIsNotFunHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

const MAXN: usize = 300000;
const MOD: u32 = 998244353;

struct PreCalc {
    fact: Vec<ModInt<MOD>>,
    inv_fact: Vec<ModInt<MOD>>,
}

impl PreCalc {
    fn new() -> Self {
        let inv = ModInt::<MOD>::gen_inverses(2 * MAXN + 1);
        let mut fact = vec![ModInt::<MOD>::from(1)];
        let mut inv_fact = vec![ModInt::<MOD>::from(1)];
        for i in 1..=2 * MAXN {
            fact.push(fact[i - 1] * i.into());
            inv_fact.push(inv_fact[i - 1] * inv[i]);
        }
        Self { fact, inv_fact }
    }

    fn catalan(self: &Self, two_n: i32) -> ModInt<MOD> {
        let n = two_n as usize / 2;
        self.fact[2 * n] * self.inv_fact[n + 1] * self.inv_fact[n]
    }

    fn inv_catalan(self: &Self, two_n: i32) -> ModInt<MOD> {
        let n = two_n as usize / 2;
        self.inv_fact[2 * n] * self.fact[n + 1] * self.fact[n]
    }
}

struct Chain<'a> {
    tree: LazySegTree<(i32, i64), i32, &'a MySpec>,
    parent: usize,
}

struct MySpec {}
impl LazySegTreeSpec<(i32, i64), i32> for &MySpec {
    fn identity(&self) -> (i32, i64) {
        (-1, 0)
    }

    fn op(&self, a: &(i32, i64), b: &(i32, i64)) -> (i32, i64) {
        (a.0.max(b.0), a.1 + b.1)
    }

    fn compose(&self, old: &i32, new: &i32) -> i32 {
        old + new
    }

    fn update(&self, t: &(i32, i64), u: &i32) -> (i32, i64) {
        (t.0, t.1 + *u as i64)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let mut a = vec![0; 2 * n];
    for (i, (l, r)) in input.read_int_pair_vec(n).into_iter().enumerate() {
        let l = l as usize - 1;
        let r = r as usize - 1;
        a[l] = i + 1;
        a[r] = i + 1;
    }
    let mut open = vec![0];
    let mut parent = vec![0; n + 1];
    let mut size = vec![0; n + 1];
    let mut heavy = vec![(n + 1, 0); n + 1];
    let mut alive = vec![false; n + 1];
    let mut order = vec![0];
    size[0] = 2 * n;
    alive[0] = true;
    for i in 0..2 * n {
        let u = a[i];
        if *open.last().unwrap() == u {
            open.pop();
            size[u] -= 2 * n - i;
            let p = *open.last().unwrap();
            parent[u] = p;
            if size[u] >= heavy[p].1 {
                heavy[p] = (u, size[u]);
            }
        } else {
            order.push(u);
            open.push(u);
            size[u] += 2 * n - i - 1;
        }
    }

    let tree_spec = MySpec {};

    let mut chains = vec![];
    let mut chain_id = vec![n + 1; n + 1];
    let mut chain_pos = vec![0; n + 1];
    for u in order {
        if chain_id[u] <= n {
            continue;
        }
        let mut chain_vals = vec![];
        let mut pos = 0;
        let mut v = u;
        while v != n + 1 {
            chain_id[v] = chains.len();
            chain_pos[v] = pos;
            chain_vals.push((if v == 0 { pos as i32 } else { -1 }, size[v] as i64));
            v = heavy[v].0;
            pos += 1;
        }
        let mut tree = LazySegTree::new(&tree_spec, chain_vals.len());
        tree.init(|i| chain_vals[i]);
        chains.push(Chain {
            tree,
            parent: parent[u],
        });
    }

    let mut result = precalc.catalan(size[0] as i32);
    let mut results = vec![result];
    for i in 1..=n {
        let uncovered_i = chains[chain_id[i]].tree.get(chain_pos[i]).1 as i32;
        result *= precalc.catalan(uncovered_i);
        let delta = -(uncovered_i + 2);
        let mut u = parent[i];
        loop {
            let chain = &mut chains[chain_id[u]];
            let p = chain.tree.product(0, chain_pos[u] + 1).0;
            if p == -1 {
                chain.tree.update(0, chain_pos[u] + 1, delta);
                u = chain.parent;
            } else {
                chain.tree.update(p as usize, chain_pos[u] + 1, delta);
                let uncovered_j = chain.tree.get(p as usize).1 as i32;
                result *= precalc.inv_catalan(uncovered_j - delta);
                result *= precalc.catalan(uncovered_j);
                break;
            }
        }
        chains[chain_id[i]]
            .tree
            .set(chain_pos[i], (chain_pos[i] as i32, uncovered_i as i64));

        results.push(result);
    }
    out.print_iter(results.into_iter().map(|r| r.unwrap()));
    out.print_line("");
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

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
