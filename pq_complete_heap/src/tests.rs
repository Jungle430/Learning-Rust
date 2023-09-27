#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use crate::pq_complete_heap::PqCompleteHeap;

    #[test]
    fn test_new() {
        let a: PqCompleteHeap<i32> = PqCompleteHeap::new();
        assert_eq!(a.len(), 0);
        let b = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1]);
        assert_eq!(b.len(), 5);
    }

    #[test]
    fn test_push() {
        let mut b = PqCompleteHeap::new_with_vec(
            vec![4, 2, 3, 5, 1]
                .into_iter()
                .map(|x| Reverse(x))
                .collect(),
        );
        assert_eq!(b.peek_mut(), Some(&mut Reverse(1)));
        b.push(Reverse(10));
        assert_eq!(b.peek_mut(), Some(&mut Reverse(1)));
        b.push(Reverse(0));
        assert_eq!(b.peek_mut(), Some(&mut Reverse(0)));
        b.push(Reverse(-1));
        assert_eq!(b.peek_mut(), Some(&mut Reverse(-1)));
        let mut a = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1]);
        assert_eq!(a.peek_mut(), Some(&mut 5));
        a.push(0);
        assert_eq!(a.peek_mut(), Some(&mut 5));
        a.push(10);
        assert_eq!(a.peek_mut(), Some(&mut 10));
        a.push(20);
        assert_eq!(a.peek_mut(), Some(&mut 20));
    }

    #[test]
    fn test_peek() {
        let mut a = PqCompleteHeap::new();
        assert_eq!(a.peek_mut(), None);
        a.push(Reverse(1));
        a.push(Reverse(2));
        assert_eq!(a.peek_mut(), Some(&mut Reverse(1)));
    }

    #[test]
    fn test_pop() {
        let mut b = PqCompleteHeap::new_with_vec(
            vec![4, 2, 3, 5, 1]
                .into_iter()
                .map(|x| Reverse(x))
                .collect(),
        );
        for i in 1..=5 {
            assert_eq!(b.pop(), Some(Reverse(i)));
        }
        assert_eq!(b.pop(), None);
        let mut a = PqCompleteHeap::new_with_vec(vec![4, 2, 3, 5, 1]);
        for i in (1..=5).rev() {
            assert_eq!(a.pop(), Some(i));
        }
        assert_eq!(a.pop(), None);
    }
}
