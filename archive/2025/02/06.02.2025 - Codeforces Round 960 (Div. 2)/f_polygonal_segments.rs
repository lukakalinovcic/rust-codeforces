//{"name":"F. Polygonal Segments","group":"Codeforces - Codeforces Round 960 (Div. 2)","url":"https://codeforces.com/contest/1990/problem/F","interactive":false,"timeLimit":8000,"tests":[{"input":"2\n5 6\n3 1 2 2 8\n1 1 3\n1 1 4\n1 1 5\n2 1 5\n1 1 4\n1 1 5\n4 10\n500000000000 500000000000 1000000000000 500000000000\n1 1 3\n1 2 4\n1 1 4\n2 1 499999999999\n2 3 999999999999\n1 1 3\n1 2 4\n1 1 4\n2 3 1000000000000\n1 1 3\n","output":"-1\n4\n4\n3\n5\n-1\n-1\n4\n-1\n3\n4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FPolygonalSegments"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

#[derive(Clone, Debug)]
struct Anchor {
    pos: i32,
    val: i64,
    sum: i64,
}

impl Anchor {
    fn new(pos: i32, val: i64, sum: i64) -> Self {
        Self { pos, val, sum }
    }
}

#[derive(Clone, Debug)]
struct NodeData {
    result: i32,
    len: i32,
    l_anchors: Vec<Anchor>,
    r_anchors: Vec<Anchor>,
}

impl NodeData {
    fn empty() -> Self {
        Self {
            result: 0,
            len: 0,
            l_anchors: vec![Anchor::new(-1, INF, 0)],
            r_anchors: vec![Anchor::new(0, INF, 0)],
        }
    }

    fn new(a: i64) -> Self {
        Self {
            result: 0,
            len: 1,
            l_anchors: vec![Anchor::new(0, a, 0), Anchor::new(-1, INF, a)],
            r_anchors: vec![Anchor::new(0, a, 0), Anchor::new(1, INF, a)],
        }
    }
}

const INF: i64 = 1000000000000000000;

struct MySpec {}
impl LazySegTreeSpec<NodeData, ()> for &MySpec {
    fn identity(&self) -> NodeData {
        NodeData::empty()
    }

    fn op(&self, a: &NodeData, b: &NodeData) -> NodeData {
        let mut result = a.result.max(b.result);
        let mut i = 0;
        let mut j = 0;
        let mut mx = 0;
        while a.l_anchors[i].val < INF || b.r_anchors[j].val < INF {
            let sum = a.l_anchors[i].sum + b.r_anchors[j].sum;
            let len = (a.len - a.l_anchors[i].pos - 1) + b.r_anchors[j].pos;
            if 2 * mx < sum {
                result = result.max(len);
            }
            if a.l_anchors[i].val <= b.r_anchors[j].val {
                mx = a.l_anchors[i].val;
                i += 1;
            } else {
                mx = b.r_anchors[j].val;
                j += 1;
            }
        }
        let sum = a.l_anchors[i].sum + b.r_anchors[j].sum;
        let len = (a.len - a.l_anchors[i].pos - 1) + b.r_anchors[j].pos;
        if 2 * mx < sum {
            result = result.max(len);
        }

        let a_sum = a.r_anchors.last().unwrap().sum;
        let b_sum = b.l_anchors.last().unwrap().sum;
        let mut l_anchors = vec![];
        let mut r_anchors = vec![];
        for bl in &b.l_anchors {
            if bl.val != INF {
                l_anchors.push(Anchor::new(a.len + bl.pos, bl.val, bl.sum));
            }
        }
        for al in &a.l_anchors {
            if al.val >= al.sum + b_sum {
                l_anchors.push(Anchor::new(al.pos, al.val, al.sum + b_sum));
            }
        }
        for ar in &a.r_anchors {
            if ar.val != INF {
                r_anchors.push(Anchor::new(ar.pos, ar.val, ar.sum));
            }
        }
        for br in &b.r_anchors {
            if br.val >= a_sum + br.sum {
                r_anchors.push(Anchor::new(a.len + br.pos, br.val, a_sum + br.sum));
            }
        }
        NodeData {
            result,
            len: a.len + b.len,
            l_anchors,
            r_anchors,
        }
    }

    fn compose(&self, _old: &(), _new: &()) -> () {}

    fn update(&self, t: &NodeData, _u: &()) -> NodeData {
        t.clone()
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_long_vec(n);
    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, n);
    tree.init(|i| NodeData::new(a[i]));
    for _ in 0..q {
        match input.read() {
            1 => {
                let l = input.read_size() - 1;
                let r = input.read_size();
                let p = tree.product(l, r);
                if p.result < 3 {
                    out.print_line(-1);
                } else {
                    out.print_line(p.result);
                }
            }
            2 => {
                let i = input.read_size() - 1;
                let x = input.read_long();
                tree.set(i, NodeData::new(x));
            }
            _ => panic!("unexpected query type"),
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
