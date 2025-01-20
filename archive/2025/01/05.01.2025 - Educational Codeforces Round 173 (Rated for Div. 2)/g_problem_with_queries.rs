//{"name":"G. Problem with Queries","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/G","interactive":false,"timeLimit":8000,"tests":[{"input":"3\n1 2 3\n5\n2 0 2\n1 0 2\n2 0 2\n1 2 0\n2 1 0\n","output":"3 2 0\n"},{"input":"7\n1 3 4 4 7 1 3\n3\n2 1 6\n2 1 0\n2 5 6\n","output":"13 18 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GProblemWithQueries"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAX_LEN: usize = 100000;
const MAX_VALUE: usize = 100000;
const BLOCK_SIZE: usize = 300;
const N_BLOCKS: usize = MAX_LEN / BLOCK_SIZE + 1;

type PreCalc = ();

struct BlockDecomposition {
    value_prefix_count: [[i32; N_BLOCKS]; MAX_VALUE],
    left_updates: [[i64; N_BLOCKS]; N_BLOCKS],
    right_updates: [[i64; N_BLOCKS]; N_BLOCKS],
}

impl BlockDecomposition {
    fn new() -> Self {
        BlockDecomposition {
            value_prefix_count: [[0; N_BLOCKS]; MAX_VALUE],
            left_updates: [[0; N_BLOCKS]; N_BLOCKS],
            right_updates: [[0; N_BLOCKS]; N_BLOCKS],
        }
    }

    fn update(&mut self, block_index: usize, value: usize, delta: i32) {
        for i in block_index..N_BLOCKS {
            self.value_prefix_count[value][i] += delta;
        }
        for i in 0..=block_index {
            let vc = self.get_value_count(i, block_index, value);
            let pair_count_delta = if delta == -1 { -vc } else { vc - 1 } as i64;
            self.right_updates[i][block_index] += pair_count_delta;
        }
        for j in block_index + 1..N_BLOCKS {
            let vc = self.get_value_count(block_index + 1, j, value);
            let pair_count_delta = if delta == -1 { -vc } else { vc } as i64;
            self.left_updates[block_index][j] += pair_count_delta;
        }
    }

    fn get_value_count(&self, block_lo: usize, block_hi: usize, value: usize) -> i32 {
        let mut result = self.value_prefix_count[value][block_hi];
        if block_lo > 0 {
            result -= self.value_prefix_count[value][block_lo - 1]
        }
        result
    }

    fn get_block_segment_pair_count(&self, block_lo: usize, block_hi: usize) -> i64 {
        let mut result = 0;
        for i in block_lo..=block_hi {
            result += self.right_updates[block_lo][i];
            result += self.left_updates[i][block_hi];
        }
        result
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int() as usize;
    let mut a: Vec<usize> = (0..n).map(|_| input.read_int() as usize - 1).collect();
    let mut b = BlockDecomposition::new();
    let mut c = vec![0; MAX_VALUE];
    for (i, value) in a.iter().enumerate() {
        let block_index = i / BLOCK_SIZE;
        b.update(block_index, *value, 1);
    }

    let mut last = 0;
    let q = input.read_int() as usize;
    for _ in 0..q {
        let t = input.read_int();
        let x = (input.read_int() as usize + last) % n + 1;
        let y = (input.read_int() as usize + last) % n + 1;
        if t == 1 {
            let i = x - 1;
            let old_value = a[i];
            let new_value = y - 1;
            let block_index = i / BLOCK_SIZE;
            b.update(block_index, old_value, -1);
            b.update(block_index, new_value, 1);
            a[i] = new_value;
        } else {
            let lo = x.min(y) - 1;
            let hi = x.max(y) - 1;
            let block_lo = lo / BLOCK_SIZE;
            let block_hi = hi / BLOCK_SIZE;
            let mut result = (hi - lo) as i64 * (hi - lo + 1) as i64 / 2;
            result -= b.get_block_segment_pair_count(block_lo, block_hi);
            for i in block_lo * BLOCK_SIZE..lo {
                result += b.get_value_count(block_lo, block_hi, a[i]) as i64 - 1;
                result -= c[a[i]];
                c[a[i]] += 1;
            }
            for i in hi + 1..((block_hi + 1) * BLOCK_SIZE).min(a.len()) {
                result += b.get_value_count(block_lo, block_hi, a[i]) as i64 - 1;
                result -= c[a[i]];
                c[a[i]] += 1;
            }
            for i in block_lo * BLOCK_SIZE..lo {
                c[a[i]] -= 1;
            }
            for i in hi + 1..((block_hi + 1) * BLOCK_SIZE).min(a.len()) {
                c[a[i]] -= 1;
            }
            out.print_line(result);
            last = result as usize;
        }
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
