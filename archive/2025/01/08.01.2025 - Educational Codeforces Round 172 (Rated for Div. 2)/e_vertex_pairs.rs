//{"name":"E. Vertex Pairs","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/contest/2042/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n1 1 3 2 3 2\n4 2\n1 6\n6 2\n6 3\n2 5\n","output":"3\n2 4 5\n"},{"input":"3\n2 3 1 3 2 1\n6 4\n2 4\n5 2\n3 6\n3 1\n","output":"4\n1 3 4 6\n"},{"input":"6\n5 2 3 4 6 4 2 5 6 1 1 3\n10 8\n2 10\n12 7\n4 10\n5 9\n6 2\n1 9\n3 4\n12 6\n11 5\n4 5\n","output":"6\n2 3 4 5 8 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EVertexPairs"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, color: &Vec<i32>, out: &mut Vec<i32>) {
    out.push(color[u]);
    for v in adj[u].iter() {
        let v = *v;
        if v != p {
            dfs(adj, v, u, color, out);
        }
    }
}

fn merge(mut a: HashSet<i32>, mut b: HashSet<i32>) -> HashSet<i32> {
    if a.len() > b.len() {
        for x in b {
            a.insert(x);
        }
        a
    } else {
        for x in a {
            b.insert(x);
        }
        b
    }
}

fn root_tree(
    adj: &Vec<Vec<usize>>,
    u: usize,
    p: usize,
    color: &Vec<i32>,
    keep: &mut Vec<bool>,
    parent: &mut Vec<usize>,
) -> HashSet<i32> {
    let mut u_keep = false;
    let mut u_seen_colors = HashSet::new();
    parent[u] = p;
    for v in adj[u].iter() {
        let v = *v;
        if v == p {
            continue;
        }
        let v_seen_colors = root_tree(adj, v, u, color, keep, parent);
        u_keep |= keep[v];
        let pre_merge_len = u_seen_colors.len() + v_seen_colors.len();
        u_seen_colors = merge(u_seen_colors, v_seen_colors);
        if u_seen_colors.len() < pre_merge_len {
            u_keep = true;
        }
    }
    if !u_seen_colors.insert(color[u]) {
        u_keep = true;
    }
    keep[u] = u_keep;
    u_seen_colors
}

fn mark_dead(
    adj: &Vec<Vec<usize>>,
    u: usize,
    p: usize,
    dead: &mut Vec<bool>,
    out: &mut Vec<usize>,
) {
    dead[u] = true;
    out.push(u);
    for v in adj[u].iter() {
        let v = *v;
        if v != p && !dead[v] {
            mark_dead(adj, v, u, dead, out);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let color = input.read_int_vec(2 * n);
    let mut pos = vec![2 * n; n + 1];
    let mut other = vec![0; 2 * n];
    for (i, c) in color.iter().enumerate() {
        let c = (*c) as usize;
        if pos[c] < i {
            other[i] = pos[c];
            other[pos[c]] = i;
        } else {
            pos[c] = i;
        }
    }

    let mut adj = vec![vec![]; 2 * n];
    for (a, b) in input.read_int_pair_vec(2 * n - 1) {
        let (a, b) = (a as usize - 1, b as usize - 1);
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut can_kill_last = false;
    for v in adj[2 * n - 1].iter() {
        let v = *v;
        let mut subtree_colors = vec![];
        dfs(&adj, v, 2 * n - 1, &color, &mut subtree_colors);
        subtree_colors.sort();
        subtree_colors.dedup();
        if subtree_colors.len() == n {
            can_kill_last = true;
        }
    }
    let root = if can_kill_last == false {
        2 * n - 1
    } else {
        other[2 * n - 1]
    };

    let mut parent = vec![0; 2 * n];
    let mut keep = vec![false; 2 * n];
    let mut dead = vec![false; 2 * n];
    root_tree(&adj, root, 2 * n, &color, &mut keep, &mut parent);
    let mut result = vec![];
    for u in (0..2 * n).rev() {
        if keep[u] {
            result.push(u + 1);
        } else {
            let mut marked = vec![];
            mark_dead(&adj, u, parent[u], &mut dead, &mut marked);
            for d in marked {
                let mut v = other[d];
                while !keep[v] {
                    keep[v] = true;
                    v = parent[v];
                }
            }
        }
    }
    result.sort();
    out.print_line(result.len());
    out.print_iter(result.into_iter());
    out.put(b'\n');
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
