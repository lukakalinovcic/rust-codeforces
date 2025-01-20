//{"name":"F. Number of Cubes","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"6\n1 1 1 1\n1\n6 1 1 3\n1 2 3\n12 1 1 3\n2 4 6\n3 3 1 2\n3 6\n2 3 3 2\n6 12\n72 60 96 4\n17280 86400 120960 190080\n","output":"1\n10\n1160\n12\n1044\n231490207\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FNumberOfCubes"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    fact: Vec<i32>,
    inverse_fact: Vec<i32>,
    phi: Vec<i32>,
}

const MOD: i64 = 998244353;
const MAX: usize = 3000000;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 1 {
        (a * pow(a, n - 1)) % MOD
    } else {
        let x = pow(a, n / 2);
        (x * x) % MOD
    }
}

fn invert(a: i64) -> i64 {
    pow(a, MOD - 2)
}

fn gen(f: &[(i64, i32)], i: usize, mut divisor: i64, mut factors: i32, out: &mut Vec<(i64, i32)>) {
    if i == f.len() {
        out.push((divisor, factors));
    } else {
        for _ in 0..=f[i].1 {
            gen(f, i + 1, divisor, factors, out);
            divisor *= f[i].0;
            factors += 1;
        }
    }
}

fn divisors(mut a: i64) -> Vec<(i64, i32)> {
    let mut f = vec![];
    for d in 2.. {
        if d * d > a {
            break;
        }
        let mut cnt = 0;
        while a % d == 0 {
            cnt += 1;
            a /= d;
        }
        f.push((d, cnt));
    }
    if a > 1 {
        f.push((a, 1))
    }
    let mut divisors = vec![];
    gen(&f, 0, 1, 0, &mut divisors);
    divisors
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, data: &mut PreCalc) {
    let a = input.read_long();
    let b = input.read_long();
    let c = input.read_long();
    let k = input.read();
    let d = input.read_long_vec(k);
    let mut g = 0;
    for x in d.iter() {
        g = gcd(g, *x);
    }
    let adivs = divisors(a);
    let bdivs = divisors(b);
    let cdivs = divisors(c);
    let mut result = 0;
    let mut multinomial_cache = HashMap::new();
    for (x, _) in adivs.iter() {
        for (y, _) in bdivs.iter() {
            for (z, _) in cdivs.iter() {
                let (x, y, z) = (*x, *y, *z);
                let mut lcm = x;
                lcm = (lcm * y) / gcd(lcm, y);
                lcm = (lcm * z) / gcd(lcm, z);
                if g % lcm != 0 {
                    continue;
                }
                let cnt = data.phi[x as usize] as i64
                    * data.phi[y as usize] as i64
                    * data.phi[z as usize] as i64;
                let multinomial = if let Some(result) = multinomial_cache.get(&lcm) {
                    *result
                } else {
                    let mut multinomial = data.fact[(a * b * c / lcm) as usize] as i64;
                    for k in d.iter() {
                        multinomial =
                            (multinomial * data.inverse_fact[(*k / lcm) as usize] as i64) % MOD;
                    }
                    multinomial_cache.insert(lcm, multinomial);
                    multinomial
                };
                result = (result + cnt * multinomial) % MOD;
            }
        }
    }
    out.print_line(result * invert(a * b * c) % MOD);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut fact = vec![1; MAX + 1];
    let mut inverse_fact = vec![1; MAX + 1];
    let mut factor = vec![0; MAX + 1];
    let mut phi = vec![1; MAX + 1];
    for i in 2..=MAX {
        fact[i] = ((i as i64 * fact[i - 1] as i64) % MOD) as i32;
        inverse_fact[i] = invert(fact[i] as i64) as i32;

        if factor[i] == 0 {
            let mut j = i * i;
            while j <= MAX {
                if factor[j] == 0 {
                    factor[j] = i as i32;
                }
                j += i;
            }
            factor[i] = i as i32;
        }
        let mut d = i as i32;
        let mut p = 1;
        while d % factor[i] == 0 {
            d /= factor[i];
            p *= factor[i];
        }
        phi[i] = (phi[d as usize] as i64 * (p - p / factor[i]) as i64) as i32;
    }
    let mut pre_calc = PreCalc {
        fact,
        inverse_fact,
        phi,
    };

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
