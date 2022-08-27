use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut a:(f64,f64),
    mut b:(f64,f64),
    mut c:(f64,f64),
    mut d:(f64,f64),
        }
    b = (b.0 - a.0, b.1 - a.1);
    d = (d.0 - a.0, d.1 - a.1);

    let th = std::f64::consts::PI * 2.0;
    if (b.1.atan2(b.0) + th) % th + (d.1.atan2(d.0) + th) % th > std::f64::consts::PI {
        println!("No");
        return;
    }
    b = (b.0 + a.0, b.1 + a.1);
    d = (d.0 + a.0, d.1 + a.1);
    // when b
    a = (a.0 - b.0, a.1 - b.1);
    c = (c.0 - b.0, c.1 - b.1);
    if (c.1.atan2(c.0) + th) % th + (a.1.atan2(a.0) + th) % th > std::f64::consts::PI {
        println!("No");
        return;
    }

    a = (a.0 + b.0, a.1 + b.1);
    c = (c.0 + b.0, c.1 + b.1);
    // when c
    d = (d.0 - c.0, d.1 - c.1);
    b = (b.0 - c.0, b.1 - c.1);
    if (b.1.atan2(b.0) + th) % th + (d.1.atan2(d.0) + th) % th > std::f64::consts::PI {
        println!("No");
        return;
    }
    d = (d.0 + c.0, d.1 + c.1);
    b = (b.0 + c.0, b.1 + c.1);
    // when d
    a = (a.0 - d.0, a.1 - d.1);
    c = (c.0 - d.0, c.1 - d.1);
    if (c.1.atan2(c.0) + th) % th + (a.1.atan2(a.0) + th) % th > std::f64::consts::PI {
        println!("No");
        return;
    }
    println!("Yes");
}
