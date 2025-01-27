//{"name":"F1. Counting Is Not Fun (Easy Version)","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/F1","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3\n2 5\n1 6\n3 4\n4\n1 6\n7 8\n2 3\n4 5\n6\n2 3\n1 6\n7 8\n9 12\n10 11\n4 5\n","output":"5 1 1 1\n14 2 2 1 1\n132 42 5 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F1CountingIsNotFunEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const MAXN: usize = 5000;
const MOD: u32 = 998244353;

struct PreCalc {
    fact: Vec<ModInt<MOD>>,
    inv_fact: Vec<ModInt<MOD>>,
}

impl PreCalc {
    fn new() -> Self {
        let inv = ModInt::<MOD>::gen_inverses(2 * MAXN + 1);
        let mut fact = vec![ModInt::<MOD>::from(1)];
        let mut inv_fact = vec![ModInt::<MOD>::from(1)];
        for i in 1..=2 * MAXN {
            fact.push(fact[i - 1] * i.into());
            inv_fact.push(inv_fact[i - 1] * inv[i]);
        }
        Self { fact, inv_fact }
    }

    fn catalan(self: &Self, n: usize) -> ModInt<MOD> {
        self.fact[2 * n] * self.inv_fact[n + 1] * self.inv_fact[n]
    }

    fn inv_catalan(self: &Self, n: usize) -> ModInt<MOD> {
        self.inv_fact[2 * n] * self.fact[n + 1] * self.fact[n]
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let mut a = vec![0; 2 * n];
    for (i, (l, r)) in input.read_int_pair_vec(n).into_iter().enumerate() {
        let l = l as usize - 1;
        let r = r as usize - 1;
        a[l] = i + 1;
        a[r] = i + 1;
    }
    let mut open = vec![0];
    let mut parent = vec![0; n + 1];
    let mut subtree_uncovered = vec![0; n + 1];
    let mut alive = vec![false; n + 1];
    subtree_uncovered[0] = 2 * n;
    alive[0] = true;
    for i in 0..2 * n {
        if *open.last().unwrap() == a[i] {
            open.pop();
            subtree_uncovered[a[i]] -= 2 * n - i;
            parent[a[i]] = *open.last().unwrap();
        } else {
            open.push(a[i]);
            subtree_uncovered[a[i]] += 2 * n - i - 1;
        }
    }
    let mut result = precalc.catalan(subtree_uncovered[0] / 2);
    let mut results = vec![result];
    for i in 1..=n {
        result *= precalc.catalan(subtree_uncovered[i] / 2);
        let delta = subtree_uncovered[i] + 2;
        let mut j = i;
        while !alive[j] {
            j = parent[j];
            subtree_uncovered[j] -= delta;
        }
        result *= precalc.inv_catalan((subtree_uncovered[j] + delta) / 2);
        result *= precalc.catalan(subtree_uncovered[j] / 2);
        results.push(result);

        alive[i] = true;
    }
    out.print_iter(results.into_iter().map(|r| r.unwrap()));
    out.print_line("");
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

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
