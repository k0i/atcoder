use proconio::input;

pub fn main(){
input!{
   n:usize,
   d:u64,
   x:u64,
   a:[u64;n]
};
let mut res:u64 =x;
for i in a{
if i>d{
res +=1
}else{
   res += if d%i==0{d/i}else{d/i+1};
}
}

println!("{}",res);
}