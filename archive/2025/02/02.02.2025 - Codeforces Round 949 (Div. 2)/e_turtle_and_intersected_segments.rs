//{"name":"E. Turtle and Intersected Segments","group":"Codeforces - Codeforces Round 949 (Div. 2)","url":"https://codeforces.com/contest/1981/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5\n1 7 3\n2 4 6\n3 5 5\n6 7 9\n3 4 4\n5\n2 7 3\n1 3 6\n4 5 5\n6 7 9\n1 1 4\n4\n1 4 3\n1 2 1\n3 4 5\n1 4 4\n3\n1 3 1\n2 3 3\n4 5 8\n","output":"9\n13\n4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETurtleAndIntersectedSegments"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n as i32).collect::<Vec<_>>(),
        }
    }

    fn find(&mut self, u: i32) -> i32 {
        if self.p[u as usize] != u {
            self.p[u as usize] = self.find(self.p[u as usize]);
        }
        self.p[u as usize]
    }

    fn union(&mut self, u: i32, v: i32) -> bool {
        let u = self.find(u);
        let v = self.find(v);
        if u == v {
            false
        } else {
            self.p[v as usize] = u;
            true
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut events = vec![];
    for i in 0..n {
        let l = input.read_int();
        let r = input.read_int();
        let a = input.read_int();
        events.push((l, 0, a, i));
        events.push((r, 1, a, i));
    }
    events.sort();

    let mut edges = vec![];
    let mut active: BTreeSet<(i32, i32)> = BTreeSet::new();

    let mut maybe_add_edge = |a: i32, i: i32, active: &BTreeSet<(i32, i32)>| {
        if let Some(prev) = active.range(..(a, i)).rev().next() {
            if let Some(next) = active.range((a, i)..).next() {
                edges.push((next.0 - prev.0, prev.1, next.1));
            }
        }
    };

    for (_x, t, a, i) in events {
        if t == 0 {
            maybe_add_edge(a, i, &active);
            active.insert((a, i));
            maybe_add_edge(a, i, &active);
            maybe_add_edge(a, i + 1, &active);
        } else {
            maybe_add_edge(a, i, &active);
            maybe_add_edge(a, i + 1, &active);
            active.remove(&(a, i));
            maybe_add_edge(a, i, &active);
        }
    }

    let mut mst_cost: i64 = 0;
    let mut components = n;
    let mut dsu = DSU::new(n as usize);
    edges.sort();
    for (cost, u, v) in edges {
        if dsu.union(u, v) {
            mst_cost += cost as i64;
            components -= 1;
        }
    }
    if components != 1 {
        out.print_line(-1);
    } else {
        out.print_line(mst_cost);
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
