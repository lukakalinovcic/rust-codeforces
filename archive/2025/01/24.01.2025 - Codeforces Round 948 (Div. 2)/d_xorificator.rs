//{"name":"D. XORificator","group":"Codeforces - Codeforces Round 948 (Div. 2)","url":"https://codeforces.com/contest/1977/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4\n1010\n0110\n0100\n1 1\n1\n1 1\n0\n2 5\n00101\n10110\n3 3\n101\n111\n000\n","output":"3\n010\n1\n0\n1\n1\n3\n00\n2\n010\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DXORificator"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const BASE1: u32 = 1337;
const BASE2: u32 = 31337;
const MOD1: u32 = 1000000007;
const MOD2: u32 = 998244353;
const MAX: usize = 300000;

struct PreCalc {
    base1_pow: Vec<ModInt<MOD1>>,
    base2_pow: Vec<ModInt<MOD2>>,
}

impl PreCalc {
    fn new() -> Self {
        let mut base1_pow = vec![];
        let mut base2_pow = vec![];
        let mut x1 = 1.into();
        let mut x2 = 1.into();
        for _ in 0..MAX {
            base1_pow.push(x1);
            base2_pow.push(x2);
            x1 *= BASE1.into();
            x2 *= BASE2.into();
        }
        Self {
            base1_pow,
            base2_pow,
        }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let a = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();
    let mut cnt: HashMap<(u32, u32), i32> = HashMap::new();
    let mut max_cnt = 0;
    let mut max_ji = (0, 0);
    for j in 0..m {
        let mut h1: ModInt<MOD1> = 0.into();
        let mut h2: ModInt<MOD2> = 0.into();
        for i in 0..n {
            if a[i][j] == b'1' {
                h1 += precalc.base1_pow[i];
                h2 += precalc.base2_pow[i];
            }
        }
        for i in 0..n {
            let (h1, h2) = if a[i][j] == b'1' {
                let h1 = (h1 - precalc.base1_pow[i]).unwrap();
                let h2 = (h2 - precalc.base2_pow[i]).unwrap();
                (h1, h2)
            } else {
                let h1 = (h1 + precalc.base1_pow[i]).unwrap();
                let h2 = (h2 + precalc.base2_pow[i]).unwrap();
                (h1, h2)
            };
            let entry = cnt.entry((h1, h2)).or_default();
            *entry += 1 as i32;
            if *entry > max_cnt {
                max_cnt = *entry;
                max_ji = (j, i);
            }
        }
    }
    out.print_line(max_cnt);
    let (j, i) = max_ji;
    let mut s = vec![0; n];
    for k in 0..n {
        s[k] = a[k][j];
    }
    s[i] ^= 1;
    out.print_line(String::from_utf8(s).unwrap());
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
