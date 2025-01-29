//{"name":"B. Index and Maximum Value","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5 5\n1 2 3 2 1\n+ 1 3\n- 2 3\n+ 1 2\n+ 2 4\n- 6 8\n5 5\n1 3 3 4 5\n+ 1 4\n+ 2 3\n- 4 5\n- 3 3\n- 2 6\n5 5\n1 1 1 1 1\n+ 2 3\n- 4 5\n+ 1 6\n- 2 5\n+ 1 8\n1 1\n1\n- 1 1\n1 1\n1000000000\n+ 1000000000 1000000000\n","output":"4 4 4 5 5\n5 5 4 4 3\n1 1 2 1 2\n0\n1000000001\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIndexAndMaximumValue"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Fenwick {
    a: Vec<i32>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Self { a: vec![0; n + 1] }
    }

    fn add(&mut self, mut x: usize, delta: i32) {
        let n = self.a.len() - 1;
        while x <= n {
            self.a[x] += delta;
            x += (x as i32 & -(x as i32)) as usize;
        }
    }

    fn prefix_sum(&self, mut x: usize) -> i32 {
        let mut sum = 0;
        while x > 0 {
            sum += self.a[x];
            x -= (x as i32 & -(x as i32)) as usize;
        }
        sum
    }

    fn prefix_sum_lower_bound(&self, lo: i32) -> usize {
        let n = self.a.len() - 1;
        let mut p = n.next_power_of_two();
        let mut sum = 0;
        let mut i = 0;
        while p > 0 {
            if i + p <= n && sum + self.a[i + p] < lo {
                sum += self.a[i + p];
                i += p;
            }
            p /= 2;
        }
        i + 1
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let mut a = input.read_int_vec(n);
    a.sort();
    let mut f = Fenwick::new(n + 1);
    f.add(1, a[0]);
    for i in 1..n {
        f.add(i + 1, a[i] - a[i - 1]);
    }
    f.add(n + 1, 1000000001);
    let mut result = vec![];
    for _ in 0..m {
        let c = input.read_char();
        let l = input.read_int();
        let r = input.read_int();
        let l = f.prefix_sum_lower_bound(l);
        let r = f.prefix_sum_lower_bound(r + 1);
        f.add(l, if c == '+' { 1 } else { -1 });
        f.add(r, if c == '+' { -1 } else { 1 });
        result.push(f.prefix_sum(n))
    }
    out.print_line(result);
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
