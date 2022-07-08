use proconio::input;
pub fn main(){
   input!{
      r:i64,
      g:i64,
      b:i64,
      n:i64
   }
   let mut ans = 0;
      for i in 0..=n{
         for j in 0..=n{
            let temp = n-(i*r+j*g);
            if temp < 0{continue;}
              if temp %b == 0{
                 ans += 1;
              }
            
         }
      } 
   print!("{}",ans);
}