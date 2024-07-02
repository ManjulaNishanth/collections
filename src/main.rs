mod list;

use crate::list::model::List;

fn main() {
    println!("Hello, world!");
    let list = List::new();
    println!("\n head : {list:?}");
    assert_eq!(list.head(), None);

    let list = list.push_front(5);
    println!("\n push_front : {list:?}");
    assert_eq!(list.head(), Some(&5));

    let list = list.push_front(6);
    println!("\n list : {list:?}");

    let list = list.tail();
    println!("\n tail : {list:?}");
    assert_eq!(list.head(), Some(&5));

    let list = list.push_front(6);
    let list = list.push_front(7);

    let iter = list.iter();
    println!("\n iter : {iter:?}");

    let mut list = list.push_back(7);
    println!("\n push_back : {list:?}");

    list.drop_list();
    println!("\n drop : {list:?}");
    // assert_eq!(list.head(), Some(&5));
}
