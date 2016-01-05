/// ## 4.1 Variable Bindings
fn syntax_01() {
    let (x,y) = (1,2);
    let mut z = 0;
    z += 10;
    println!("x, y, z : ");
    for i in [x, y, z].iter() {
        print!("{:?} ", i);
    }
    print!("\n\n");
}

/// ## 4.2 Functions


/// ## 4.5 Iterators

/// ### Iterators over vectors directly
fn syntax_05() {
    let nums = vec![1, 2, 3];
    println!("Iterate over `vec![1, 2, 3]` : ");
    for num in &nums {
        print!("{} ", num);
    }
    print!("\n\n");
}

/// ### Another detail
/// `println!(  )` handles the dereferencing for us
fn auto_deref_println() {
    let nums = vec![1, 2, 3];
    for num in &nums {
        println!("{}", *num);
    }
}

/// ### Consumers on an iterators
/// `collect()`
/// The type of things collected must be declared,
/// so `let cydb = (1..101).collect();` will get an error from the compiler.
fn iter_consumers() {
    let cydb = (1..101).collect::<Vec<i32>>();
    for num in cydb.iter() {
        println!("{}", num);
    }
}

fn main() {
    println!("########## Hell, Wall! ##########");

    println!(" ## 4.1 Variable Bindings");
    syntax_01();
    
    println!(" ## 4.5 Iterators");
    syntax_05();

}
