//{"name":"E. Beautiful Array","group":"Codeforces - Codeforces Round 954 (Div. 3)","url":"https://codeforces.com/contest/1986/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"11\n1 1000000000\n1\n2 1\n624323799 708290323\n3 1\n3 2 1\n4 1\n7 1 5 3\n5 1\n11 2 15 7 10\n7 1\n1 8 2 16 8 16 31\n13 1\n2 1 1 3 3 11 12 22 45 777 777 1500 74\n10 2\n1 2 1 2 1 2 1 2 1 2\n11 2\n1 2 1 2 1 2 1 2 1 2 1\n13 3\n2 3 9 14 17 10 22 20 18 30 1 4 28\n5 1\n2 3 5 3 5\n","output":"0\n83966524\n1\n4\n6\n1\n48\n-1\n0\n14\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBeautifulArray"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    let mut classes = HashMap::<i32, Vec<i32>>::new();
    for a in a {
        classes.entry(a % k).or_default().push(a);
    }
    let mut num_odd = 0;
    let mut cost: i64 = 0;
    for mut class in classes.into_values() {
        class.sort();
        let m = class.len();
        if m % 2 == 1 {
            num_odd += 1;
            cost += odd_cost(&class, k);
        } else {
            for i in (0..m).step_by(2) {
                cost += ((class[i + 1] - class[i]) / k) as i64;
            }
        }
    }
    if n % 2 == 0 && num_odd != 0 {
        out.print_line(-1);
    } else if n % 2 == 1 && num_odd != 1 {
        out.print_line(-1);
    } else {
        out.print_line(cost);
    }
}

fn odd_cost(a: &[i32], k: i32) -> i64 {
    let m = a.len();
    let mut prefix = vec![0; m];
    for i in (0..m - 1).step_by(2) {
        prefix[i + 2] = prefix[i] + ((a[i + 1] - a[i]) / k) as i64;
    }
    let mut result: i64 = 1000000000000000000;
    let mut suffix: i64 = 0;
    for i in (0..m).step_by(2).rev() {
        if i + 2 < m {
            suffix += ((a[i + 2] - a[i + 1]) / k) as i64;
        }
        result = result.min(suffix + prefix[i]);
    }
    result
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
