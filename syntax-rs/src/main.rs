// 4.1 Variable Bindings
fn syntax_01() {
    let (x,y) = (1,2);
    let mut z = 0;
    z += 10;
    println!("x, y, z = ");
    for i in [x, y, z].iter() {
        print!("{:?} ", i);
    }
    println!("");
}

// 4.2 Functions


fn main() {
    println!("########## Hell, Wall! ##########");
    syntax_01();
}
