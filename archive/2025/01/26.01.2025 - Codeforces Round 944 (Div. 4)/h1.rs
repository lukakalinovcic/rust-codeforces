//{"name":"H. ±1","group":"Codeforces - Codeforces Round 944 (Div. 4)","url":"https://codeforces.com/contest/1971/problem/H","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 -2 -3 -2\n-4 4 -1 -3\n1 2 -2 4\n2\n1 2\n-1 -2\n2 -2\n5\n1 2 3 4 5\n-2 3 -4 -5 -1\n3 -5 1 2 2\n6\n1 3 -6 2 5 2\n1 3 -2 -3 -6 -5\n-2 -1 -3 2 3 1\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"H1"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

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

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a0 = input.read_int_vec(n);
    let a1 = input.read_int_vec(n);
    let a2 = input.read_int_vec(n);

    let node = |x: i32| if x < 0 { (-x) * 2 - 2 } else { x * 2 - 1 } as usize;
    let not = |x| x ^ 1;

    let mut adj = vec![vec![]; 2 * n];
    let mut add_term = |a, b| {
        let a = node(a);
        let b = node(b);
        adj[not(a)].push(b);
        adj[not(b)].push(a);
    };
    for i in 0..n {
        add_term(a0[i], a1[i]);
        add_term(a0[i], a2[i]);
        add_term(a1[i], a2[i]);
    }
    let (_, comp) = scc(&adj);
    for i in (0..2 * n).step_by(2) {
        if comp[i] == comp[i + 1] {
            out.print_line("NO");
            return;
        }
    }
    out.print_line("YES");
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
