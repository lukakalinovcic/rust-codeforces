//{"name":"F. Bermart Ice Cream","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"12\n2 1 5 7\n2 1 3 4\n4 1 4\n4 1 8\n4 1 2\n1 1\n2 2 4 10\n4 1 9\n4 2 9\n3 1\n4 1 9\n4 2 9\n","output":"4\n11\n0\n11\n17\n4\n17\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBermartIceCream"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MAXP: usize = 2000;

type DpStack = Vec<(i32, i32, [i32; MAXP + 1])>;

fn push_stack(s: &mut DpStack, p: i32, t: i32) {
    let mut dp = if let Some((_, _, prev_dp)) = s.last() {
        prev_dp.clone()
    } else {
        [0; MAXP + 1]
    };

    let offset = p as usize;
    for i in (offset..=MAXP).rev() {
        dp[i] = dp[i].max(t + dp[i - offset])
    }
    s.push((p, t, dp));
}

fn drain_half_stack(source: &mut DpStack, sink: &mut DpStack) {
    let items: Vec<_> = source.drain(..).map(|(p, t, _)| (p, t)).collect();
    let mid = (items.len() + 1) / 2;
    for item in items[0..mid].iter().rev() {
        let (p, t) = *item;
        push_stack(sink, p, t);
    }
    for item in items[mid..].iter() {
        let (p, t) = *item;
        push_stack(source, p, t);
    }
}

struct DpDeque {
    front: DpStack,
    back: DpStack,
}

impl DpDeque {
    fn new() -> Self {
        DpDeque {
            front: vec![],
            back: vec![],
        }
    }

    fn query(&self, p: i32) -> i32 {
        let mut result = 0;
        for i in 0..=p {
            let mut a = 0;
            let mut b = 0;
            if let Some((_, _, dp)) = self.front.last() {
                a = dp[i as usize];
            }
            if let Some((_, _, dp)) = self.back.last() {
                b = dp[(p - i) as usize];
            }
            result = result.max(a + b);
        }
        result
    }

    fn push_back(&mut self, p: i32, t: i32) {
        push_stack(&mut self.back, p, t);
    }

    fn push_front(&mut self, p: i32, t: i32) {
        push_stack(&mut self.front, p, t);
    }

    fn pop_back(&mut self) -> (i32, i32) {
        if self.back.is_empty() {
            drain_half_stack(&mut self.front, &mut self.back);
        }
        let (p, t, _) = self.back.pop().unwrap();
        (p, t)
    }

    fn pop_front(&mut self) -> (i32, i32) {
        if self.front.is_empty() {
            drain_half_stack(&mut self.back, &mut self.front);
        }
        let (p, t, _) = self.front.pop().unwrap();
        (p, t)
    }
}

#[derive(Clone, Debug)]
enum Op {
    Clone(usize),
    PushBack(usize, i32, i32),
    PopFront(usize),
    Query(i32, usize),
}

fn traverse(ops: &Vec<Vec<Op>>, i: usize, dp_deque: &mut DpDeque, results: &mut Vec<i32>) {
    for op in &ops[i] {
        match *op {
            Op::Clone(j) => traverse(ops, j, dp_deque, results),
            Op::PushBack(j, p, t) => {
                dp_deque.push_back(p, t);
                traverse(ops, j, dp_deque, results);
                dp_deque.pop_back();
            }
            Op::PopFront(j) => {
                let (p, t) = dp_deque.pop_front();
                traverse(ops, j, dp_deque, results);
                dp_deque.push_front(p, t);
            }
            Op::Query(p, query_index) => {
                results[query_index] = dp_deque.query(p);
            }
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let q = input.read();
    let mut store_version = vec![0];
    let mut version_ops = vec![vec![]];
    let mut result = vec![];
    for _ in 0..q {
        let t = input.read_int();
        let store = (input.read_int() - 1) as usize;
        let version = store_version[store];
        let new_version = if t != 4 {
            let new_version = version_ops.len();
            version_ops.push(vec![]);
            new_version
        } else {
            version
        };
        match t {
            1 => {
                version_ops[version].push(Op::Clone(new_version));
                store_version.push(new_version);
            }
            2 => {
                let p = input.read_int();
                let t = input.read_int();
                version_ops[version].push(Op::PushBack(new_version, p, t));
                store_version[store] = new_version;
            }
            3 => {
                version_ops[version].push(Op::PopFront(new_version));
                store_version[store] = new_version;
            }
            4 => {
                let p = input.read_int();
                version_ops[version].push(Op::Query(p, result.len()));
                result.push(0);
            }
            _ => panic!("unexpected op"),
        }
    }
    let mut dp_deque = DpDeque::new();
    traverse(&version_ops, 0, &mut dp_deque, &mut result);
    out.print_iter(result.into_iter());
    out.print("\n");
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
