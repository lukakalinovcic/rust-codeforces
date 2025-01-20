//{"name":"C. Increasing Sequence with Fixed OR","group":"Codeforces - Codeforces Round 958 (Div. 2)","url":"https://codeforces.com/contest/1988/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n3\n14\n23\n","output":"1\n1\n3\n1 2 3\n4\n4 10 12 14\n5\n7 18 21 22 23\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIncreasingSequenceWithFixedOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let mut bits = vec![];
    for i in 0..=60 {
        if ((n >> i) & 1) == 1 {
            bits.push(i);
        }
    }
    if bits.len() == 1 {
        out.print_line(1);
        out.print_line(n);
    } else {
        let mut nums = vec![];
        for i in 0..bits.len() {
            let mut x = 0;
            for j in 0..bits.len() {
                if j != bits.len() - i - 1 {
                    x |= (1 as i64) << bits[j];
                }
            }
            nums.push(x);
        }
        nums.push(n);
        for i in 1..nums.len() {
            if (nums[i] | nums[i - 1]) != n {
                eprintln!("Check failed! {n} {nums:?}");
            }
        }
        out.print_line(nums.len());
        out.print_iter(nums.into_iter());
        out.print_line("");
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
