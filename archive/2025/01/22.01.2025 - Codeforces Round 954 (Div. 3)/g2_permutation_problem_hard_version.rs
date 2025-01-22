//{"name":"G2. Permutation Problem (Hard Version)","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/G2","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1\n1\n2\n1 2\n3\n2 3 1\n5\n2 4 1 3 5\n12\n8 9 7 12 1 10 6 3 2 4 11 5\n15\n1 2 4 6 8 10 12 14 3 9 15 5 7 11 13\n","output":"0\n1\n1\n3\n9\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G2PermutationProblemHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX: usize = 500000;

fn gen_divisors(f: &[(i32, i32)], i: usize, mut divisor: i32, out: &mut Vec<i32>) {
    if i == f.len() {
        out.push(divisor);
    } else {
        gen_divisors(f, i + 1, divisor, out);
        for _ in 1..=f[i].1 {
            divisor *= f[i].0;
            gen_divisors(f, i + 1, divisor, out);
        }
    }
}

struct PreCalc {
    divisors: Vec<i32>,
    divisor_start: Vec<i32>,
}

impl PreCalc {
    fn new(n: usize) -> Self {
        let mut min_factor = vec![(0, 0); n + 1];
        for i in 2..=n {
            if min_factor[i].0 != 0 {
                continue;
            }
            min_factor[i] = (i as i32, 1);
            let mut j = i * i;
            let mut k = i;
            while j <= n {
                if min_factor[j].0 == 0 {
                    min_factor[j] = (i as i32, k as i32);
                }
                k += 1;
                j += i;
            }
        }
        let mut buffer = vec![];
        let mut divisors = vec![0; 6639000];
        let mut divisor_start = vec![0; n + 2];
        let mut buffer_index = 0;
        for i in 1..=n {
            let mut t = i as i32;
            let mut factors = vec![];
            while t != 1 {
                let prime = min_factor[t as usize].0;
                let mut power = 0;
                while prime == min_factor[t as usize].0 {
                    t = min_factor[t as usize].1;
                    power += 1;
                }
                factors.push((prime, power));
            }
            gen_divisors(&factors, 0, 1, &mut buffer);
            buffer.sort();
            divisor_start[i] = buffer_index as i32;
            for i in 0..buffer.len() {
                divisors[buffer_index + i] = buffer[i];
            }
            buffer_index += buffer.len();
            buffer.clear();
        }
        divisor_start[n + 1] = buffer_index as i32;

        Self {
            divisors,
            divisor_start,
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);
    let mut by_a = vec![(0, 0, 0); n];
    let mut by_b = vec![(0, 0, 0); n];
    let mut by_a_start = vec![0; n + 2];
    let mut by_b_start = vec![0; n + 2];
    for (i, p) in p.into_iter().enumerate() {
        let a = p;
        let b = (i + 1) as i32;
        let g = gcd(a, b);
        let a = a / g;
        let b = b / g;
        by_a[i] = (a, i as i32, b);
        by_b[i] = (b, i as i32, a);
    }
    by_a.sort();
    by_b.sort();
    {
        let mut j = 0;
        for i in 1..=n + 1 {
            while j < by_a.len() && by_a[j].0 < i as i32 {
                j += 1;
            }
            by_a_start[i] = j;
        }
    }
    {
        let mut j = 0;
        for i in 1..=n + 1 {
            while j < by_b.len() && by_b[j].0 < i as i32 {
                j += 1;
            }
            by_b_start[i] = j;
        }
    }

    const CHUNK_SIZE: usize = 35;
    let mut active_buffer = vec![0; n * CHUNK_SIZE];
    let mut cnt_divs = vec![0; n + 1];
    let mut active_range = vec![(0, 0); n + 1];
    let mut non_empty_divisors = vec![0; n + 1];

    let mut result = 0;
    for b2 in 1..=n {
        for chunk_start in (0..).step_by(CHUNK_SIZE) {
            let mut buffer_start = 0;
            let mut is_done = true;
            let mut n_non_empty_divisors = 0;
            for (_, _, a2) in &by_b[by_b_start[b2]..by_b_start[b2 + 1]] {
                let a2 = *a2;
                let divisors_lo = precalc.divisor_start[a2 as usize] as usize;
                let divisors_hi = precalc.divisor_start[a2 as usize + 1] as usize;
                let divs = &precalc.divisors[divisors_lo..divisors_hi];
                if chunk_start >= divs.len() {
                    continue;
                }
                let divs_chunk = &divs[chunk_start..(chunk_start + CHUNK_SIZE).min(divs.len())];
                if !divs_chunk.is_empty() {
                    is_done = false;
                }
                for d in divs_chunk {
                    let d = *d;
                    if cnt_divs[d as usize] == 0 {
                        non_empty_divisors[n_non_empty_divisors] = d;
                        n_non_empty_divisors += 1;
                    }
                    cnt_divs[d as usize] += 1;
                }
            }
            if is_done {
                break;
            }
            for d in &non_empty_divisors[0..n_non_empty_divisors] {
                let d = *d;
                active_range[d as usize] = (
                    buffer_start as i32,
                    buffer_start as i32 + cnt_divs[d as usize],
                );
                buffer_start += cnt_divs[d as usize];
                cnt_divs[d as usize] = 0;
            }
            for (_, i2, a2) in &by_b[by_b_start[b2]..by_b_start[b2 + 1]] {
                let a2 = *a2;
                let i2 = *i2;
                let divisors_lo = precalc.divisor_start[a2 as usize] as usize;
                let divisors_hi = precalc.divisor_start[a2 as usize + 1] as usize;
                let divs = &precalc.divisors[divisors_lo..divisors_hi];
                if chunk_start >= divs.len() {
                    continue;
                }
                let divs_chunk = &divs[chunk_start..(chunk_start + CHUNK_SIZE).min(divs.len())];
                for d in divs_chunk {
                    let d = *d;
                    let p = active_range[d as usize].0 as usize + cnt_divs[d as usize] as usize;
                    active_buffer[p] = i2 as i32;
                    cnt_divs[d as usize] += 1;
                }
            }

            for a1 in (b2..=n).step_by(b2) {
                for (_, i1, b1) in &by_a[by_a_start[a1]..by_a_start[a1 + 1]] {
                    let b1 = *b1;
                    let i1 = *i1;
                    if cnt_divs[b1 as usize] > 0 {
                        let lo = active_range[b1 as usize].0 as usize;
                        let hi = active_range[b1 as usize].1 as usize;
                        let active_slice = &active_buffer[lo..hi];
                        let (Ok(j) | Err(j)) = active_slice.binary_search(&(i1 + 1));
                        let add = (active_slice.len() - j) as i64;
                        result += add;
                    }
                }
            }
            for d in &non_empty_divisors[0..n_non_empty_divisors] {
                let d = *d;
                cnt_divs[d as usize] = 0;
            }
        }
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(MAX);

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
