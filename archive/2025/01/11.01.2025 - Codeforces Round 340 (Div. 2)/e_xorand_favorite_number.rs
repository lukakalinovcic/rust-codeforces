//{"name":"E. XOR and Favorite Number","group":"Codeforces - Codeforces Round 340 (Div. 2)","url":"https://codeforces.com/contest/617/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"6 2 3\n1 2 1 1 0 3\n1 6\n3 5\n","output":"7\n0\n"},{"input":"5 3 1\n1 1 1 1 1\n1 5\n2 4\n1 3\n","output":"9\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EXORAndFavoriteNumber"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// Mo's algorithm.
struct PairCounter {
    num_pairs: i64,
    open: [i32; 1 << 20],
    closed: [i32; 1 << 20],
}

impl PairCounter {
    fn new() -> Self {
        Self {
            num_pairs: 0,
            open: [0; 1 << 20],
            closed: [0; 1 << 20],
        }
    }

    fn add_right(&mut self, x: usize, is_open: bool) {
        if is_open {
            self.open[x] += 1;
        } else {
            self.num_pairs += self.open[x] as i64;
            self.closed[x] += 1;
        }
    }

    fn remove_right(&mut self, x: usize, is_open: bool) {
        if is_open {
            self.open[x] -= 1;
        } else {
            self.num_pairs -= self.open[x] as i64;
            self.closed[x] -= 1;
        }
    }

    fn add_left(&mut self, x: usize, is_open: bool) {
        if is_open {
            self.num_pairs += self.closed[x] as i64;
            self.open[x] += 1;
        } else {
            self.closed[x] += 1;
        }
    }

    fn remove_left(&mut self, x: usize, is_open: bool) {
        if is_open {
            self.num_pairs -= self.closed[x] as i64;
            self.open[x] -= 1;
        } else {
            self.closed[x] -= 1;
        }
    }
}

const BLOCK_SIZE: usize = 1000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let k = input.read_int();
    let mut prefix_xor = vec![0; n + 1];
    prefix_xor[0] = 0;
    for (i, a) in input.read_int_vec(n).into_iter().enumerate() {
        prefix_xor[i + 1] = prefix_xor[i] ^ a;
    }

    let mut queries = input
        .read_int_pair_vec(q)
        .into_iter()
        .enumerate()
        .map(|(i, (a, b))| {
            let block = a as usize / BLOCK_SIZE;
            (block, b as usize, a as usize, i)
        })
        .collect::<Vec<_>>();
    queries.sort();

    let mut lt = 1;
    let mut rt = 0;
    let mut pc = PairCounter::new();
    let mut results = vec![0; q];
    for (_, b, a, i) in queries {
        while lt > a {
            lt -= 1;
            pc.add_left(prefix_xor[lt] as usize, false);
            pc.add_left((prefix_xor[lt - 1] ^ k) as usize, true);
        }
        while rt < b {
            rt += 1;
            pc.add_right((prefix_xor[rt - 1] ^ k) as usize, true);
            pc.add_right(prefix_xor[rt] as usize, false);
        }
        while lt < a {
            pc.remove_left((prefix_xor[lt - 1] ^ k) as usize, true);
            pc.remove_left(prefix_xor[lt] as usize, false);
            lt += 1;
        }
        while rt > b {
            pc.remove_right(prefix_xor[rt] as usize, false);
            pc.remove_right((prefix_xor[rt - 1] ^ k) as usize, true);
            rt -= 1;
        }
        results[i] = pc.num_pairs;
    }
    for result in results {
        out.print_line(result);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
