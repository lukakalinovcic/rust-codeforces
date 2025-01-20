//{"name":"D1. The Endspeaker (Easy Version)","group":"Codeforces - Codeforces Round 982 (Div. 2)","url":"https://codeforces.com/contest/2027/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 2\n9 3 4 3\n11 7\n1 2\n20\n19 18\n10 2\n2 5 2 1 10 3 2 9 9 6\n17 9\n10 11\n2 2 2 2 2 2 2 2 2 2\n20 18 16 14 12 10 8 6 4 2 1\n1 6\n10\n32 16 8 4 2 1\n","output":"1\n-1\n2\n10\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1TheEndspeakerEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

struct MinStack {
    a: Vec<(i64, i64)>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            a: vec![(INF, INF)],
        }
    }

    fn push(&mut self, x: i64) {
        self.a.push((x, x.min(self.a.last().unwrap().1)));
    }

    fn pop(&mut self) -> i64 {
        self.a.pop().unwrap().0
    }

    fn min(&self) -> i64 {
        self.a.last().unwrap().1
    }

    fn is_empty(&self) -> bool {
        self.a.len() == 1
    }
}

struct MinQueue {
    front: MinStack,
    back: MinStack,
}

impl MinQueue {
    fn new() -> Self {
        Self {
            front: MinStack::new(),
            back: MinStack::new(),
        }
    }

    fn push(&mut self, x: i64) {
        self.front.push(x);
    }

    fn pop(&mut self) -> i64 {
        if self.back.is_empty() {
            while !self.front.is_empty() {
                self.back.push(self.front.pop());
            }
        }
        self.back.pop()
    }

    fn min(&self) -> i64 {
        self.back.min().min(self.front.min())
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(m);
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for k in 0..m {
        let cost = (m - k - 1) as i64;
        let mut sum = 0;
        let mut lo = 0;
        let mut q = MinQueue::new();
        q.push(dp[0]);
        for hi in 1..=n {
            sum += a[hi - 1];
            while sum > b[k] {
                sum -= a[lo];
                lo += 1;
                q.pop();
            }
            dp[hi] = dp[hi].min(cost + q.min());
            q.push(dp[hi]);
        }
    }
    if dp[n] == INF {
        out.print_line(-1);
    } else {
        out.print_line(dp[n]);
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
