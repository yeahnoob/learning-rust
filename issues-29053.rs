fn issue_code01() {
    let x: &'static str = "1";
 
    {
        let y = "y".to_string();
        let ref mut x = &*x;
        *x = &*y;
        println!("inner {:?}", x);
    }

    println!("{:?}", x);
}

fn issue_code02() {
    let ref mut y = "x";
    *y = "y";

    println!("{:?}", y);
}

fn issue_code03() {
    let x: &'static str = "x";

    {
        let y = &mut &*x;
        *y = "y";
    }

    println!("{:?}", x);
}

fn main() {
    issue_code01();
    //issue_code02();
    //issue_code03();
}
