mod linkedlist;
use linkedlist::definitions::List as LL;

fn main(){
    let mut some_linkedlist: LL<i32> = LL::new();
    some_linkedlist.push_front( 1);
    some_linkedlist.push_back( 2);
    some_linkedlist.push_front(3);
    some_linkedlist.print();
}
