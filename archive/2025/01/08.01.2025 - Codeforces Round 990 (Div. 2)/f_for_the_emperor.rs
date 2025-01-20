//{"name":"F. For the Emperor!","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n7 6\n2 1 0 1 2 3 4\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4 4\n1 1 1 1\n1 2\n1 3\n2 4\n3 4\n","output":"2\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FForTheEmperor"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn dfs(adj: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>, out: &mut Vec<usize>) {
    visited[u] = true;
    for v in adj[u].iter() {
        let v = *v;
        if !visited[v] {
            dfs(adj, v, visited, out);
        }
    }
    out.push(u);
}

fn scc(adj: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut adjt = vec![vec![]; n];
    for u in 0..n {
        for v in adj[u].iter() {
            adjt[*v].push(u);
        }
    }

    let mut visited = vec![false; n];
    let mut order = vec![];
    for i in 0..n {
        if !visited[i] {
            dfs(&adj, i, &mut visited, &mut order);
        }
    }
    order.reverse();

    let mut visited = vec![false; n];
    let mut comp_index = vec![0; n];
    let mut n_comp = 0 as usize;
    for u in order {
        if !visited[u] {
            let mut component = vec![];
            dfs(&adjt, u, &mut visited, &mut component);
            for v in component {
                comp_index[v] = n_comp;
            }
            n_comp += 1;
        }
    }
    (n_comp, comp_index)
}

fn condense(adj: Vec<Vec<usize>>, a: &Vec<i32>) -> (Vec<Vec<usize>>, Vec<i32>) {
    let n = adj.len();
    let (n_comp, comp_index) = scc(&adj);

    let mut comp_a = vec![0; n_comp];
    for i in 0..n {
        comp_a[comp_index[i]] += a[i];
    }

    let mut comp_adj = vec![vec![]; n_comp];
    for (u, adj_u) in adj.into_iter().enumerate() {
        for v in adj_u {
            let ui = comp_index[u];
            let vi = comp_index[v];
            if ui != vi {
                comp_adj[ui].push(vi);
            }
        }
    }
    for u in 0..n_comp {
        comp_adj[u].sort();
        comp_adj[u].dedup();
    }

    (comp_adj, comp_a)
}

fn build_flow_graph(
    adj: Vec<Vec<usize>>,
    a: Vec<i32>,
) -> (Vec<Vec<(usize, i32, i32, usize)>>, usize, usize) {
    let n = 4 + 3 * adj.len();
    let s = n - 4;
    let t = n - 3;
    let ss = n - 2;
    let tt = n - 1;
    let mut flow_adj = vec![vec![]; n];

    let mut add_edge = |u: usize, v: usize, cap: i32, cost: i32| {
        let uv_index = flow_adj[u].len();
        let vu_index = flow_adj[v].len();
        flow_adj[u].push((v, cap, cost, vu_index));
        flow_adj[v].push((u, 0, -cost, uv_index));
    };

    for (u, adj_u) in adj.into_iter().enumerate() {
        let u_in = u * 3;
        let u_out = u * 3 + 1;
        let u_s = u * 3 + 2;
        add_edge(s, u_s, a[u], 0);
        add_edge(u_s, u_in, 1, 1);
        add_edge(u_s, u_out, INF, 0);
        add_edge(u_in, u_out, INF, 0);
        add_edge(u_in, u_out, INF, 0);
        add_edge(u_out, t, INF, 0);
        add_edge(ss, u_out, 1, 0);
        add_edge(u_in, tt, 1, 0);
        for v in adj_u {
            let v_in = v * 3;
            add_edge(u_out, v_in, INF, 0);
        }
    }
    add_edge(t, s, INF, 0);

    (flow_adj, ss, tt)
}

fn shortest_path(adj: &Vec<Vec<(usize, i32, i32, usize)>>, s: usize) -> Vec<(usize, usize, usize)> {
    let n = adj.len();
    let mut dist = vec![INF; n];
    let mut pred = vec![(INF as usize, 0, 0); n];
    dist[s] = 0;
    let mut updated = true;
    while updated {
        updated = false;
        for u in 0..n {
            for (uv_index, e) in adj[u].iter().enumerate() {
                let (v, cap, cost, vu_index) = *e;
                if cap > 0 && dist[u] + cost < dist[v] {
                    dist[v] = dist[u] + cost;
                    pred[v] = (u, uv_index, vu_index);
                    updated = true;
                }
            }
        }
    }
    pred
}

fn max_flow(adj: &mut Vec<Vec<(usize, i32, i32, usize)>>, s: usize, t: usize) -> usize {
    let mut flow = 0;
    loop {
        let pred = shortest_path(adj, s);
        if pred[t].0 == INF as usize {
            break;
        }
        let mut v = t;
        while v != s {
            let (u, uv_index, vu_index) = pred[v];
            adj[u][uv_index].1 -= 1;
            adj[v][vu_index].1 += 1;
            v = u;
        }
        flow += 1;
    }
    flow
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let a = input.read_int_vec(n);
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(m) {
        let (u, v) = (u as usize - 1, v as usize - 1);
        adj[u].push(v);
    }

    let (adj, a) = condense(adj, &a);
    let n = adj.len();

    let (mut adj, s, t) = build_flow_graph(adj, a);

    let flow = max_flow(&mut adj, s, t);
    if flow != n {
        out.print_line(-1);
    } else {
        let mut result = 0;
        for adj_u in adj {
            for (_, cap, cost, _) in adj_u {
                if cost == -1 {
                    result += cap;
                }
            }
        }
        out.print_line(result);
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
