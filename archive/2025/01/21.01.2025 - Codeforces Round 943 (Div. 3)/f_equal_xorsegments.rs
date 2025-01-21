//{"name":"F. Equal XOR Segments","group":"Codeforces - Codeforces Round 943 (Div. 3)","url":"https://codeforces.com/contest/1968/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"4\n5 5\n1 1 2 3 0\n1 5\n2 4\n3 5\n1 3\n3 4\n5 5\n1 2 3 4 5\n1 5\n2 4\n3 5\n1 3\n2 3\n7 4\n12 9 10 9 10 11 9\n1 5\n1 7\n2 6\n2 7\n11 4\n0 0 1 0 0 1 0 1 1 0 1\n1 2\n2 5\n6 9\n7 11\n","output":"YES\nYES\nNO\nNO\nNO\n\nYES\nNO\nNO\nYES\nNO\n\nNO\nNO\nNO\nNO\n\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FEqualXORSegments"}}}

use std::collections::hash_map::Entry;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let a = input.read_int_vec(n);
    let mut prefix_xor = vec![0; n + 1];
    for i in 0..n {
        prefix_xor[i + 1] = prefix_xor[i] ^ a[i];
    }
    let mut relabel = HashMap::new();
    let mut label = vec![0; n + 1];
    let mut label_positions = vec![vec![]; n + 1];
    for i in 0..=n {
        let next_label = relabel.len() as i32;
        label[i] = match relabel.entry(&prefix_xor[i]) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                e.insert(next_label);
                next_label
            }
        };
        label_positions[label[i] as usize].push(i);
    }
    for (l, r) in input.read_int_pair_vec(q) {
        let label_l = label[l as usize - 1];
        let label_r = label[r as usize];
        let (Ok(i) | Err(i)) = label_positions[label_l as usize].binary_search(&(r as usize + 1));
        let (Ok(j) | Err(j)) = label_positions[label_r as usize].binary_search(&(l as usize - 1));
        let pl = label_positions[label_l as usize][i - 1];
        let pr = label_positions[label_r as usize][j];
        if pl >= pr {
            out.print_line("YES");
        } else {
            out.print_line("NO");
        }
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
