use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
          h:usize,
          w:usize,
    a:[Chars;h]
       }
       println!("-----minesweeper result!-----");
       let dx=[-1,-1,-1,0,1,1,1,0];
       let dy=[1,0,-1,-1,-1,0,1,1];
       for i in 0..h{
             for j in 0..w{
                if a[i][j]=='#'{
                      print!("#");
                }else{
                   let mut num =0;
                     for k in 0..8{
                        let nx=i as i32+dx[k];
                        let ny=j as i32+dy[k];
                        if nx>=0&&nx<h as i32&&ny>=0&&ny<w as i32{
                              if a[nx as usize][ny as usize]=='#'{
                                 num+=1;
                              }
                        }}
                      print!("{}",num);
                }
             }
             println!();
       }
}
