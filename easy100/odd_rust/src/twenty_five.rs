use itertools::Itertools;
use proconio::input;

pub fn main(){
 input!{
a:usize,
p:[usize;a],
q:[usize;a],
 };

 let mut n:Vec<Vec<usize>> =Vec::new();

 for i in (1..a+1).permutations(a){
     n.push(i);
 }
println!("{}",(((n.iter().position(|x| x==&p).unwrap()+1) as i32)-((n.iter().position(|x| x==&q).unwrap()+1) as i32)).abs());
}
