//{"name":"E. White Magic","group":"Codeforces - Codeforces Round 1004 (Div. 2)","url":"https://codeforces.com/contest/2067/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5\n4 3 2 1 0\n6\n4 3 3 2 1 0\n4\n2 0 1 2\n1\n777\n4\n1000000000 1 7 9\n2\n0 1\n2\n1 2\n4\n0 1 0 1\n","output":"5\n5\n3\n1\n4\n2\n2\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWhiteMagic"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

struct MySpec {}
impl LazySegTreeSpec<i32, i32> for &MySpec {
    fn identity(&self) -> i32 {
        0
    }

    fn op(&self, a: &i32, b: &i32) -> i32 {
        (*a).max(*b)
    }

    fn compose(&self, old: &i32, new: &i32) -> i32 {
        old + new
    }

    fn update(&self, t: &i32, u: &i32) -> i32 {
        t + u
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let mut pos = vec![vec![]; n + 1];
    let mut extras = 0;
    let mut result = 0;
    for i in 0..n {
        if a[i] > n as i32 {
            extras += 1;
        } else {
            pos[a[i] as usize].push(i);
        }
    }
    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, n);
    let mut remaining = n as i32 - extras;
    const INF: i32 = 1000000000;
    tree.init(|_| -INF);
    for x in 0..=n {
        remaining -= pos[x].len() as i32;
        result = result.max((*tree.all_product()).max(0) + extras + remaining);

        let mut vals = vec![];
        for &p in &pos[x] {
            vals.push(1 + tree.product(p + 1, n).max(0));
        }
        for (p, v) in pos[x].drain(..).zip(vals).rev() {
            tree.update(0, p, 1);
            tree.set(p, v);
        }
    }
    result = result.max((*tree.all_product()).max(0) + extras);
    out.print_line(result);
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
