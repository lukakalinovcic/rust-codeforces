//{"name":"H. Ksyusha and the Loaded Set","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5\n1 2 5 905 2000000\n15\n- 2\n? 2\n? 1\n- 1\n? 1\n+ 4\n+ 2\n? 2\n+ 6\n- 4\n+ 7\n? 2\n? 3\n? 4\n? 2000000\n5\n3 4 5 6 8\n9\n? 5\n- 5\n? 5\n+ 1\n? 2\n- 6\n- 8\n+ 6\n? 5\n5\n6 7 8 9 10\n10\n? 5\n- 6\n? 4\n- 10\n+ 5\n- 8\n+ 3\n+ 2\n- 3\n+ 10\n","output":"2 2 1 6 3 8 8 2000001\n9 9 9 7\n1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HKsyushaAndTheLoadedSet"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::segtree::lazy_segtree::LazySegTree;
use algo_lib::segtree::lazy_segtree::LazySegTreeSpec;

type PreCalc = ();

const INF: i32 = 1000000000;

#[derive(Debug)]
struct Gap {
    position: i32,
    size: i32,
    t_start: i32,
    t_end: i32,
}

struct GapTracker {
    m: BTreeMap<i32, (i32, i32)>,
    gaps: Vec<Gap>,
}

impl GapTracker {
    fn new(a: Vec<i32>) -> Self {
        let mut m = BTreeMap::new();
        let mut prev = 0;
        for x in a {
            m.insert(prev, (x - prev - 1, 0));
            prev = x;
        }
        m.insert(prev, (INF, 0));
        m.insert(INF, (INF, 0));
        Self { m, gaps: vec![] }
    }

    fn add(&mut self, x: i32, t: i32) {
        let (prev_x, (gap_size, t_start)) = self.m.range_mut(..x).next_back().unwrap();
        if *gap_size > 0 {
            self.gaps.push(Gap {
                position: *prev_x,
                size: *gap_size,
                t_start: *t_start,
                t_end: t,
            });
        }
        *gap_size = x - prev_x - 1;
        *t_start = t;

        self.m
            .insert(x, (self.m.range(x..).next().unwrap().0 - x - 1, t));
    }

    fn remove(&mut self, x: i32, t: i32) {
        let mut it = self.m.range_mut(..=x);
        let (_, (gap_size1, t_start1)) = it.next_back().unwrap();
        if *gap_size1 > 0 {
            self.gaps.push(Gap {
                position: x,
                size: *gap_size1,
                t_start: *t_start1,
                t_end: t,
            });
        }
        let (prev_x, (gap_size2, t_start2)) = it.next_back().unwrap();
        if *gap_size2 > 0 {
            self.gaps.push(Gap {
                position: *prev_x,
                size: *gap_size2,
                t_start: *t_start2,
                t_end: t,
            });
        }
        *gap_size2 += *gap_size1 + 1;
        *t_start2 = t;
        self.m.remove(&x);
    }

    fn finish(mut self, t: i32) -> Vec<Gap> {
        for (x, (gap_size, t_start)) in self.m.into_iter() {
            if gap_size > 0 {
                self.gaps.push(Gap {
                    position: x,
                    size: gap_size,
                    t_start: t_start,
                    t_end: t,
                });
            }
        }
        self.gaps
    }
}

#[derive(Debug)]
struct Query {
    k: i32,
    t: i32,
}

struct MySpec {}
impl LazySegTreeSpec<i32, i32> for &MySpec {
    fn identity(&self) -> i32 {
        INF
    }

    fn op(&self, a: &i32, b: &i32) -> i32 {
        (*a).min(*b)
    }

    fn compose(&self, old: &i32, new: &i32) -> i32 {
        (*new).min(*old)
    }

    fn update(&self, t: &i32, u: &i32) -> i32 {
        (*t).min(*u)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut gap_tracker = GapTracker::new(input.read_int_vec(n));
    let m = input.read();
    let mut queries = vec![];
    for t in 1..=m {
        let op = input.read_char();
        match op {
            '+' => gap_tracker.add(input.read(), t),
            '-' => gap_tracker.remove(input.read(), t),
            '?' => queries.push(Query { k: input.read(), t }),
            _ => panic!("Unexpected op"),
        }
    }
    let mut gaps = gap_tracker.finish(m + 1);
    gaps.sort_by_key(|g| -g.size);
    queries.sort_by_key(|q| -q.k);
    let mut next_gap = 0;
    let mut results = vec![];

    let tree_spec = MySpec {};
    let mut tree = LazySegTree::new(&tree_spec, m as usize + 2);
    for query in queries {
        while next_gap < gaps.len() && gaps[next_gap].size >= query.k {
            tree.update(
                gaps[next_gap].t_start as usize,
                gaps[next_gap].t_end as usize,
                gaps[next_gap].position,
            );
            next_gap += 1;
        }
        let result = tree.product(query.t as usize, query.t as usize + 1) + 1;
        results.push((query.t, result));
    }
    results.sort();
    for (_, result) in results {
        out.print_line(result);
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
