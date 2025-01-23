//{"name":"E. Fenwick Tree","group":"Codeforces - Codeforces Round 942 (Div. 2)","url":"https://codeforces.com/contest/1972/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n8 1\n1 2 1 4 1 2 1 8\n6 2\n1 4 3 17 5 16\n","output":"1 1 1 1 1 1 1 1\n1 2 3 4 5 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFenwickTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 998244353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let k: ModInt<MOD> = input.read_int().into();
    let b: Vec<ModInt<MOD>> = input
        .read_int_vec(n)
        .into_iter()
        .map(|b| b.into())
        .collect();
    let a = doit(n, k, b);
    out.print_iter(a.into_iter().map(|a| a.unwrap()));
    out.print_line("");
}

fn doit(n: usize, k: ModInt<MOD>, b: Vec<ModInt<MOD>>) -> Vec<ModInt<MOD>> {
    let mut c: Vec<ModInt<MOD>> = vec![1.into(); 20];
    for p in 1..20 {
        c[p] = c[p - 1] * (k + (p - 1).into()) / p.into();
    }
    let mut k = vec![0; n];
    let mut a = vec![];
    for i in 0..n {
        let mut p = 0;
        while ((i + 1) >> p) & 1 == 0 {
            p += 1;
        }

        let mut ai = b[i];
        for j in i + 1 - (1 << p)..i {
            ai -= a[j] * c[k[j]];
            k[j] += 1;
        }
        a.push(ai);
        k[i] += 1;
    }
    a
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
