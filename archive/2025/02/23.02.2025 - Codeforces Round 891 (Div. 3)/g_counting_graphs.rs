//{"name":"G. Counting Graphs","group":"Codeforces - Codeforces Round 891 (Div. 3)","url":"https://codeforces.com/contest/1857/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 5\n1 2 4\n4 5\n1 2 2\n2 3 4\n3 4 3\n5 6\n1 2 3\n1 3 2\n3 4 6\n3 5 1\n10 200\n1 2 3\n2 3 33\n3 4 200\n1 5 132\n5 6 1\n5 7 29\n7 8 187\n7 9 20\n7 10 4\n","output":"1\n8\n80\n650867886\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCountingGraphs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 998244353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_int();
    let mut e = vec![];
    for _ in 0..n - 1 {
        let u = input.read_int() - 1;
        let v = input.read_int() - 1;
        let w = input.read_int();
        e.push((w, u, v));
    }
    e.sort();
    let mut dsu = DSU::new(n);
    let mut sz = vec![1; n];
    let mut result: ModInt<MOD> = 1.into();
    for (w, u, v) in e {
        let u = dsu.find(u);
        let v = dsu.find(v);
        let sz_u = sz[u as usize];
        let sz_v = sz[v as usize];
        let k = (sz_u as i64) * (sz_v as i64) - 1;
        result *= ModInt::<MOD>::from(s - w + 1).power(k as u64);
        let u = dsu.union(u, v);
        sz[u as usize] = sz_u + sz_v;
    }
    out.print_line(result.unwrap());
}

struct DSU {
    p: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            p: (0..n as i32).collect::<Vec<_>>(),
        }
    }

    fn find(&mut self, u: i32) -> i32 {
        if self.p[u as usize] != u {
            self.p[u as usize] = self.find(self.p[u as usize]);
        }
        self.p[u as usize]
    }

    fn union(&mut self, u: i32, v: i32) -> i32 {
        let u = self.find(u);
        let v = self.find(v);
        if u == v {
            u
        } else {
            self.p[v as usize] = u;
            u
        }
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

fn main() {
    tester::run_tests();
}
//END MAIN
