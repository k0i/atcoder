use proconio::input;
pub fn main() {
    input! {
        a:[u32;3],
        b:[u32;3],
        c:[u32;3],
        n:u32,
        p:[u32;n]
    }

    let mut at = [false; 3];
    let mut bt = [false; 3];
    let mut ct = [false; 3];
    for i in 0..n as usize {
        if a.iter().position(|x| x == &p[i]) != None {
            at[a.iter().position(|x| x == &p[i]).unwrap()] = true
        }
        if b.iter().position(|x| x == &p[i]) != None {
            bt[b.iter().position(|x| x == &p[i]).unwrap()] = true
        }
        if c.iter().position(|x| x == &p[i]) != None {
            ct[c.iter().position(|x| x == &p[i]).unwrap()] = true
        }
    }

    let mut res: bool = false;
    for i in 0..=2 {
        if at[i] == true && bt[i] == true && ct[i] == true {
            res = true;
            break;
        } else if bt[1] == true
            && (at[0] == true && ct[2] == true || at[2] == true && ct[0] == true)
        {
            res = true;
            break;
        } else if (at[0] == true && at[1] == true && at[2] == true)
            || (bt[0] == true && bt[1] == true && bt[2] == true)
            || (ct[0] == true && ct[1] == true && ct[2] == true)
        {
            res = true;
        }
    }
    if res == true {
        println!("Yes")
    } else {
        println!("No")
    }
}
