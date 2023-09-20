#[cfg(test)]
mod tests {
    use crate::pq_complete_heap::PqCompleteHeap;

    #[test]
    fn test_new() {
        let a: PqCompleteHeap<i32> = PqCompleteHeap::new(std::cmp::Ordering::Greater);
        assert_eq!(a.len(), 0);
        assert_eq!(a.get_ord(), std::cmp::Ordering::Greater);
        let b = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1], std::cmp::Ordering::Less);
        assert_eq!(b.len(), 5);
        assert_eq!(b.get_ord(), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_insert() {
        let mut b = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1], std::cmp::Ordering::Less);
        assert_eq!(b.get_max(), Some(&mut 1));
        b.insert(10);
        assert_eq!(b.get_max(), Some(&mut 1));
        b.insert(0);
        assert_eq!(b.get_max(), Some(&mut 0));
        b.insert(-1);
        assert_eq!(b.get_max(), Some(&mut -1));
        let mut a = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1], std::cmp::Ordering::Greater);
        assert_eq!(a.get_max(), Some(&mut 5));
        a.insert(0);
        assert_eq!(a.get_max(), Some(&mut 5));
        a.insert(10);
        assert_eq!(a.get_max(), Some(&mut 10));
        a.insert(20);
        assert_eq!(a.get_max(), Some(&mut 20));
    }

    #[test]
    fn test_get_max() {
        let mut a = PqCompleteHeap::new(std::cmp::Ordering::Less);
        assert_eq!(a.get_max(), None);
        a.insert(1);
        a.insert(2);
        assert_eq!(a.get_max(), Some(&mut 1));
    }

    #[test]
    fn test_del_max() {
        let mut b = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1], std::cmp::Ordering::Less);
        for i in 1..=5 {
            assert_eq!(b.del_max(), Some(i));
        }
        assert_eq!(b.del_max(), None);
        let mut a = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1], std::cmp::Ordering::Greater);
        for i in (1..=5).rev() {
            assert_eq!(a.del_max(), Some(i));
        }
        assert_eq!(a.del_max(), None);
    }
}
