//{"name":"D. Determine Winning Islands in Race","group":"Codeforces - Codeforces Round 965 (Div. 2)","url":"https://codeforces.com/contest/1998/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n6 0\n6 1\n2 6\n6 1\n1 5\n10 4\n1 3\n1 6\n2 7\n3 8\n15 3\n2 8\n4 9\n8 15\n","output":"11111\n11011\n10011\n100001111\n11000111000111\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDetermineWinningIslandsInRace"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut adj = vec![vec![]; n];
    for i in 1..n {
        adj[i - 1].push(i);
    }
    for (u, v) in input.read_int_pair_vec(m) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
    }
    let mut dist = vec![n as i32; n];
    dist[0] = 0;
    let mut q = vec![0];
    for i in 0..n {
        let u = q[i];
        for &v in &adj[u] {
            if dist[v] > dist[u] + 1 {
                dist[v] = dist[u] + 1;
                q.push(v);
            }
        }
    }

    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, n - 1);
    let mut result = vec![];
    for u in 0..n - 1 {
        for &v in &adj[u] {
            let lo = u as i32 + 1;
            let hi = v as i32 - dist[u] - 1;
            if lo < hi {
                tree.update(lo as usize, hi as usize, 1);
            }
        }
        if *tree.get(u) == 0 {
            result.push(b'1');
        } else {
            result.push(b'0');
        }
    }
    out.print_line(String::from_utf8(result).unwrap());
}

struct MySpec {}
impl LazySegTreeSpec<i64, i32> for &MySpec {
    fn identity(&self) -> i64 {
        0
    }

    fn op(&self, a: &i64, b: &i64) -> i64 {
        a + b
    }

    fn compose(&self, old: &i32, new: &i32) -> i32 {
        old + new
    }

    fn update(&self, t: &i64, u: &i32) -> i64 {
        t + *u as i64
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
