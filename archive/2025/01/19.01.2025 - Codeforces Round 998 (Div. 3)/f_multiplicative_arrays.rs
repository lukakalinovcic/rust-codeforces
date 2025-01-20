//{"name":"F. Multiplicative Arrays","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n2 2\n4 3\n10 6969420\n","output":"2 3\n3 6 6 10\n6969420 124188773 124188773 729965558 124188773 337497990 124188773 50981194 729965558 337497990\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMultiplicativeArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const MOD: u32 = 998244353;
const MAX: usize = 100100;
const MAX_FACTORS: usize = 20;

struct PreCalc {
    min_factor: Vec<(usize, usize)>,
    inv: Vec<ModInt<MOD>>,
    bernoulli: Vec<ModInt<MOD>>,
}

impl PreCalc {
    fn new(n: usize, n_bernoulli: usize) -> Self {
        let mut min_factor = vec![(0, 0); n + 1];
        for i in 2..=n {
            if min_factor[i].0 != 0 {
                continue;
            }
            min_factor[i] = (i, 1);
            let mut j = i * i;
            let mut k = i;
            while j <= n {
                if min_factor[j].0 == 0 {
                    min_factor[j] = (i, k);
                }
                k += 1;
                j += i;
            }
        }
        let inv = ModInt::<MOD>::gen_inverses(n + 1);

        let mut bernoulli: Vec<ModInt<MOD>> = vec![1.into(); n_bernoulli];
        for m in 0..n_bernoulli {
            for k in 0..m {
                let mut choose: ModInt<MOD> = 1.into();
                for i in 1..=k {
                    choose *= inv[i] * (m - i + 1).into();
                }
                let bk = bernoulli[k];
                bernoulli[m] -= choose * bk * inv[m - k + 1];
            }
        }

        Self {
            min_factor,
            inv,
            bernoulli,
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let k: usize = input.read();
    let n: usize = input.read();
    let mut results = vec![];

    let mut power_sum: Vec<ModInt<MOD>> = vec![0.into(); MAX_FACTORS];
    power_sum[0] = n.into();
    for i in 1..MAX_FACTORS {
        let mut choose: ModInt<MOD> = 1.into();
        for j in 0..=i {
            power_sum[i] +=
                choose * precalc.bernoulli[j] * ModInt::<MOD>::from(n).power((i + 1 - j) as u32);
            choose *= precalc.inv[j + 1] * (i + 1 - j).into();
        }
        power_sum[i] *= precalc.inv[i + 1];
    }

    for x in 1..=k {
        let mut t = x;
        let mut poly: Vec<ModInt<MOD>> = vec![1.into()];
        let mut denom: ModInt<MOD> = 1.into();
        while t != 1 {
            let prime = precalc.min_factor[t].0;
            let mut power = 0;
            while prime == precalc.min_factor[t].0 {
                t = precalc.min_factor[t].1;
                power += 1;
            }

            for i in 1..=power {
                poly.insert(0, 0.into());
                for j in 0..poly.len() - 1 {
                    let add = poly[j + 1] * (power - i).into();
                    poly[j] += add;
                }
                denom *= precalc.inv[i];
            }
        }

        let mut result: ModInt<MOD> = 0.into();
        for j in (0..poly.len()).rev() {
            result += poly[j] * power_sum[j];
        }
        result *= denom;
        results.push(result.unwrap());
    }
    out.print_iter(results.into_iter());
    out.print_line("");
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(MAX, MAX_FACTORS);

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
