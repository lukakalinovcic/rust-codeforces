//{"name":"G. Sakurako and Chefir","group":"Codeforces - Codeforces Round 981 (Div. 3)","url":"https://codeforces.com/contest/2033/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5\n1 2\n2 3\n3 4\n3 5\n3\n5 1\n3 1\n2 0\n9\n8 1\n1 7\n1 4\n7 3\n4 9\n3 2\n1 5\n3 6\n7\n6 0\n2 3\n6 2\n8 2\n2 4\n9 2\n6 3\n6\n2 1\n2 5\n2 4\n5 6\n4 3\n3\n3 1\n1 3\n6 5\n","output":"2 1 2\n0 5 2 4 5 5 5\n1 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSakurakoAndChefir"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut adj = vec![vec![]; n];
    for (a, b) in input.read_int_pair_vec(n - 1) {
        let (a, b) = (a as usize - 1, b as usize - 1);
        adj[a].push(b);
        adj[b].push(a);
    }
    let mut order = vec![0];
    let mut parent = vec![0; n];
    parent[0] = n;
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
    let mut depth = vec![0; n];
    let mut top_subtree_depths = vec![vec![]; n];
    for i in (0..n).rev() {
        let u = order[i];
        for v in &adj[u] {
            let v = *v;
            if v != parent[u] {
                depth[u] = depth[u].max(1 + depth[v]);
                top_subtree_depths[u].push((1 + depth[v], v));
                let mut k = top_subtree_depths[u].len() - 1;
                while k > 0 && top_subtree_depths[u][k].0 > top_subtree_depths[u][k - 1].0 {
                    top_subtree_depths[u].swap(k - 1, k);
                    k -= 1;
                }
                top_subtree_depths[u].truncate(2);
            }
        }
    }
    let mut dp_jump = vec![vec![]; 18];
    let mut parent_jump = vec![vec![]; 18];
    for i in 0..n {
        let p = parent[i];
        let x = if p == n {
            0
        } else {
            if top_subtree_depths[p][0].1 != i {
                1 + top_subtree_depths[p][0].0
            } else if top_subtree_depths[p].len() >= 2 {
                1 + top_subtree_depths[p][1].0
            } else {
                1
            }
        };
        dp_jump[0].push(x);
        parent_jump[0].push(p);
    }
    for layer in 1..18 {
        for i in 0..n {
            let mut x = dp_jump[layer - 1][i];
            let p = parent_jump[layer - 1][i];
            if p != n {
                x = x.max((1 << (layer - 1)) + dp_jump[layer - 1][p]);
            }
            dp_jump[layer].push(x);
            let pp = if p != n { parent_jump[layer - 1][p] } else { n };
            parent_jump[layer].push(pp);
        }
    }
    let q = input.read();
    for (v, k) in input.read_int_pair_vec(q) {
        let (mut v, mut k) = (v as usize - 1, k);

        let mut result = 0;
        if let Some(first) = top_subtree_depths[v].first() {
            result = result.max(first.0);
        }
        let mut moved_up = 0;
        for layer in (0..18).rev() {
            if k >= (1 << layer) && parent_jump[layer][v] != n {
                result = result.max(moved_up + dp_jump[layer][v]);
                k -= 1 << layer;
                moved_up += 1 << layer;
                v = parent_jump[layer][v];
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
