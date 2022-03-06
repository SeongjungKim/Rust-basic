use testing::List;

#[test]
fn test_list_insert_10k_item() {
    let mut list = List::new_empty();
    for _ in 0..1000 {
        list.append(100);
    }
    assert_eq!(list.length, 1000);
}