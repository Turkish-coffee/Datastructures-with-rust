#[cfg(test)]
mod tests {
    use crate::linkedlist::definitions::List;

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop_back() {
        let mut list = List::new();
        list.print();
        list.push_back(1);
        list.print();
        list.push_back(2);
        list.print();
        list.push_back(3);
        list.print();
        assert_eq!(list.pop_back(), Some(3));
        list.print();
        assert_eq!(list.pop_back(), Some(2));
        list.print();
        assert_eq!(list.pop_back(), Some(1));
        list.print();
        assert_eq!(list.pop_back(), None);
        list.print();
    }
    #[test]
    fn test_push_pop_front() {
        let mut list = List::new();
        list.print();
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();

        assert_eq!(list.pop_front(), Some(3));
        list.print();
        assert_eq!(list.pop_front(), Some(2));
        list.print();
        assert_eq!(list.pop_front(), Some(1));
        list.print();
        assert_eq!(list.pop_front(), None);
        list.print();
    }

    #[test]
    fn test_pop_by_key() {
        let mut list = List::new();
        list.pop_by_key(1);
        list.print();  
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();  
        list.push_front(4);
        list.print();
        list.push_front(5);
        list.print();
        list.pop_by_key(5);
        list.print();
        list.pop_by_key(1);
        list.print();
        list.pop_by_key(3);
        list.print();
        list.pop_by_key(12345);
        list.print();
    }
    #[test]
    fn test_pop_by_position() {
        let mut list = List::new();
        list.pop_by_position(2);
        list.print();  
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();  
        list.push_front(4);
        list.print();
        list.push_front(5);
        list.print();
        list.pop_by_position(2);
        list.print();
        list.pop_by_position(2);
        list.print();
        list.pop_by_position(2);
        list.print();
        list.pop_by_position(2);
        list.print();
    }
    #[test]
    fn test_update_by_key() {
        let mut list = List::new();
        list.update_by_key(1, 2);
        list.print();  
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();  
        list.push_front(4);
        list.print();
        list.push_front(5);
        list.print();
        list.update_by_key(5, 10);
        list.print();
        list.update_by_key(1, 10);
        list.print();
        list.update_by_key(3, 10);
        list.print();
        list.update_by_key(12345, 10);
        list.print();
    }
    #[test]
    fn test_update_by_position() {
        let mut list = List::new();
        list.update_by_position(2, 2);
        list.print();  
        list.push_front(1);
        list.print();
        list.push_front(2);
        list.print();
        list.push_front(3);
        list.print();  
        list.push_front(4);
        list.print();
        list.push_front(5);
        list.print();
        list.update_by_position(0, 10);
        list.print();
        list.update_by_position(1, 10);
        list.print();
        list.update_by_position(4, 10);
        list.print();
        list.update_by_position(20, 10);
        list.print();
    }
}
}
