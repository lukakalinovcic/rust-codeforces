//{"name":"F. Choose Your Queries","group":"Codeforces - Educational Codeforces Round 170 (Rated for Div. 2)","url":"https://codeforces.com/contest/2025/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3 4\n1 2\n3 2\n3 1\n1 2\n","output":"y+\nx+\nx-\ny-\n"},{"input":"4 4\n1 2\n2 3\n3 4\n3 2\n","output":"y+\ny+\nx-\ny-\n"},{"input":"4 2\n2 1\n4 3\n","output":"y+\nx+\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FChooseYourQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Query {
    x: i32,
    y: i32,
    selected: i32,
}

fn dfs(qs: &Vec<Query>, adj: &Vec<Vec<i32>>, u: i32, parent: &mut Vec<i32>, out: &mut Vec<i32>) {
    out.push(u);
    for qi in &adj[u as usize] {
        let qi = *qi;
        let v = if qs[qi as usize].x == u {
            qs[qi as usize].y
        } else {
            qs[qi as usize].x
        };
        if parent[v as usize] == -1 {
            parent[v as usize] = qi;
            dfs(qs, adj, v, parent, out);
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q: usize = input.read();
    let mut adj = vec![vec![]; n];
    let mut qs = (0..q)
        .map(|i| {
            let x = input.read_int() - 1;
            let y = input.read_int() - 1;
            adj[x as usize].push(i as i32);
            adj[y as usize].push(i as i32);
            Query { x, y, selected: -1 }
        })
        .collect::<Vec<_>>();

    let mut order = vec![];
    let mut parent = vec![-1; n];
    for k in 0..n as i32 {
        if parent[k as usize] == -1 {
            dfs(&qs, &adj, k, &mut parent, &mut order);
        }
    }

    for u in order.into_iter().rev() {
        let mut prev = -1;
        for qi in &adj[u as usize] {
            let qi = *qi;
            if qi == parent[u as usize] {
                continue;
            }
            if qs[qi as usize].selected != -1 {
                continue;
            }
            if prev == -1 {
                prev = qi;
            } else {
                qs[qi as usize].selected = u;
                qs[prev as usize].selected = u;
                prev = -1;
            }
        }
        if prev != -1 && parent[u as usize] != -1 {
            qs[parent[u as usize] as usize].selected = u;
            qs[prev as usize].selected = u;
        }
    }

    let mut cnt = vec![0; n];
    for query in qs {
        if query.selected == -1 {
            out.print_line("x+");
        } else {
            cnt[query.selected as usize] += 1;
            let ch1 = if query.x == query.selected { 'x' } else { 'y' };
            let ch2 = if cnt[query.selected as usize] % 2 == 1 {
                '+'
            } else {
                '-'
            };
            out.print(ch1);
            out.print(ch2);
            out.print_line("");
        }
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
