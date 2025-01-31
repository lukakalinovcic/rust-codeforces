//{"name":"F. Max Plus Min Plus Size","group":"Codeforces - Codeforces Round 975 (Div. 2)","url":"https://codeforces.com/contest/2019/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n5 4 5\n3\n4 5 4\n10\n3 3 3 3 4 1 2 3 5 4\n10\n17 89 92 42 29 41 92 14 70 45\n","output":"12\n11\n12\n186\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaxPlusMinPlusSize"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Debug, Clone)]
struct Run {
    pos: i32,
    len: i32,
    max_at_even: bool,
    max_at_odd: bool,
}

#[derive(Default, Debug)]
struct DS {
    max: Option<i32>,
    min: Option<i32>,
    sum_size: i32,
    max_covers: i32,
    runs: BTreeMap<i32, Run>,
}

impl DS {
    fn new() -> Self {
        DS::default()
    }

    fn activate(&mut self, i: i32, x: i32) {
        let max = self.max.unwrap_or(x);
        self.max = Some(max);
        self.min = Some(x);

        let mut run = Run {
            pos: i,
            len: 1,
            max_at_even: x == max,
            max_at_odd: false,
        };
        self.tally_run(&run, 1);
        if let Some((_, next)) = self.runs.range(i + 1..).next() {
            if next.pos == i + 1 {
                let next = next.clone();
                self.runs.remove(&next.pos);
                run = self.merge(run, next);
            }
        }

        if let Some((_, prev)) = self.runs.range(..i).rev().next() {
            if prev.pos + prev.len == i {
                let prev = prev.clone();
                self.runs.remove(&prev.pos);
                run = self.merge(prev, run);
            }
        }

        self.runs.insert(run.pos, run);
    }

    fn tally_run(&mut self, run: &Run, sign: i32) {
        self.sum_size += sign * (run.len + 1) / 2;
        let is_covering_max = run.max_at_even || (run.max_at_odd && run.len % 2 == 0);
        if is_covering_max {
            self.max_covers += sign;
        }
    }

    fn merge(&mut self, a: Run, b: Run) -> Run {
        self.tally_run(&a, -1);
        self.tally_run(&b, -1);
        let c = Run {
            pos: a.pos,
            len: a.len + b.len,
            max_at_even: a.max_at_even
                | if a.len % 2 == 0 {
                    b.max_at_even
                } else {
                    b.max_at_odd
                },
            max_at_odd: a.max_at_odd
                | if a.len % 2 == 0 {
                    b.max_at_odd
                } else {
                    b.max_at_even
                },
        };
        self.tally_run(&c, 1);
        c
    }

    fn get_value(&self) -> i32 {
        let max = self.max.unwrap_or_default();
        let min = self.min.unwrap_or_default();
        return max + min + self.sum_size - if self.max_covers == 0 { 1 } else { 0 };
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input
        .read_int_vec(n)
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<_>>();
    a.sort();
    a.reverse();
    let mut ds = DS::new();
    let mut result = 0;
    for (x, i) in a {
        ds.activate(i as i32, x);
        result = result.max(ds.get_value());
    }
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
