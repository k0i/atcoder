use proconio::input;

pub fn main() {
    input! {
       n:usize,
       a:[u16;n]
    }
    let mut deviation: Vec<u16> = Vec::new();
    for i in a {
        deviation.push(i / 400);
    }
    let superior = deviation
        .iter()
        .filter(|x| x >= &&8)
        .collect::<Vec<&u16>>()
        .len();
    deviation.dedup();
    deviation.sort();
deviation.retain(|x| x<&8);

    if superior == 0 {
        println!("{} {}", deviation.len(), deviation.len())
    } else {
        if deviation.len() ==0 {
            println!("{} {}", 1, superior)
        } else {
            println!("{} {}", deviation.len(), deviation.len() + superior)
        }
    }
}
