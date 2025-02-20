//{"name":"B. Set of Strangers","group":"Codeforces - Educational Codeforces Round 174 (Rated for Div. 2)","url":"https://codeforces.com/contest/2069/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 1\n1\n3 3\n1 2 1\n2 3 2\n1 3 1\n1 6\n5 4 5 4 4 5\n3 4\n1 4 2 2\n1 4 3 5\n6 6 3 5\n","output":"0\n2\n1\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSetOfStrangers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let a = (0..n).map(|_| input.read_int_vec(m)).collect::<Vec<_>>();
    let mut color_status = vec![0; n * m + 1];
    for r in 0..n as i32 {
        for c in 0..m as i32 {
            let color1 = a[r as usize][c as usize] as usize;
            color_status[color1] = color_status[color1].max(1);
            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let rr = r + dr;
                let cc = c + dc;
                if rr < 0 || rr >= n as i32 || cc < 0 || cc >= m as i32 {
                    continue;
                }
                let color2 = a[rr as usize][cc as usize] as usize;
                if color1 == color2 {
                    color_status[color1] = color_status[color1].max(2);
                }
            }
        }
    }
    let mut num1 = 0;
    let mut num2 = 0;
    for color in 1..=n * m {
        match color_status[color] {
            1 => num1 += 1,
            2 => num2 += 1,
            _ => {}
        }
    }
    if num2 == 0 {
        out.print_line(num1 - 1);
    } else {
        out.print_line((num2 - 1) * 2 + num1);
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
