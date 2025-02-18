//{"name":"E. Mycraft Sand Sort","group":"Codeforces - Codeforces Round 1005 (Div. 2)","url":"https://codeforces.com/contest/2064/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n1\n1\n1\n5\n5 3 4 1 2\n1 1 1 1 1\n5\n4 2 3 1 5\n2 1 4 1 5\n40\n29 15 20 35 37 31 27 1 32 36 38 25 22 8 16 7 3 28 11 12 23 4 14 9 39 13 10 30 6 2 24 17 19 5 34 18 33 26 40 21\n3 1 2 2 1 2 3 1 1 1 1 2 1 3 1 1 3 1 1 1 2 2 1 3 3 3 2 3 2 2 2 2 1 3 2 1 1 2 2 2\n","output":"1\n120\n1\n143654893\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMycraftSandSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 998244353;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);
    let c = input.read_int_vec(n);
    let mut group = vec![0; n];
    let mut group_size = vec![0; n];
    let mut group_color = vec![0; n];
    let mut group_idx = 0;
    let mut prev = c[0];
    group_color[0] = c[0];
    let mut group_prev = vec![n; n];
    let mut group_next = vec![n; n];
    for i in 0..n {
        if c[i] != prev {
            group_idx += 1;
            group_color[group_idx] = c[i];
            group_prev[group_idx] = group_idx - 1;
            group_next[group_idx - 1] = group_idx;
            prev = c[i];
        }
        group[p[i] as usize - 1] = group_idx;
        group_size[group_idx] += 1;
    }
    let mut dsu = DSU::new(group_idx + 1);
    let mut result: ModInt<MOD> = 1.into();
    for i in 0..n {
        let g = dsu.find(group[i] as i32) as usize;
        result *= group_size[g].into();
        group_size[g] -= 1;
        if group_size[g] == 0 {
            let prev = group_prev[g];
            let next = group_next[g];
            if prev != n {
                group_next[prev] = next;
            }
            if next != n {
                group_prev[next] = prev;
            }
            if prev != n && next != n && group_color[prev] == group_color[next] {
                let g = dsu.union(prev as i32, next as i32) as usize;
                group_size[g] = group_size[prev] + group_size[next];
                group_prev[g] = group_prev[prev];
                group_next[g] = group_next[next];
                if group_prev[g] != n {
                    group_next[group_prev[g]] = g;
                }
                if group_next[g] != n {
                    group_prev[group_next[g]] = g;
                }
            }
        }
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
