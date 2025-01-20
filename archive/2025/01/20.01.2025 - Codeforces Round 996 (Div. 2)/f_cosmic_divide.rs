//{"name":"F. Cosmic Divide","group":"Codeforces - Codeforces Round 996 (Div. 2)","url":"https://codeforces.com/contest/2055/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n2\n1 2\n2 3\n4\n4 4\n2 4\n1 4\n1 2\n3\n1 2\n1 2\n2 3\n2\n1 3\n3 3\n2\n1 3\n2 2\n3\n1 2\n1 3\n1 3\n4\n8 9\n6 8\n6 8\n5 6\n","output":"YES\nYES\nNO\nNO\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCosmicDivide"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

fn small_rand(prev: u16) -> u16 {
    let x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let h1 = x >> 16;
    let h2 = x & 0xffff;
    let curr = (h1 ^ h2) as u16;
    (prev << 8) ^ (prev >> 8) ^ curr ^ 0xcd15
}

fn is_prime(p: i32) -> bool {
    for d in 2.. {
        if p % d == 0 {
            return false;
        }
        if d * d > p {
            break;
        }
    }
    true
}

struct PreCalc {
    hasher_params: Vec<(i32, i32)>,
}

impl PreCalc {
    fn new(prime_lo: i32, n: usize) -> Self {
        let mut params = vec![];
        let mut r = small_rand(12345);
        let mut p = prime_lo + (r as i32) % 31337;
        while params.len() < n {
            if is_prime(p) {
                let b = p - 1337 - r as i32;
                params.push((b, p));
                r = small_rand(r);
                p += (r as i32) % 31337;
            }
            p += 1;
        }
        Self {
            hasher_params: params,
        }
    }
}

fn mult(a: i32, b: i32, p: i32) -> i32 {
    (((a as i64) * (b as i64)) % (p as i64)) as i32
}

struct RollingHash {
    p: i32,
    b_pow_i: Vec<i32>,
    prefix_sum: Vec<i32>,
}

impl RollingHash {
    fn new(a: &[i32], b: i32, p: i32) -> Self {
        let n = a.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut b_pow_i = vec![0; n + 1];
        b_pow_i[0] = 1;
        for i in 0..n {
            let a = ((a[i] % p) + p) % p;
            prefix_sum[i + 1] = (mult(prefix_sum[i], b, p) + a) % p;
            b_pow_i[i + 1] = mult(b_pow_i[i], b, p);
        }
        Self {
            p,
            b_pow_i,
            prefix_sum,
        }
    }

    // 1-indexed.
    fn query(&self, lo: usize, hi: usize) -> i32 {
        if lo > hi {
            return 0;
        }
        let hi_sum = self.prefix_sum[hi];
        let lo_sum = mult(self.prefix_sum[lo - 1], self.b_pow_i[hi - lo + 1], self.p);
        (hi_sum + self.p - lo_sum) % self.p
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let mut l = vec![0; n];
    let mut r = vec![0; n];
    for i in 0..n {
        l[i] = input.read_int();
        r[i] = input.read_int() + 1;
    }
    if doit(&l, &r, &precalc) {
        out.print_line("YES");
    } else {
        out.print_line("NO");
    }
}

fn doit(l: &[i32], r: &[i32], precalc: &PreCalc) -> bool {
    let n = l.len();
    let mut l = l.to_vec();
    let mut r = r.to_vec();
    if check_candidate_shapes(horizontal_split(n, &l, &r)) {
        return true;
    }
    if check_candidate_shapes(vertical_split(n, &l, &r)) {
        return true;
    }
    for _ in 0..2 {
        let mut l_diff = vec![0; n - 1];
        let mut r_diff = vec![0; n - 1];
        for i in 0..n - 1 {
            l_diff[i] = l[i + 1] - l[i];
            r_diff[i] = r[i + 1] - r[i];
        }
        let l_diff_hashers = precalc
            .hasher_params
            .iter()
            .map(|(base, prime)| RollingHash::new(&l_diff, *base, *prime))
            .collect::<Vec<_>>();
        let r_diff_hashers = precalc
            .hasher_params
            .iter()
            .map(|(base, prime)| RollingHash::new(&r_diff, *base, *prime))
            .collect::<Vec<_>>();

        for d in 1..(n + 1) / 2 {
            if check_candidate_shapes(shifted_split(
                n,
                d,
                &l,
                &r,
                &l_diff_hashers,
                &r_diff_hashers,
            )) {
                return true;
            }
        }
        l.reverse();
        r.reverse();
    }
    false
}

fn check_connected(a: &[(i32, i32)]) -> bool {
    for i in 0..a.len() {
        if a[i].1 <= a[i].0 {
            return false;
        }
        if i > 0 && a[i].1 <= a[i - 1].0 {
            return false;
        }
        if i > 0 && a[i].0 >= a[i - 1].1 {
            return false;
        }
    }
    true
}

fn check_candidate_shapes(candidate: Option<CandidateSplit>) -> bool {
    let (first, second) = match candidate {
        None => return false,
        Some((first, second)) => (first, second),
    };
    if !check_connected(&first) || !check_connected(&second) {
        return false;
    }
    if first.len() != second.len() {
        return false;
    }
    for i in 0..first.len() {
        let first_size = first[i].1 - first[i].0;
        let first_offset = first[i].0 - first[0].0;
        let second_size = second[i].1 - second[i].0;
        let second_offset = second[i].0 - second[0].0;
        if first_size != second_size || first_offset != second_offset {
            return false;
        }
    }
    true
}

type CandidateSplit = (Vec<(i32, i32)>, Vec<(i32, i32)>);

fn horizontal_split(n: usize, l: &[i32], r: &[i32]) -> Option<CandidateSplit> {
    if n % 2 == 1 {
        return None;
    }
    let mut first = vec![];
    let mut second = vec![];
    let half = n / 2;
    for i in 0..half {
        first.push((l[i], r[i]));
        second.push((l[i + half], r[i + half]));
    }
    Some((first, second))
}

fn vertical_split(n: usize, l: &[i32], r: &[i32]) -> Option<CandidateSplit> {
    let mut first = vec![];
    let mut second = vec![];
    for i in 0..n {
        let mid = (l[i] + r[i]) / 2;
        first.push((l[i], mid));
        second.push((mid, r[i]));
    }
    Some((first, second))
}

fn shifted_split(
    n: usize,
    d: usize,
    l: &[i32],
    r: &[i32],
    l_diff_hashers: &Vec<RollingHash>,
    r_diff_hashers: &Vec<RollingHash>,
) -> Option<CandidateSplit> {
    let first_row_size = r[0] - l[0];
    let first_rd = r[d] - first_row_size;
    if first_rd <= l[d - 1] {
        return None;
    }

    let last_row_size = r[n - 1] - l[n - 1];
    let second_ld = l[n - 1 - d] + last_row_size;
    if second_ld >= r[n - d] {
        return None;
    }

    let k = n - 1 - 2 * d;
    for (l_diff_hasher, r_diff_hasher) in l_diff_hashers.iter().zip(r_diff_hashers.iter()) {
        if l_diff_hasher.query(1, k) != r_diff_hasher.query(2 * d + 1, 2 * d + k) {
            return None;
        }
    }

    let k = d - 1;
    for r_diff_hasher in r_diff_hashers.iter() {
        if r_diff_hasher.query(1, k) != r_diff_hasher.query(d + 1, d + k) {
            return None;
        }
    }
    for l_diff_hasher in l_diff_hashers.iter() {
        if l_diff_hasher.query(n - k, n - 1) != l_diff_hasher.query(n - d - k, n - d - 1) {
            return None;
        }
    }

    let mut first = vec![];
    let mut second = vec![];
    for i in 0..n {
        if i < d {
            first.push((l[i], r[i]));
        } else if i < n - d {
            let mid = r[i] - (first[i - d].1 - first[i - d].0);
            first.push((l[i], mid));
            second.push((mid, r[i]));
        } else {
            second.push((l[i], r[i]));
        }
    }
    Some((first, second))
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(1000000000, 1);

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

fn brute(l: &[i32], r: &[i32]) -> bool {
    let n = l.len();
    let mut l = l.to_vec();
    let mut r = r.to_vec();
    if check_candidate_shapes(horizontal_split(n, &l, &r)) {
        return true;
    }
    if check_candidate_shapes(vertical_split(n, &l, &r)) {
        return true;
    }
    for _ in 0..2 {
        for d in 1..(n + 1) / 2 {
            if check_candidate_shapes(shifted_split_wo_checks(n, d, &l, &r)) {
                return true;
            }
        }
        l.reverse();
        r.reverse();
    }
    false
}

fn shifted_split_wo_checks(n: usize, d: usize, l: &[i32], r: &[i32]) -> Option<CandidateSplit> {
    let mut first = vec![];
    let mut second = vec![];
    for i in 0..n {
        if i < d {
            first.push((l[i], r[i]));
        } else if i < n - d {
            let mid = r[i] - (first[i - d].1 - first[i - d].0);
            first.push((l[i], mid));
            second.push((mid, r[i]));
        } else {
            second.push((l[i], r[i]));
        }
    }
    Some((first, second))
}

fn main() {
    tester::run_tests();

    let precalc = PreCalc::new(1000000000, 1);
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 100 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(1..=20);
        let (l, r) = loop {
            let mut l = vec![0; n];
            let mut r = vec![0; n];
            let mut a = vec![];
            let mut area = 0;
            for i in 0..n {
                l[i] = rng.gen_range(1..=100000000);
                r[i] = l[i] + rng.gen_range(1..=100000000);
                a.push((l[i], r[i]));
                area += (r[i] - l[i]) as i64;
            }
            if area % 2 == 1 || !check_connected(&a) {
                continue;
            }
            break (l, r);
        };
        let x = doit(&l, &r, &precalc);
        let y = brute(&l, &r);
        if x != y {
            eprintln!("Mismatch at {l:?} {r:?}  {x} vs. {y:?}");
            break;
        }
    }
}
//END MAIN
