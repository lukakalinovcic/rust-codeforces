//{"name":"D. Sums of Segments","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 2 5 10\n15\n1 1\n1 2\n1 3\n1 4\n1 5\n1 10\n5 10\n6 10\n2 8\n3 4\n3 10\n3 8\n5 6\n5 5\n1 8\n","output":"1\n4\n12\n30\n32\n86\n56\n54\n60\n26\n82\n57\n9\n2\n61\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSumsOfSegments"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let mut prefix_sum = vec![0; n + 1];
    let mut triangle = vec![0; n + 1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + a[i - 1];
        triangle[i] = triangle[i - 1] + prefix_sum[i];
    }

    let query = |row: usize, lo: usize, hi: usize| {
        triangle[row + hi] - triangle[row + lo] - (hi - lo) as i64 * prefix_sum[row]
    };
    let mut prefix_row_sum = vec![0; n + 1];
    let mut prefix_cnt = vec![0; n + 1];
    for i in 0..n {
        prefix_row_sum[i + 1] = prefix_row_sum[i] + query(i, 0, n - i);
        prefix_cnt[i + 1] = prefix_cnt[i] + (n - i) as i64;
    }
    let q = input.read();
    for (l, r) in input.read_long_pair_vec(q) {
        let (Ok(row_lo) | Err(row_lo)) = prefix_cnt.binary_search(&l);
        let (Ok(row_hi) | Err(row_hi)) = prefix_cnt.binary_search(&r);
        let (row_lo, row_hi) = (row_lo - 1, row_hi - 1);
        let result = if row_lo == row_hi {
            let i = (l - prefix_cnt[row_lo]) as usize - 1;
            let j = (r - prefix_cnt[row_lo]) as usize;
            query(row_lo, i, j)
        } else {
            let i = (l - prefix_cnt[row_lo]) as usize - 1;
            let j = (r - prefix_cnt[row_hi]) as usize;
            query(row_lo, i, n - row_lo) + prefix_row_sum[row_hi as usize]
                - prefix_row_sum[row_lo as usize + 1]
                + query(row_hi, 0, j)
        };
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
