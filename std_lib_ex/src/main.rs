// Examples of rust standard library
//

fn btree() {
    use std::collections::BTreeMap;

    let mut map = BTreeMap::new();
    map.insert(1, "a");
    assert_eq!(map.contains_key(&1), true);
    assert_eq!(map.contains_key(&2), false);
    
    let n = 1;
    match map.get(&n) {
        Some(val) => println!("\tGet the value of `{:?}` in map : ... {:?}", n, val),
        _ => panic!("\n!!!Don't have any value of {:?} in `map`!!!", n)
    };

}

fn main() {
    println!("########## Hell, Wall! ##########");
    println!("`use std::collections::BTreeMap`");
    btree();
}
