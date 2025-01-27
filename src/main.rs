mod linkedlist;
use linkedlist::definitions::List as LL;

#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person {
            name,
            age,
        }
    }
    
}

fn main(){
    let p1 = Person::new("John".to_string(), 23);
    let p2 = Person::new("Jane".to_string(), 25);
    let mut some_linkedlist: LL<Person> = LL::new();
    some_linkedlist.push_front( p1);
    some_linkedlist.print();
    some_linkedlist.push_back( p2);
    some_linkedlist.print();
    some_linkedlist.pop_by_key(Person::new("John".to_string(), 24));
    some_linkedlist.print();
    some_linkedlist.pop_by_key(Person::new("John".to_string(), 23));
    some_linkedlist.print();
}
