//{"name":"E3. Digital Village (Extreme Version)","group":"Codeforces - Codeforces Round 977 (Div. 2, based on COMPFEST 16 - Final Round)","url":"https://codeforces.com/contest/2021/problem/E3","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n9 8 5\n2 5 6 8 9\n1 2 1\n1 3 2\n3 4 10\n4 5 3\n4 6 5\n1 7 10\n7 8 4\n7 9 2\n3 3 2\n3 1\n1 2 1\n2 3 3\n1 3 2\n","output":"34 19 9 4 0 0 0 0 0\n2 0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E3DigitalVillageExtremeVersion"}}}

use std::collections::BinaryHeap;
use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct DsuNode {
    parent: usize,
    rank: i32,
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
    u
}

#[derive(Debug)]
struct TreeNode {
    sz: i64,
    w: i32,
    dp: i64,
    ch1: i32,
    ch2: i32,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let p = input.read();

    let mut dsu = vec![];
    let mut dsu_to_root = vec![-1; n];
    let mut tree = vec![];
    for i in 0..n {
        dsu.push(DsuNode { parent: i, rank: 0 });
    }
    for s in input.read_int_vec(p) {
        let s = s as usize - 1;
        dsu_to_root[s] = tree.len() as i32;
        tree.push(TreeNode {
            dp: 0,
            sz: 1,
            w: 0,
            ch1: -1,
            ch2: -1,
        });
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
        let root_a = dsu_to_root[u];
        let root_b = dsu_to_root[v];
        let u = dsu_merge(&mut dsu, u, v);

        if root_a == -1 {
            dsu_to_root[u] = root_b;
        } else if root_b == -1 {
            dsu_to_root[u] = root_a;
        } else {
            let sz = tree[root_a as usize].sz + tree[root_b as usize].sz;
            let cost_a = tree[root_a as usize].dp + w as i64 * tree[root_b as usize].sz;
            let cost_b = tree[root_b as usize].dp + w as i64 * tree[root_a as usize].sz;
            let (dp, ch1, ch2) = if cost_a < cost_b {
                (cost_a, root_a, root_b)
            } else {
                (cost_b, root_b, root_a)
            };
            dsu_to_root[u] = tree.len() as i32;
            tree.push(TreeNode {
                sz,
                dp,
                ch1,
                ch2,
                w,
            })
        }
    }

    let mut result = vec![];
    let root = tree.len() as i32 - 1;
    let mut pq = BinaryHeap::new();
    pq.push((-tree[root as usize].dp, root));
    while !pq.is_empty() {
        let (cost, mut u) = pq.pop().unwrap();
        let cost = -cost;
        result.push(result.last().cloned().unwrap_or_default() + cost);
        loop {
            let (ch1, ch2, w) = (
                tree[u as usize].ch1,
                tree[u as usize].ch2,
                tree[u as usize].w,
            );
            if ch1 == -1 {
                break;
            }
            pq.push((
                -(tree[ch2 as usize].dp - tree[ch2 as usize].sz as i64 * w as i64),
                ch2,
            ));
            u = ch1;
        }
    }

    while result.len() < n {
        result.push(0);
    }
    out.print_iter(result.into_iter());
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
