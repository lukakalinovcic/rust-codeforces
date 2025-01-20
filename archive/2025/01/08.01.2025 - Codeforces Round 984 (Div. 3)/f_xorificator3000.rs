//{"name":"F. XORificator 3000","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 3 1 0\n2 28 3 7\n15 43 1 0\n57 2007 1 0\n1010 1993 2 2\n1 1000000000 30 1543\n","output":"2\n2\n13\n0\n4\n1000000519\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FXORificator3000"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn mask(i: i32) -> i64 {
    (1 << i) - 1
}

fn rec(pos: i32, lo: i64, l: i64, r: i64, i: i32, k: i64) -> i64 {
    let hi = lo | mask(pos);
    if lo > r || hi < l {
        return 0;
    }
    if pos <= 2 {
        let mut result = 0;
        for x in lo..=lo | mask(pos) {
            if x >= l && x <= r && (x & mask(i) != k) {
                result ^= x;
            }
        }
        return result;
    }
    if lo >= l && hi <= r {
        let result = if pos <= i && (lo >> pos) & mask(i - pos) == (k >> pos) {
            lo | k
        } else if pos == i + 1 {
            1 << (pos - 1)
        } else {
            0
        };
        return result;
    }
    rec(pos - 1, lo, l, r, i, k) ^ rec(pos - 1, lo | (1 << (pos - 1)), l, r, i, k)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_long();
    let r = input.read_long();
    let i = input.read_int();
    let k = input.read_long();
    out.print_line(rec(60, 0, l, r, i, k));
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
