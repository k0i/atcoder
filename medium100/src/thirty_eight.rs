use std::collections::HashMap;

use proconio::input;
pub fn main() {
input!{
n:usize,
d:[u64;n],
m:usize,
t:[u64;m],
}
let mut d_hash = HashMap::new();
let mut t_hash = HashMap::new();
for i in 0 ..n{
let s =d_hash.entry(d[i]).or_insert(0);
   *s+=1;
}
for i in 0 ..m{
let s =t_hash.entry(t[i]).or_insert(0);
   *s+=1;
}
let mut res ="";
for (key,value)in &t_hash{
   let s =d_hash.entry(*key).or_insert(0);
   if *s>=*value{
   continue;
   }else{
   res = "NO";
   break;
   }
}
if res == ""{
   res = "YES";
}
println!("{}",res);
}