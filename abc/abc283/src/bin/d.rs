use im_rc::HashSet;

fn main() {
    proconio::input! {
    s:proconio::marker::Chars
        }
    let mut a = std::collections::HashSet::new();
    let mut stacks = vec![HashSet::new()];
    for i in 0..s.len() {
        match s[i] {
            '(' => {
                stacks.push(HashSet::new());
                continue;
            }
            ')' => {
                let stack = stacks.pop().unwrap();
                for x in stack.iter() {
                    a.remove(x);
                }
            }
            _ => {
                if a.contains(&s[i]) {
                    println!("No");
                    return;
                }
                a.insert(s[i]);
                let mut stack = stacks.pop().unwrap();
                stack.insert(s[i]);
                stacks.push(stack);
            }
        }
    }
    println!("Yes");
}
