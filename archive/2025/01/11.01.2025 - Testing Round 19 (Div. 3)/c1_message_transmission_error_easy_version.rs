//{"name":"C1. Message Transmission Error (easy version)","group":"Codeforces - Testing Round 19 (Div. 3)","url":"https://codeforces.com/contest/2010/problem/C1","interactive":false,"timeLimit":2000,"tests":[{"input":"abrakadabrabrakadabra\n","output":"YES\nabrakadabra\n"},{"input":"acacacaca\n","output":"YES\nacaca\n"},{"input":"abcabc\n","output":"NO\n"},{"input":"abababab\n","output":"YES\nababab\n"},{"input":"tatbt\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C1MessageTransmissionErrorEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn mult(a: i32, b: i32, p: i32) -> i32 {
    (((a as i64) * (b as i64)) % (p as i64)) as i32
}

struct RollingHash {
    p: i32,
    b_pow_i: Vec<i32>,
    prefix_sum: Vec<i32>,
}

impl RollingHash {
    fn new(a: &[u8], b: i32, p: i32) -> Self {
        let n = a.len();
        let mut prefix_sum = vec![0; n + 1];
        let mut b_pow_i = vec![0; n + 1];
        b_pow_i[0] = 1;
        for i in 0..n {
            prefix_sum[i + 1] = (mult(prefix_sum[i], b, p) + a[i] as i32) % p;
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
        let hi_sum = self.prefix_sum[hi];
        let lo_sum = mult(self.prefix_sum[lo - 1], self.b_pow_i[hi - lo + 1], self.p);
        (hi_sum + self.p - lo_sum) % self.p
    }
}

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

fn pick_hash_params(prime_lo: i32, n: usize) -> Vec<(i32, i32)> {
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
    params
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let a = input.read_line().into_bytes();
    let hashers = pick_hash_params(1000000000, 5)
        .into_iter()
        .map(|(base, prime)| RollingHash::new(&a, base, prime))
        .collect::<Vec<_>>();

    let n = a.len();
    for i in 2..=(n + 1) / 2 {
        let mut ok = true;
        for hasher in &hashers {
            if hasher.query(i, n) != hasher.query(1, n - i + 1) {
                ok = false;
                break;
            }
        }
        if ok {
            out.print_line("YES");
            out.print_line(String::from_utf8(a[i - 1..].to_vec()).unwrap());
            return;
        }
    }
    out.print_line("NO");
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
