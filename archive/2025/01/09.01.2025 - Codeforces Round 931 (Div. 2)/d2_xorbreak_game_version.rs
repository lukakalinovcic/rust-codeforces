//{"name":"D2. XOR Break — Game Version","group":"Codeforces - Codeforces Round 931 (Div. 2)","url":"https://codeforces.com/contest/1934/problem/D2","interactive":true,"timeLimit":3000,"tests":[{"input":"4\n1\n\n0 0\n3\n\n\n0 0\n13\n\n\n3 4\n\n0 0\n777777770001\n\n\n0 0\n","output":"\nsecond\n\n\nfirst\n2 1\n\n\nfirst\n10 7\n\n1 2\n\n\nfirst\n777777770000 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2XORBreakGameVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn cnt_bits(x: i64) -> i32 {
    let mut bits = 0;
    for i in 0..63 {
        if ((x >> i) & 1) == 1 {
            bits += 1;
        }
    }
    bits
}

fn winning(x: i64) -> bool {
    cnt_bits(x) % 2 == 0
}

fn high_bit(x: i64) -> i64 {
    for i in (0..63).rev() {
        if ((x >> i) & 1) == 1 {
            return 1 << i;
        }
    }
    panic!("No bits in num");
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut x = input.read_long();
    let mut my_turn = if winning(x) {
        out.print_line("first");
        out.flush();
        true
    } else {
        out.print_line("second");
        out.flush();
        false
    };

    loop {
        if my_turn {
            let y = high_bit(x);
            out.print_line(format!("{} {}", y, y ^ x));
            out.flush();
        } else {
            let (p1, p2) = (input.read_long(), input.read_long());
            if (p1, p2) == (0, 0) {
                return;
            }
            x = if winning(p1) { p1 } else { p2 };
        }
        my_turn = !my_turn;
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);
    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);
    run(input, output);
}

//END MAIN
