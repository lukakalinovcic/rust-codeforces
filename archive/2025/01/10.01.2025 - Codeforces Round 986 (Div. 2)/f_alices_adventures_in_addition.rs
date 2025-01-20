//{"name":"F. Alice's Adventures in Addition","group":"Codeforces - Codeforces Round 986 (Div. 2)","url":"https://codeforces.com/contest/2028/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n5 4\n2 1 1 1 2\n5 5\n2 1 1 1 2\n5 6\n2 1 1 1 2\n5 7\n2 1 1 1 2\n5 8\n2 1 1 1 2\n5 6\n2 0 2 2 3\n","output":"YES\nYES\nYES\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FAlicesAdventuresInAddition"}}}

use std::ops::Shr;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MAX: usize = 10000;
const MAX_DP_HISTORY: usize = 32;

#[derive(Clone)]
struct BitVector {
    m: i32,
    a: [u64; MAX / 64 + 1],
}

impl BitVector {
    fn new(m: i32) -> Self {
        Self {
            m,
            a: [0; MAX / 64 + 1],
        }
    }

    fn set(&mut self, i: i32) {
        let (pos, bit) = ((i / 64) as usize, (i % 64) as usize);
        self.a[pos] |= 1 << bit;
    }

    fn get(&self, i: i32) -> bool {
        let (pos, bit) = ((i / 64) as usize, (i % 64) as usize);
        ((self.a[pos] >> bit) & 1) == 1
    }

    fn shifted_or(&mut self, other: &BitVector, shift: i32) {
        let (pos_delta, carry_bits) = ((shift / 64) as usize, (shift % 64) as u32);
        let keep_mask = u64::MAX >> carry_bits;
        let shift_end = (self.m / 64) as usize + 1;
        let mut prev_carry = 0;
        for pos in pos_delta..shift_end {
            let curr = other.a[pos - pos_delta];
            let (curr_keep, curr_carry) = (
                curr & keep_mask,
                curr.checked_shr(64 - carry_bits).unwrap_or_default(),
            );
            self.a[pos] |= (curr_keep << carry_bits) | prev_carry;
            prev_carry = curr_carry;
        }
    }

    fn get_all(&self) -> Vec<bool> {
        (0..=self.m).map(|i| self.get(i)).collect()
    }
}

#[derive(Debug)]
enum Entry {
    Zero,
    RunOfOnes(i32),
    TwoPlus(i32),
}

fn doit(entries: Vec<Entry>, m: i32) -> Vec<bool> {
    if let [Entry::RunOfOnes(x)] = entries.as_slice() {
        let mut result = BitVector::new(m);
        for i in 1..=*x {
            if i <= m {
                result.set(i);
            }
        }
        return result.get_all();
    }
    let n = entries.len();
    let mut dp = vec![BitVector::new(m); MAX_DP_HISTORY];
    let mut prev_zero = BitVector::new(m);
    let mut next_zero = BitVector::new(m);
    dp[0].set(0);
    for (i, entry) in entries.iter().enumerate() {
        let mut curr = prev_zero.clone();
        match entry {
            Entry::Zero => {
                curr.set(0);
                curr.shifted_or(&dp[i % MAX_DP_HISTORY], 0);
                curr.shifted_or(&next_zero, 0);
            }
            Entry::RunOfOnes(k) => {
                for shift in 0..=*k {
                    curr.shifted_or(&dp[i % MAX_DP_HISTORY], shift);
                }
            }
            Entry::TwoPlus(a) => {
                let mut shift = *a;
                if shift <= m {
                    curr.shifted_or(&dp[i % MAX_DP_HISTORY], shift);
                }
                for j in (0..i).rev() {
                    shift = match entries[j] {
                        Entry::Zero => break,
                        Entry::RunOfOnes(_) => shift,
                        Entry::TwoPlus(a) => shift * a,
                    };
                    if shift > m {
                        break;
                    }
                    curr.shifted_or(&dp[j % MAX_DP_HISTORY], shift);
                }
            }
        }
        next_zero.shifted_or(&curr, 0);
        dp[(i + 1) % MAX_DP_HISTORY] = curr;
        if let Entry::Zero = entry {
            prev_zero = next_zero.clone();
        }
    }
    dp[n % MAX_DP_HISTORY].get_all()
}

fn process_and_doit(m: i32, a: Vec<i32>) -> Vec<bool> {
    let mut entries = vec![];
    let mut curr_run = None;
    for a in a {
        if a == 1 {
            curr_run = Some(curr_run.unwrap_or(0) + 1);
        } else {
            if let Some(k) = curr_run {
                entries.push(Entry::RunOfOnes(k));
                curr_run = None;
            }
            if a == 0 {
                entries.push(Entry::Zero);
            } else {
                entries.push(Entry::TwoPlus(a));
            }
        }
    }
    if let Some(k) = curr_run {
        entries.push(Entry::RunOfOnes(k));
    }
    doit(entries, m)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read_int();
    let a = input.read_int_vec(n);
    let result = process_and_doit(m, a);
    if result[m as usize] {
        out.print_line("YES");
    } else {
        out.print_line("NO");
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

fn brute(m: i32, a: Vec<i32>) -> Vec<bool> {
    let n = a.len();
    let mut dp = vec![vec![false; m as usize + 1]; n + 1];
    dp[n][0] = true;
    for i in (0..n).rev() {
        for s in 0..=m {
            let mut p = 1;
            for j in i..n {
                p = p * a[j];
                if p > m {
                    p = m + 1;
                }
                if s - p >= 0 {
                    dp[i][s as usize] |= dp[j + 1][(s - p) as usize];
                }
            }
        }
    }
    dp[0].clone()
}

fn main() {
    tester::run_tests();
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 10 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(1..200);
        let a: Vec<i32> = (0..n).map(|_| rng.gen_range(0..=100)).collect();
        let m = 100;
        let s1 = process_and_doit(m, a.clone());
        let s2 = brute(m, a.clone());
        if s1 != s2 {
            eprintln!("{n} {a:?}");
            eprintln!("{s1:?}");
            eprintln!("{s2:?}");
            break;
        }
    }
}
//END MAIN
