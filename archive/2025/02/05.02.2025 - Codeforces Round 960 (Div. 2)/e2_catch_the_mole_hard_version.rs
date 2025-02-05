//{"name":"E2. Catch the Mole(Hard Version)","group":"Codeforces - Codeforces Round 960 (Div. 2)","url":"https://codeforces.com/contest/1990/problem/E2","interactive":true,"timeLimit":4000,"tests":[{"input":"2\n2\n1 2\n\n1\n\n6\n1 2\n1 3\n1 4\n4 5\n5 6\n\n0\n\n0\n\n1\n","output":"\n\n? 2\n\n! 2\n\n\n\n\n\n\n? 2\n\n? 6\n\n? 4\n\n! 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2CatchTheMoleHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn query(input: &mut Input, out: &mut Output, x: i32) -> i32 {
    out.print_line(format!("? {x}"));
    out.flush();
    input.read_int()
}

fn report(out: &mut Output, x: i32) {
    out.print_line(format!("! {x}"));
    out.flush();
}

enum Mode<'a> {
    IO(Input<'a>, Output<'a>),
    Given(Vec<i32>, i32),
}

struct Interactor<'a> {
    mode: Mode<'a>,
    num_queries: i32,
    curr: i32,
}

impl<'a> Interactor<'a> {
    fn new(mode: Mode<'a>) -> Self {
        Self {
            mode,
            num_queries: 0,
            curr: 0,
        }
    }

    fn get_input(&mut self) -> (usize, Vec<(i32, i32)>) {
        self.num_queries = 0;
        match &mut self.mode {
            Mode::IO(input, _) => {
                let n = input.read_size();
                let e = input.read_int_pair_vec(n - 1);
                (n, e)
            }
            Mode::Given(parent, start) => {
                self.curr = *start;
                let n = parent.len() - 1;
                let mut e = vec![];
                for i in 2..=n {
                    e.push((i as i32, parent[i]));
                }
                (parent.len() - 1, e)
            }
        }
    }

    fn query(&mut self, x: i32) -> bool {
        self.num_queries += 1;
        if self.num_queries > 300 {
            panic!("too many queries");
        }
        match &mut self.mode {
            Mode::IO(input, out) => query(input, out, x) == 1,
            Mode::Given(parent, _) => {
                let mut y = self.curr;
                while y != 0 {
                    if y == x {
                        return true;
                    }
                    y = parent[y as usize];
                }
                if parent[self.curr as usize] != 0 {
                    self.curr = parent[self.curr as usize];
                }
                false
            }
        }
    }

    fn report(&mut self, x: i32) {
        match &mut self.mode {
            Mode::IO(_input, out) => report(out, x),
            Mode::Given(parent, start) => {
                if x == self.curr {
                    // OK.
                } else {
                    panic!("Wrong answer for {parent:?} {start}");
                }
            }
        }
    }
}

fn solve(interactor: &mut Interactor) {
    let (n, e) = interactor.get_input();
    let mut adj = vec![vec![]; n + 1];
    for (u, v) in e {
        adj[u as usize].push(v as usize);
        adj[v as usize].push(u as usize);
    }
    let mut parent = vec![0; n + 1];
    let mut depth = vec![0; n + 1];
    let mut bfs_order = vec![1];
    let mut deg = vec![0; n + 1];
    let mut alive = vec![true; n + 1];
    depth[1] = 1;
    for i in 0..n {
        let u = bfs_order[i];
        for &v in &adj[u] {
            if v != 1 && depth[v] == 0 {
                depth[v] = depth[u] + 1;
                parent[v] = u;
                deg[u] += 1;
                bfs_order.push(v);
            }
        }
    }
    let mut dfs_ctx = DfsContext::new(n + 1);
    dfs(&adj, 1, 1, &mut dfs_ctx);

    let mut leaves = vec![];
    for i in 1..=n {
        if deg[i] == 0 {
            leaves.push(i);
        }
    }

    let mut root = 1;
    for u in bfs_order.into_iter().rev() {
        while alive[u] {
            let delta = depth[u] - depth[root];
            if delta == 0 {
                interactor.report(u as i32);
                return;
            }
            let mut v = u;
            for _ in 0..delta / 2 {
                v = parent[v];
            }
            let mut next_leaves = vec![];
            if interactor.query(v as i32) {
                for leaf in leaves {
                    if dfs_ctx.is_ancestor(v, leaf) {
                        next_leaves.push(leaf);
                    } else {
                        let mut t = leaf;
                        while t != 0 && alive[t] {
                            alive[t] = false;
                            deg[parent[t]] -= 1;
                            t = parent[t];
                        }
                    }
                }
                root = v;
            } else {
                let vp_deg = deg[parent[v]];
                for leaf in leaves {
                    if dfs_ctx.is_ancestor(v, leaf) {
                        let mut t = leaf;
                        while t != v && alive[t] {
                            alive[t] = false;
                            deg[parent[t]] -= 1;
                            t = parent[t];
                        }
                    } else {
                        alive[leaf] = false;
                        let p = parent[leaf];
                        if p != 0 {
                            deg[p] -= 1;
                            if alive[p] && deg[p] == 0 {
                                next_leaves.push(p);
                            }
                        }
                    }
                }
                {
                    alive[v] = false;
                    let p = parent[v];
                    if p != 0 {
                        deg[p] -= 1;
                        if vp_deg == 1 {
                            let pp = parent[p];
                            if pp != 0 {
                                alive[p] = false;
                                deg[pp] -= 1;
                                if alive[pp] && deg[pp] == 0 {
                                    next_leaves.push(pp);
                                }
                            }
                        } else {
                            if alive[p] && deg[p] == 0 {
                                next_leaves.push(p);
                            }
                        }
                    }
                }

                if parent[root] != 0 {
                    root = parent[root];
                    if !alive[root] {
                        alive[root] = true;
                        deg[parent[root]] += 1;
                    }
                }
            }
            leaves = next_leaves;
        }
    }
    panic!("should not happen");
}

#[derive(Debug)]
struct DfsContext {
    t: i32,
    enter: Vec<i32>,
    exit: Vec<i32>,
}

impl DfsContext {
    fn new(n: usize) -> Self {
        Self {
            t: 0,
            enter: vec![0; n],
            exit: vec![0; n],
        }
    }

    fn is_ancestor(&self, u: usize, v: usize) -> bool {
        self.enter[u] <= self.enter[v] && self.exit[u] >= self.exit[v]
    }
}

fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, ctx: &mut DfsContext) {
    ctx.enter[u] = ctx.t;
    ctx.t += 1;
    for v in &adj[u] {
        let v = *v;
        if v == p {
            continue;
        }
        dfs(adj, v, u, ctx);
    }
    ctx.exit[u] = ctx.t;
    ctx.t += 1;
}

pub(crate) fn run<'a>(mut input: Input<'a>, output: Output<'a>) -> bool {
    let t = input.read();
    let mut interactor = Interactor::new(Mode::IO(input, output));
    for _ in 0..t {
        solve(&mut interactor);
    }
    true
}

//START MAIN
mod tester;
fn main() {
    if std::env::args().find(|x| x == "io").is_some() {
        let mut sin = std::io::stdin();
        let input = algo_lib::io::input::Input::new(&mut sin);
        let mut stdout = std::io::stdout();
        let output = algo_lib::io::output::Output::new(&mut stdout);
        run(input, output);
    } else {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for test_num in 0.. {
            if test_num % 1000 == 0 {
                eprintln!("test num: {test_num}");
            }
            let n = rng.gen_range(2..=5000);
            let mut parent = vec![0; n + 1];
            for i in 2..=n {
                parent[i] = rng.gen_range(1..i as i32);
            }
            let start = rng.gen_range(1..=n as i32);
            let mut interactor = Interactor::new(Mode::Given(parent, start));
            solve(&mut interactor);
        }
    }
}
//END MAIN
