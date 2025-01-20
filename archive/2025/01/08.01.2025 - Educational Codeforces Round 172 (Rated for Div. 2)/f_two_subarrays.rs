//{"name":"F. Two Subarrays","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/contest/2042/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n3 -1 4 -3 2 4 0\n0 6 1 0 -3 -2 -1\n6\n3 1 7\n1 2 0\n3 3 6\n2 5 -3\n1 3 2\n3 1 5\n","output":"18\n7\n16\n"},{"input":"10\n2 -1 -3 -2 0 4 5 6 2 5\n2 -4 -5 -1 6 2 5 -6 4 2\n10\n3 6 7\n1 10 -2\n3 5 7\n3 2 8\n2 1 -5\n2 7 4\n3 1 3\n3 3 8\n3 2 3\n1 4 4\n","output":"23\n28\n28\n-17\n27\n-22\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FTwoSubarrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000;
const BLOCK_SIZE: usize = 300;

struct TmpArrays {
    lt_c: [i64; BLOCK_SIZE],
    lt_o: [i64; BLOCK_SIZE],
    lt_oc: [i64; BLOCK_SIZE],
}

#[derive(Clone, Debug)]
struct ProcessingResult {
    ococ: i64,
    oco: i64,
    coc: i64,
    oc: i64,
    co: i64,
    o: i64,
    c: i64,
    pass: i64,
}

impl Default for ProcessingResult {
    fn default() -> Self {
        ProcessingResult {
            ococ: -INF,
            oco: -INF,
            coc: -INF,
            oc: -INF,
            co: -INF,
            o: -INF,
            c: -INF,
            pass: -INF,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    n: usize,
    a: [i64; BLOCK_SIZE],
    b: [i64; BLOCK_SIZE],
    result: ProcessingResult,
}

impl Default for Block {
    fn default() -> Self {
        Block {
            n: 0,
            a: [0; BLOCK_SIZE],
            b: [0; BLOCK_SIZE],
            result: ProcessingResult::default(),
        }
    }
}

impl Block {
    fn reprocess(&mut self, tmp: &mut TmpArrays) {
        self.result = process(&self.a, &self.b, tmp);
    }
}

fn process(a: &[i64], b: &[i64], tmp: &mut TmpArrays) -> ProcessingResult {
    let n = a.len();
    let mut result = ProcessingResult::default();
    let mut prev_a = 0;
    let mut prev_b = 0;
    let mut curr_o = 0;
    let mut curr_c = 0;
    let mut best_o = -INF;
    let mut best_c = -INF;
    let mut best_oc = -INF;
    for i in 0..n {
        curr_o += -prev_a + b[i] - prev_b;
        curr_c += a[i] + b[i] - prev_b;
        best_o = best_o.max(curr_o);
        best_c = best_c.max(curr_c);
        let curr_oc = curr_c + best_o;
        best_oc = best_oc.max(curr_oc);
        tmp.lt_o[i] = best_o;
        tmp.lt_c[i] = best_c;
        tmp.lt_oc[i] = best_oc;
        prev_a = a[i];
        prev_b = b[i];
    }
    result.c = best_c;
    result.oc = best_oc;
    result.pass = curr_c - b[n - 1];
    let mut prev_a = 0;
    let mut prev_b = 0;
    let mut curr_o = 0;
    let mut curr_c = 0;
    let mut best_o = -INF;
    let mut best_c = -INF;
    let mut best_oc = -INF;
    for i in (0..n).rev() {
        curr_c += -prev_a + b[i] - prev_b;
        curr_o += a[i] + b[i] - prev_b;
        best_c = best_c.max(curr_c);
        best_o = best_o.max(curr_o);
        let curr_oc = curr_o + best_c;
        best_oc = best_oc.max(curr_oc);
        prev_a = a[i];
        prev_b = b[i];
        if i > 0 {
            result.ococ = result.ococ.max(tmp.lt_oc[i - 1] + best_oc);
            result.oco = result.oco.max(tmp.lt_oc[i - 1] + best_o);
            result.coc = result.coc.max(tmp.lt_c[i - 1] + best_oc);
            result.co = result.co.max(tmp.lt_c[i - 1] + best_o);
        }
    }
    result.o = best_o;
    result
}

fn doit(block_results: Vec<ProcessingResult>) -> i64 {
    let mut prev_o = -INF;
    let mut prev_oc = -INF;
    let mut prev_oco = -INF;
    let mut prev_ococ = -INF;
    for block in block_results {
        let curr_o = block.o.max(prev_o + block.pass);
        let curr_oc = block.oc.max(prev_o + block.c).max(prev_oc);
        let curr_oco = block
            .oco
            .max(prev_o + block.co)
            .max(prev_oc + block.o)
            .max(prev_oco + block.pass);
        let curr_ococ = block
            .ococ
            .max(prev_o + block.coc)
            .max(prev_oc + block.oc)
            .max(prev_oco + block.c)
            .max(prev_ococ);
        prev_o = curr_o;
        prev_oc = curr_oc;
        prev_oco = curr_oco;
        prev_ococ = curr_ococ;
    }
    prev_ococ
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let mut blocks = vec![Block::default(); n / BLOCK_SIZE + 1];
    let mut tmp = TmpArrays {
        lt_c: [0; BLOCK_SIZE],
        lt_o: [0; BLOCK_SIZE],
        lt_oc: [0; BLOCK_SIZE],
    };
    for (i, chunk) in a.chunks(BLOCK_SIZE).enumerate() {
        blocks[i].n = chunk.len();
        blocks[i].a[0..chunk.len()].copy_from_slice(chunk);
    }
    for (i, chunk) in b.chunks(BLOCK_SIZE).enumerate() {
        blocks[i].b[0..chunk.len()].copy_from_slice(chunk);
        blocks[i].reprocess(&mut tmp);
    }
    let q = input.read();
    for _ in 0..q {
        let t = input.read_int();
        if t == 1 {
            let (p, x) = (input.read_int() as usize - 1, input.read_long());
            let i = p / BLOCK_SIZE;
            blocks[i].a[p % BLOCK_SIZE] = x;
            blocks[i].reprocess(&mut tmp);
        } else if t == 2 {
            let (p, x) = (input.read_int() as usize - 1, input.read_long());
            let i = p / BLOCK_SIZE;
            blocks[i].b[p % BLOCK_SIZE] = x;
            blocks[i].reprocess(&mut tmp);
        } else {
            let (l, r) = (input.read_int() as usize - 1, input.read_int() as usize - 1);
            let block_l = l / BLOCK_SIZE;
            let block_r = r / BLOCK_SIZE;
            let mut block_results = vec![];
            for i in block_l..=block_r {
                if i == block_l || i == block_r {
                    let lo = if i == block_l { l % BLOCK_SIZE } else { 0 };
                    let hi = if i == block_r {
                        r % BLOCK_SIZE + 1
                    } else {
                        blocks[i].n
                    };
                    block_results.push(process(
                        &blocks[i].a[lo..hi],
                        &blocks[i].b[lo..hi],
                        &mut tmp,
                    ));
                } else {
                    block_results.push(blocks[i].result.clone());
                }
            }

            out.print_line(doit(block_results));
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
