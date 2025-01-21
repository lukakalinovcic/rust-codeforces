//{"name":"F. Unfair Game","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1 1 1 0\n1 0 1 2\n2 2 2 0\n3 3 2 0\n0 9 9 9\n","output":"1\n1\n3\n3\n12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FUnfairGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let c1 = input.read_int();
    let c2 = input.read_int();
    let c3 = input.read_int();
    let c4 = input.read_int();
    out.print_line(doit(0, c1, c2, c3, c4));
}

fn doit(depth: i32, c1: i32, c2: i32, c3: i32, c4: i32) -> i32 {
    let mut result = 0;
    if c1 == 0 && c2 == 0 && c3 == 0 && c4 == 0 {
        return 0;
    }
    if c4 > 0 && depth > 3 {
        return 0;
    }
    if depth > 6 {
        return 0;
    }
    let mut can_make_long_move = false;
    if c1 % 2 == c2 % 2 && c2 % 2 == c3 % 2 {
        if c4 != 0 {
            result = result.max(doit(depth + 1, c1, c2, c3, 0) + c4 / 2);
            can_make_long_move = true;
        } else {
            if c1 > 1 {
                result = result.max(doit(depth + 1, 1, c2, c3, 0) + c1 / 2);
                can_make_long_move = true;
            }
            if c2 > 1 {
                result = result.max(doit(depth + 1, c1, 1, c3, 0) + c2 / 2);
                can_make_long_move = true;
            }
            if c3 > 1 {
                result = result.max(doit(depth + 1, c1, c2, 1, 0) + c3 / 2);
                can_make_long_move = true;
            }
        }
    }
    if can_make_long_move {
        return result;
    }
    let add = if c4 % 2 == 0 && c1 % 2 == c2 % 2 && c2 % 2 == c3 % 2 {
        1
    } else {
        0
    };
    if c1 > 0 {
        result = result.max(doit(depth + 1, c1 - 1, c2, c3, c4) + add);
    };
    if c2 > 0 {
        result = result.max(doit(depth + 1, c1, c2 - 1, c3, c4) + add);
    };
    if c3 > 0 {
        result = result.max(doit(depth + 1, c1, c2, c3 - 1, c4) + add);
    };
    if c4 > 0 {
        result = result.max(doit(depth + 1, c1, c2, c3, c4 - 1) + add);
    };
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

fn doit_slower(depth: i32, c1: i32, c2: i32, c3: i32, c4: i32) -> i32 {
    let mut result = 0;
    if c1 == 0 && c2 == 0 && c3 == 0 && c4 == 0 {
        return 0;
    }
    if depth > 10 {
        return 0;
    }
    if c1 % 2 == c2 % 2 && c2 % 2 == c3 % 2 {
        if c4 != 0 {
            result = result.max(doit_slower(depth + 1, c1, c2, c3, 0) + c4 / 2);
        } else {
            if c1 > 1 {
                result = result.max(doit_slower(depth + 1, 1, c2, c3, 0) + c1 / 2);
            }
            if c2 > 1 {
                result = result.max(doit_slower(depth + 1, c1, 1, c3, 0) + c2 / 2);
            }
            if c3 > 1 {
                result = result.max(doit_slower(depth + 1, c1, c2, 1, 0) + c3 / 2);
            }
        }
    }
    let add = if c4 % 2 == 0 && c1 % 2 == c2 % 2 && c2 % 2 == c3 % 2 {
        1
    } else {
        0
    };
    if c1 > 0 {
        result = result.max(doit_slower(depth + 1, c1 - 1, c2, c3, c4) + add);
    };
    if c2 > 0 {
        result = result.max(doit_slower(depth + 1, c1, c2 - 1, c3, c4) + add);
    };
    if c3 > 0 {
        result = result.max(doit_slower(depth + 1, c1, c2, c3 - 1, c4) + add);
    };
    if c4 > 0 {
        result = result.max(doit_slower(depth + 1, c1, c2, c3, c4 - 1) + add);
    };
    result
}

fn main() {
    tester::run_tests();
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let c1 = rng.gen_range(0..=200);
        let c2 = rng.gen_range(0..=200);
        let c3 = rng.gen_range(0..=200);
        let c4 = rng.gen_range(0..=200);
        // let c1 = rng.gen_range(0..=5);
        // let c2 = rng.gen_range(0..=4);
        // let c3 = rng.gen_range(0..=4);
        // let c4 = rng.gen_range(0..=4);
        let x = doit(0, c1, c2, c3, c4);
        //        let y = doit_slower(0, c1, c2, c3, c4);
        // if x != y {
        //     eprintln!("Mismatch for {c1} {c2} {c3} {c4}   {x} != {y}");
        //     break;
        // }
    }
}
//END MAIN
