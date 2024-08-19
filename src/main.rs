use crate::btrees::*;
pub mod btrees;



fn main() {
    let mut test = BPlusTree::new();
     
    for i in 1..10 {
        test.insert(KeyValue { key: i, value: 100 });
    }

    test.print_tree(0,1);

    // !!!! Be careful while using delete because if you delete too much keyvalue < 5 the tree will underflow!!!!!
    for i in 1..3 {
        test.delete(i);
    }
    

    test.print_tree(0, 1);
}
