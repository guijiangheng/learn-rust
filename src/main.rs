use crate::hash_set::HashSet;

mod hash_map;
mod hash_set;

fn main() {
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("{:?}", set);
}
