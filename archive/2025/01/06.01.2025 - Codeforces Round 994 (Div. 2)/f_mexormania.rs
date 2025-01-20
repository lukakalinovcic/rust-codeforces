//{"name":"F. MEX OR Mania","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n6 3\n0 0 1 0 1 0\n6 1\n3 2\n6 3\n3 1\n1 3 1\n1 1\n","output":"6\n3\n2\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMEXORMania"}}}

use std::collections::BTreeSet;
use std::mem::swap;
use std::ops::Range;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

enum CandidateState {
    Dead,
    Alive,
    Active,
}

struct Candidate {
    lo: i32,
    hi: i32,
    cnt: Vec<i32>,
    unique_cnt: i32,
    state: CandidateState,
}

impl Candidate {
    fn new(k: usize, lo: usize, hi: usize) -> Self {
        Self {
            lo: lo as i32,
            hi: hi as i32,
            cnt: vec![0; k],
            unique_cnt: 0,
            state: CandidateState::Alive,
        }
    }

    fn insert(&mut self, x: usize) {
        self.cnt[x] += 1;
        if self.cnt[x] == 1 {
            self.unique_cnt += 1;
            if self.unique_cnt == self.cnt.len() as i32 {
                self.state = CandidateState::Active;
            }
        }
    }

    fn erase(&mut self, x: usize) {
        self.cnt[x] -= 1;
        if self.cnt[x] == 0 {
            self.unique_cnt -= 1;
            self.state = CandidateState::Alive;
        }
    }
}

struct CandidateSet {
    k: usize,
    cands: Vec<Candidate>,
    cand_index: Vec<i32>,
    active_cands: BTreeSet<(i32, i32)>,
}

impl CandidateSet {
    fn new(n: usize, k: usize) -> Self {
        Self {
            k,
            cands: vec![],
            cand_index: vec![-1; n],
            active_cands: BTreeSet::new(),
        }
    }

    fn new_cand(&mut self, a: &[i32], lo: usize, hi: usize) {
        let mut cand = Candidate::new(self.k, lo, hi);
        let i = self.cands.len();
        for p in lo..hi {
            self.cand_index[p] = i as i32;
            cand.insert(a[p] as usize);
        }
        if let CandidateState::Active = cand.state {
            self.active_cands.insert((cand.hi - cand.lo, i as i32));
        }
        self.cands.push(cand);
    }

    fn query(&mut self, a: &[i32], p: usize, delta: i32) {
        let i = self.cand_index[p];
        if i == -1 {
            return;
        }
        let cand = &mut self.cands[i as usize];
        if let CandidateState::Dead = cand.state {
            return;
        }
        if let CandidateState::Active = cand.state {
            self.active_cands.remove(&(cand.hi - cand.lo, i as i32));
        }
        let old_value = a[p];
        cand.erase(old_value as usize);
        let new_value = a[p] + delta;
        if new_value < self.k as i32 {
            cand.insert(new_value as usize);
            if let CandidateState::Active = cand.state {
                self.active_cands.insert((cand.hi - cand.lo, i as i32));
            }
        } else {
            self.cand_index[p] = -1;
            let mut r1 = Range {
                start: cand.lo as usize,
                end: p,
            };
            let mut r2 = Range {
                start: p + 1,
                end: cand.hi as usize,
            };
            if r1.len() < r2.len() {
                swap(&mut r1, &mut r2);
            }
            for j in r2.clone() {
                cand.erase(a[j] as usize);
                self.cand_index[j] = -1;
            }
            cand.lo = r1.start as i32;
            cand.hi = r1.end as i32;
            if r1.len() < self.k {
                cand.state = CandidateState::Dead;
                for j in r1 {
                    self.cand_index[j] = -1;
                }
            }
            if let CandidateState::Active = cand.state {
                self.active_cands.insert((cand.hi - cand.lo, i as i32));
            }
            if r2.len() >= self.k {
                self.new_cand(a, r2.start, r2.end);
            }
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let q: usize = input.read();
    let mut a = input.read_int_vec(n);
    let mut cand_sets = vec![];
    let mut k = 1;
    while k <= n {
        let mut cand_set = CandidateSet::new(n, k);

        let mut lo = 0;
        while lo < a.len() {
            let mut hi = lo;
            while hi < a.len() && a[hi] < k as i32 {
                hi += 1;
            }
            if lo == hi {
                hi += 1;
            } else if hi - lo >= k {
                cand_set.new_cand(&a, lo, hi);
            }
            lo = hi;
        }

        cand_sets.push(cand_set);
        k *= 2;
    }
    for _ in 0..q {
        let p = input.read_int() as usize - 1;
        let delta = input.read_int();
        for cs in cand_sets.iter_mut() {
            cs.query(&a, p, delta);
        }
        a[p] += delta;

        let mut result = 0;
        for cs in cand_sets.iter() {
            if let Some((largest_active_cand_size, _)) = cs.active_cands.last() {
                result = result.max(*largest_active_cand_size);
            }
        }
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
