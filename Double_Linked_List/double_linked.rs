use std::fmt::Debug;
use std::{marker::PhantomData, ptr::NonNull};

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            next: None,
            prev: None,
        }
    }

    fn into_val(self: Box<Self>) -> T {
        self.val
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _market: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
            _market: PhantomData,
        }
    }

    pub fn push_front(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.next = self.head;
        node.prev = None;
        let node = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.tail = node,
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
        }

        self.head = node;
        self.length += 1;
    }

    pub fn push_back(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));
        node.next = None;
        node.prev = self.tail;
        let node = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node,
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
        }

        self.tail = node;
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            self.length -= 1;

            unsafe {
                let node = Box::from_raw(node.as_ptr());

                self.head = node.next;

                match self.head {
                    None => self.tail = None,
                    Some(head) => (*head.as_ptr()).prev = None,
                }
                node.into_val()
            }
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            self.length -= 1;

            unsafe {
                let node = Box::from_raw(node.as_ptr());

                self.tail = node.prev;

                match self.tail {
                    None => self.head = None,
                    Some(tail) => (*tail.as_ptr()).next = None,
                }
                node.into_val()
            }
        })
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().val) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().val) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().val) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().val) }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn get(&self, at: usize) -> &T {
        assert!(at < self.length);
        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        unsafe { &cur.as_ref().unwrap().as_ref().val }
    }

    pub fn get_mut(&mut self, at: usize) -> &mut T {
        assert!(at < self.length);
        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        unsafe { &mut cur.as_mut().unwrap().as_mut().val }
    }

    pub fn insert(&mut self, at: usize, data: T) {
        assert!(at <= self.length);

        if at == 0 {
            return self.push_front(data);
        } else if at == self.length {
            return self.push_back(data);
        }

        let at = at - 1;
        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        unsafe {
            let mut spliced_node = Box::new(Node::new(data));
            let before_node = cur;
            let after_node = before_node.unwrap().as_ref().next;
            spliced_node.prev = before_node;
            spliced_node.next = after_node;

            let spliced_node = NonNull::new(Box::into_raw(spliced_node));
            before_node.unwrap().as_mut().next = spliced_node;
            after_node.unwrap().as_mut().prev = spliced_node;
        }

        self.length += 1;
    }

    pub fn remove(&mut self, at: usize) -> T {
        assert!(at <= self.length);

        if at == 0 {
            return self.pop_front().unwrap();
        } else if at == self.length {
            return self.pop_back().unwrap();
        }

        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        let node = unsafe { cur.unwrap().as_mut() };

        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            None => self.head = node.next,
        }

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            None => self.tail = node.prev,
        }

        self.length -= 1;

        unsafe {
            let unliked_node = Box::from_raw(cur.unwrap().as_ptr());
            unliked_node.val
        }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn split_off(&mut self, at: usize) -> LinkedList<T> {
        assert!(at <= self.length);

        if at == 0 || at == self.length {
            return LinkedList {
                head: None,
                tail: None,
                length: 0,
                _market: PhantomData,
            };
        }

        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        unsafe {
            let before_node = cur.unwrap().as_ref().prev;
            before_node.unwrap().as_mut().next = None;
            let tail = self.tail;
            self.tail = before_node;
            let length = self.length;
            self.length = at;
            LinkedList {
                head: cur,
                tail,
                length: length - at,
                _market: PhantomData,
            }
        }
    }

    pub fn find_mut<P>(&mut self, predicate: P) -> Option<&mut T>
    where
        P: Fn(&T) -> bool,
    {
        self.iter_mut().find(|x| predicate(x))
    }

    fn get_ptr(&mut self, at: usize) -> Option<NonNull<Node<T>>> {
        assert!(at < self.length);
        let offset_from_end = self.length - at - 1;
        let mut cur;

        if at <= offset_from_end {
            cur = self.head;
            for _ in 0..at {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().next;
                    },
                    None => cur = self.head,
                }
            }
        } else {
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                    None => cur = self.tail,
                }
            }
        }

        cur
    }
}

impl<T> LinkedList<T>
where
    T: PartialEq,
{
    pub fn contains(&self, data: &T) -> bool {
        self.iter().any(|x| x == data)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.length, Some(self.list.length))
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
    _marker: PhantomData<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            length: self.length,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.head.map(|node| {
                self.length -= 1;

                unsafe {
                    let node = &*node.as_ptr();
                    self.head = node.next;
                    &node.val
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.tail.map(|node| {
                self.length -= 1;

                unsafe {
                    let node = &*node.as_ptr();
                    self.tail = node.prev;
                    &node.val
                }
            })
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
    _marker: PhantomData<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            length: self.length,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.head.map(|node| {
                self.length -= 1;

                unsafe {
                    let node = &mut *node.as_ptr();
                    self.head = node.next;
                    &mut node.val
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.length == 0 {
            None
        } else {
            self.tail.map(|node| {
                self.length -= 1;

                unsafe {
                    let node = &mut *node.as_ptr();
                    self.tail = node.prev;
                    &mut node.val
                }
            })
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            std::mem::forget(guard);
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self.iter() {
            new_list.push_back(item.clone());
        }
        new_list
    }
}
impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        list.extend(iter);
        list
    }
}

pub trait MergeSort {
    fn merge_sort(&mut self);
    fn merge_sorted(&mut self, lo: usize, hi: usize);
    unsafe fn merge(&mut self, low: usize, middle: usize, high: usize);
}

impl<T: PartialOrd + Default + Clone> MergeSort for LinkedList<T> {
    fn merge_sort(&mut self) {
        self.merge_sorted(0, self.length)
    }

    fn merge_sorted(&mut self, lo: usize, hi: usize) {
        if hi - lo < 2 {
            return;
        }
        let mi = (lo + hi) >> 1;
        self.merge_sorted(lo, mi);
        self.merge_sorted(mi, hi);
        unsafe { self.merge(lo, mi, hi) }
    }

    unsafe fn merge(&mut self, low: usize, middle: usize, high: usize) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        let mut b = LinkedList::new();
        let mut node = self.get_ptr(low);

        for _ in low..middle {
            unsafe {
                b.push_back(node.as_ref().unwrap().as_ref().val.clone());
            }
            match node.take() {
                Some(current) => unsafe {
                    node = current.as_ref().next;
                },
                None => node = self.tail,
            }
        }

        let (lb, lc) = (middle - low, high - middle);

        let mut b_j = b.get_ptr(j);
        let mut a_middle_k = self.get_ptr(middle + k);
        let mut a_low_i = self.get_ptr(low + i);

        while j < lb && k < lc {
            unsafe {
                if b_j.unwrap().as_ref().val <= a_middle_k.unwrap().as_ref().val {
                    a_low_i.as_mut().unwrap().as_mut().val =
                        b_j.as_ref().unwrap().as_ref().val.clone();
                    match b_j.take() {
                        Some(current) => {
                            b_j = current.as_ref().next;
                        }
                        None => b_j = b.tail,
                    }
                    j += 1;
                } else {
                    a_low_i.as_mut().unwrap().as_mut().val =
                        a_middle_k.as_ref().unwrap().as_ref().val.clone();
                    match a_middle_k.take() {
                        Some(current) => {
                            a_middle_k = current.as_ref().next;
                        }
                        None => a_middle_k = self.tail,
                    }
                    k += 1;
                }

                match a_low_i.take() {
                    Some(current) => {
                        a_low_i = current.as_ref().next;
                    }
                    None => a_low_i = self.tail,
                }
                i += 1;
            }
        }

        while j < lb {
            a_low_i.as_mut().unwrap().as_mut().val = b_j.as_ref().unwrap().as_ref().val.clone();

            match a_low_i.take() {
                Some(current) => {
                    a_low_i = current.as_ref().next;
                }
                None => a_low_i = self.tail,
            }
            i += 1;
            match b_j.take() {
                Some(current) => {
                    b_j = current.as_ref().next;
                }
                None => b_j = b.tail,
            }
            j += 1;
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<T: Send> Send for IntoIter<T> {}
unsafe impl<T: Sync> Sync for IntoIter<T> {}

unsafe impl<'a, T: Send> Send for Iter<'a, T> {}
unsafe impl<'a, T: Sync> Sync for Iter<'a, T> {}

unsafe impl<'a, T: Send> Send for IterMut<'a, T> {}
unsafe impl<'a, T: Sync> Sync for IterMut<'a, T> {}
