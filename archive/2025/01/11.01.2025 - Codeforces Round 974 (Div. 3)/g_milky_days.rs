//{"name":"G. Milky Days","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 1 3\n1 5\n2 3 3\n1 5\n2 7\n4 5 2\n1 9\n2 6\n4 9\n5 6\n5 2 4\n4 7\n5 3\n7 1\n11 2\n12 1\n4 1 3\n5 10\n9 4\n14 8\n15 3\n5 5 5\n8 9\n10 7\n16 10\n21 5\n28 9\n","output":"3\n3\n4\n5\n10\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMilkyDays"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let need_milk = input.read_long();
    let days_to_spoil = input.read_long();
    let mut new_milk = input.read_long_pair_vec(n);
    new_milk.push((INF, 0));

    let mut day = 0;
    let mut next_milk = 0;
    let mut milk: VecDeque<(i64, i64)> = VecDeque::new();
    let mut milk_satisfaction_days = 0;
    while day != INF {
        let mut days_until_next_event = INF;
        // More milk arrives.
        days_until_next_event = days_until_next_event.min(new_milk[next_milk].0 - day);
        if !milk.is_empty() {
            // Old milk spoils.
            days_until_next_event =
                days_until_next_event.min(milk.back().unwrap().0 + days_to_spoil - day);
            // Fresh milk is running low.
            days_until_next_event =
                days_until_next_event.min((milk.front().unwrap().1 - 1) / need_milk);
        }

        if days_until_next_event == 0 {
            if new_milk[next_milk].0 == day {
                milk.push_front(new_milk[next_milk]);
                next_milk += 1;
            }
            if !milk.is_empty() && milk.back().unwrap().0 + days_to_spoil == day {
                milk.pop_back();
            }
            let mut need_milk = need_milk;
            while !milk.is_empty() && need_milk > 0 {
                if milk.front().unwrap().1 > need_milk {
                    milk.front_mut().unwrap().1 -= need_milk;
                    need_milk = 0;
                } else {
                    need_milk -= milk.front().unwrap().1;
                    milk.pop_front();
                }
            }
            if need_milk == 0 {
                milk_satisfaction_days += 1;
            }
            day += 1;
        } else {
            if !milk.is_empty() {
                milk.front_mut().unwrap().1 -= days_until_next_event * need_milk;
                milk_satisfaction_days += days_until_next_event;
            }
            day += days_until_next_event
        }
    }
    out.print_line(milk_satisfaction_days);
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
