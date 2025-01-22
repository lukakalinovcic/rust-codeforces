//{"name":"F. Expected Median","group":"Codeforces - Codeforces Round 964 (Div. 4)","url":"https://codeforces.com/contest/1999/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n4 3\n1 0 0 1\n5 1\n1 1 1 1 1\n5 5\n0 1 0 1 0\n6 3\n1 0 1 0 1 1\n4 3\n1 0 1 1\n5 3\n1 0 1 1 0\n2 1\n0 0\n34 17\n1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1\n","output":"2\n5\n0\n16\n4\n7\n0\n333606206\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FExpectedMedian"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const MAX: usize = 200000;
const MOD: u32 = 1000000007;

struct PreCalc {
    fact: Vec<ModInt<MOD>>,
    inv_fact: Vec<ModInt<MOD>>,
}

impl PreCalc {
    fn new(n: usize) -> Self {
        let inv = ModInt::<MOD>::gen_inverses(n);
        let mut fact = vec![ModInt::<MOD>::from(1)];
        let mut inv_fact = vec![ModInt::<MOD>::from(1)];
        for i in 1..n {
            fact.push(fact[i - 1] * i.into());
            inv_fact.push(inv_fact[i - 1] * inv[i]);
        }
        Self { fact, inv_fact }
    }

    fn choose(&self, n: usize, k: usize) -> ModInt<MOD> {
        self.fact[n] * self.inv_fact[k] * self.inv_fact[n - k]
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let k: usize = input.read();
    let mut a = input.read_int_vec(n);
    a.sort();
    let k = k / 2;
    let mut result: ModInt<MOD> = 0.into();
    for i in k..n - k {
        result += precalc.choose(i, k) * precalc.choose(n - i - 1, k) * a[i].into();
    }
    out.print_line(result.unwrap());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(MAX + 1);

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
