//{"name":"E. Do You Love Your Hero and His Two-Hit Multi-Target Attacks?","group":"Codeforces - Codeforces Round 1006 (Div. 3)","url":"https://codeforces.com/contest/2072/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n0\n2\n5\n","output":"6\n69 52\n4 20\n789 9308706\n1337 1337\n-1234 -5678\n23456178 707\n10\n-236 -346262358\n273568 6435267\n2365437 31441367\n246574 -45642372\n-236 56\n4743623 -192892\n10408080 -8173135\n-237415357 31441367\n-78125638 278\n56 143231\n5\n1 1\n2 1\n1 5\n3 5\n1 10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EDoYouLoveYourHeroAndHisTwoHitMultiTargetAttacks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut k = input.read_int();
    let (mut x, mut y) = (0, 0);
    let (mut dx, mut dy) = (1, 0);
    let mut pts = vec![(x, y)];
    while k > 0 {
        let mut t = 1;
        while (t + 1) * (t + 2) / 2 <= k {
            t += 1;
        }
        k -= t * (t + 1) / 2;

        for _ in 0..t {
            x += dx;
            y += dy;
            pts.push((x, y));
        }

        (dx, dy) = (dy, dx);
    }
    out.print_line(pts.len());
    for p in pts {
        out.print_line(p);
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
