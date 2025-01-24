//{"name":"E2. Nene vs. Monsters (Hard Version)","group":"Codeforces - Codeforces Round 939 (Div. 2)","url":"https://codeforces.com/contest/1956/problem/E2","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\n2 5 3\n2\n0 0\n4\n1 5 7 2\n4\n4 2 1 2\n13\n1 1 4 5 1 4 1 9 1 9 8 1 0\n","output":"1\n1\n0\n\n1\n1\n2\n1 3\n6\n1 3 6 8 10 12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E2NeneVsMonstersHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);

    let mut i = 0;
    loop {
        let j = if i + 1 < n { i + 1 } else { 0 };
        let x = a[i];
        let y = a[j];
        a[j] = (y - x).max(0);
        i = j;
        if a[j] == 0 {
            break;
        }
    }
    loop {
        let mut group = 0;
        let mut max_group = 0;
        for _ in 0..n {
            let j = if i + 1 < n { i + 1 } else { 0 };
            let x = a[i];
            let y = a[j];
            a[j] = (y - x).max(0);
            i = j;
            if a[j] == 0 {
                max_group = max_group.max(group);
                group = 0;
            } else {
                group += 1;
            }
        }
        assert_eq!(group, 0);
        if max_group <= 3 {
            break;
        }
    }

    let mut result = vec![];
    let mut process_group = |group: &Vec<(usize, i32)>| {
        if group.len() == 0 {
            return;
        }
        result.push(group[0].0 + 1);
        if group.len() == 3 {
            let (x, y, z) = (group[0].1, group[1].1, group[2].1);
            let y_dies_in = (y + x - 1) / x;
            let y_attacks = y_dies_in - 1;
            let y_first_attack = y - x;
            let y_last_attack = y - y_attacks * x;
            let y_dmg_to_z = (y_first_attack + y_last_attack) as i64 * y_attacks as i64 / 2;
            if y_dmg_to_z < z as i64 {
                result.push(group[2].0 + 1);
            }
        }
    };
    let mut group: Vec<(usize, i32)> = vec![];
    for _ in 0..n {
        let j = if i + 1 < n { i + 1 } else { 0 };
        let x = a[i];
        let y = a[j];
        a[j] = (y - x).max(0);
        i = j;
        if a[j] == 0 {
            process_group(&group);
            group.clear();
        } else {
            group.push((j, a[j]));
        }
    }
    assert_eq!(group.len(), 0);
    result.sort();
    out.print_line(result.len());
    out.print_iter(result.into_iter());
    out.print_line("");
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
