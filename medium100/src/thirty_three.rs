use proconio::input;
use proconio::marker::Chars;
pub fn main() {
input!{
s:Chars
}

let mut ans = vec![];
for i in 0..s.len() {
    if s[i] == 'B' {
        if ans.len()==0{
           continue;
        }else{
            ans.pop();
        }
    }else{
       ans.push(s[i]);
    }
}
println!("{}",ans.iter().collect::<String>());
}