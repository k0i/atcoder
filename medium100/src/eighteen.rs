use proconio::input;
pub fn main() {
    input! {
       mut a:[i16;3],
       b:[i16;3],
       c:[i16;3]
    }
    let mut temp = vec![0, 0, 0];
    temp.append(&mut a);
    temp[1] = b[0] - temp[3];
    temp[2] = c[0] - temp[3];
    let mut res = "Yes";
if temp.iter().any(|x| x < &0) {res ="No"}
else{
    for i in 0..2 {
        if b[i] != temp[1] + temp[i + 3] {
            res = "No";
            break;
        }
        if c[i] != temp[2] + temp[i + 3] {
            res = "No";
            break;
        }
    }
   }
    println!("{}", res);
}
