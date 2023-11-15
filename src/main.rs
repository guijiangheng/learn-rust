use crate::link_list::LinkList;

mod link_list;

fn main() {
    let mut a = LinkList::new();
    a.push_front(1);
    a.push_front(2);
    a.push_front(3);
    assert_eq!(format!("{:?}", a), "3 -> 2 -> 1");

    for x in &mut a {
        *x += 1;
    }

    assert_eq!(format!("{:?}", a), "4 -> 3 -> 2");
}
