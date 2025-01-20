//{"name":"E1. Digital Village (Easy Version)","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n9 8 5\n2 5 6 8 9\n1 2 1\n1 3 2\n3 4 10\n4 5 3\n4 6 5\n1 7 10\n7 8 4\n7 9 2\n3 3 2\n3 1\n1 2 1\n2 3 3\n1 3 2\n","output":"34 19 9 4 0 0 0 0 0\n2 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1DigitalVillageEasyVersion"}}}

use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

struct DsuNode {
    parent: usize,
    rank: i32,
    num_marked: i32,
}

fn dsu_find(dsu: &mut Vec<DsuNode>, u: usize) -> usize {
    if dsu[u].parent != u {
        dsu[u].parent = dsu_find(dsu, dsu[u].parent);
    }
    dsu[u].parent
}

fn dsu_merge(dsu: &mut Vec<DsuNode>, mut u: usize, mut v: usize) -> usize {
    if dsu[u].rank < dsu[v].rank {
        swap(&mut u, &mut v);
    }
    if dsu[u].rank == dsu[v].rank {
        dsu[u].rank += 1;
    }
    dsu[v].parent = u;
    dsu[u].num_marked += dsu[v].num_marked;
    u
}

fn dp_merge(dp_a: Vec<i64>, dp_b: Vec<i64>, num_a: usize, num_b: usize, w: i64) -> Vec<i64> {
    let n = num_a + num_b;
    let mut dp = vec![INF; n + 1];
    for i in 1..=n {
        if i < dp_a.len() {
            dp[i] = dp[i].min(dp_a[i] + num_b as i64 * w);
        }
        if i < dp_b.len() {
            dp[i] = dp[i].min(dp_b[i] + num_a as i64 * w);
        }
        dp[i] = dp[i].min(dp[i - 1]);
    }
    for i in 1..=num_a {
        for j in 1..=num_b {
            dp[i + j] = dp[i + j].min(dp_a[i] + dp_b[j]);
        }
    }
    dp
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let p = input.read();

    let mut dsu = vec![];
    let mut dp = vec![vec![]; n];
    for i in 0..n {
        dsu.push(DsuNode {
            parent: i,
            rank: 0,
            num_marked: 0,
        });
    }
    for s in input.read_int_vec(p) {
        let s = s as usize - 1;
        dsu[s].num_marked = 1;
        dp[s] = vec![INF, 0];
    }

    let mut edges = vec![];
    for _ in 0..m {
        let u = input.read_int() as usize - 1;
        let v = input.read_int() as usize - 1;
        let w = input.read_int();
        edges.push((u, v, w));
    }
    edges.sort_by_key(|(_, _, w)| *w);
    for (u, v, w) in edges {
        let u = dsu_find(&mut dsu, u);
        let v = dsu_find(&mut dsu, v);
        if u == v {
            continue;
        }
        let num_a = dsu[u].num_marked;
        let num_b = dsu[v].num_marked;
        let mut dp_a: Vec<i64> = vec![];
        let mut dp_b: Vec<i64> = vec![];
        swap(&mut dp_a, &mut dp[u]);
        swap(&mut dp_b, &mut dp[v]);
        let u = dsu_merge(&mut dsu, u, v);

        if num_a == 0 {
            dp[u] = dp_b;
        } else if num_b == 0 {
            dp[u] = dp_a;
        } else {
            dp[u] = dp_merge(dp_a, dp_b, num_a as usize, num_b as usize, w as i64);
        }
    }
    let u = dsu_find(&mut dsu, 0);
    out.print_iter((1..=n).map(|k| if k < dp[u].len() { dp[u][k] } else { 0 }));
    out.print_line("");
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
