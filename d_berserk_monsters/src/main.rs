//{"name":"D. Berserk Monsters","group":"Codeforces - Educational Codeforces Round 161 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1922/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n3 4 7 5 10\n4 9 1 18 1\n2\n2 1\n1 3\n4\n1 1 2 4\n3 3 4 2\n","output":"3 1 0 0 0\n0 0\n1 1 1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBerserkMonsters"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = vec![0; n + 1];
    for i in 1..=n {
        a[i] = input.read_int();
    }
    let mut d = vec![0; n + 1];
    for i in 1..=n {
        d[i] = input.read_int();
    }

    let mut prev = vec![0; n + 1];
    let mut next = vec![0; n + 1];
    let mut candidates = vec![0; 0];

    for i in 1..=n {
        prev[i] = i - 1;
        next[i] = i + 1;
        candidates.push(i);
    }
    // make linked list circular
    next[n] = 0;
    prev[0] = n;
    next[0] = 1;

    for _round in 0..n {
        let mut p = next[0];
        let mut died_monsters = vec![0; 0];
        for &p in &candidates {
            if next[p] == p && prev[p] == p {
                // p is already deleted
                continue;
            }
            let damage_received = a[prev[p]] + a[next[p]];
            if damage_received > d[p] {
                died_monsters.push(p);
            }
        }
        out.print(died_monsters.len());
        out.print(" ");

        candidates.clear();
        for p in died_monsters {
            next[prev[p]] = next[p];
            prev[next[p]] = prev[p];
            if prev[p] != 0 {
                candidates.push(prev[p]);
            }
            prev[p] = p;
            if next[p] != 0 {
                candidates.push(next[p]);
            }
            next[p] = p;
        }

        candidates.sort();
        candidates.dedup();
    }

    out.print_line("");
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
