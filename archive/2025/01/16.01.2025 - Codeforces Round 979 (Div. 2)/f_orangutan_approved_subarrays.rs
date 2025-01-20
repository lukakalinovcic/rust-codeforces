//{"name":"F. Orangutan Approved Subarrays","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n4 2\n1 2 2 1\n1 4\n1 3\n5 3\n1 2 1 2 1\n2 5\n3 5\n1 3\n8 4\n1 2 3 2 1 3 2 3\n1 5\n2 8\n3 5\n6 8\n","output":"YES\nYES\nNO\nYES\nYES\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FOrangutanApprovedSubarrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

struct MySpec {}
impl LazySegTreeSpec<i32, ()> for &MySpec {
    fn identity(&self) -> i32 {
        -1
    }

    fn op(&self, a: &i32, b: &i32) -> i32 {
        (*a).max(*b)
    }

    fn compose(&self, _old: &(), _new: &()) -> () {
        ()
    }

    fn update(&self, t: &i32, _u: &()) -> i32 {
        *t
    }
}

struct Query {
    l: usize,
    r: usize,
    i: usize,
}

fn doit(a: Vec<i32>, qs: Vec<(i32, i32)>) -> Vec<bool> {
    let n = a.len();
    let q = qs.len();

    let mut qs = qs
        .into_iter()
        .enumerate()
        .map(|(i, (l, r))| Query {
            l: l as usize - 1,
            r: r as usize - 1,
            i,
        })
        .collect::<Vec<_>>();
    qs.sort_by_key(|query| query.r);

    let tree_spec = MySpec {};
    let mut result = vec![false; q];
    let mut last = vec![-1; n + 1];
    let mut next = LazySegTree::new(&tree_spec, n);
    next.init(|_| -1);

    let mut next_q = 0;
    let mut i = 0;
    for j in 0..n {
        let aj = a[j] as usize;
        let prev = if last[aj] >= 0 {
            let prev = last[aj];
            next.set(prev as usize, j as i32);
            prev
        } else {
            -1
        };
        last[aj] = j as i32;
        while prev > i as i32 && next.product(i, prev as usize) > prev {
            i += 1;
        }
        while next_q < qs.len() && qs[next_q].r == j {
            result[qs[next_q].i] = qs[next_q].l >= i;
            next_q += 1;
        }
    }
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let qs = input.read_int_pair_vec(q);
    let result = doit(a, qs);
    for i in 0..q {
        if result[i] {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
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
