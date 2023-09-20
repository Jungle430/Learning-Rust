use std::cmp::Ordering;

macro_rules! parent {
    ($i:expr) => {
        ($i - 1) >> 1
    };
}

macro_rules! l_child {
    ($i:expr) => {
        ($i << 1) + 1
    };
}

macro_rules! r_child {
    ($i:expr) => {
        ($i + 1) << 1
    };
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PqCompleteHeap<T>
where
    T: Ord,
{
    elem: Vec<T>,
    compare: Ordering,
}

#[allow(dead_code)]
impl<T> PqCompleteHeap<T>
where
    T: Ord,
{
    pub fn new(ord: Ordering) -> Self {
        assert_ne!(ord, Ordering::Equal);
        PqCompleteHeap {
            elem: vec![],
            compare: ord,
        }
    }

    pub fn new_with_vec(elem: Vec<T>, ord: Ordering) -> Self {
        assert_ne!(ord, Ordering::Equal);
        //heapify
        let mut pq = PqCompleteHeap { elem, compare: ord };
        pq.heapify();
        pq
    }

    pub fn insert(&mut self, e: T) {
        self.elem.push(e);
        self.percolate_up(self.elem.len() - 1);
    }

    pub fn get_max(&mut self) -> Option<&mut T> {
        if !self.elem.is_empty() {
            Some(&mut self.elem[0])
        } else {
            None
        }
    }

    pub fn del_max(&mut self) -> Option<T> {
        if !self.elem.is_empty() {
            let size = self.elem.len();
            self.elem.swap(0, size - 1);
            let e = self.elem.pop();
            self.percolate_down(0);
            e
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.elem.len()
    }

    pub fn get_ord(&self) -> Ordering {
        self.compare
    }
}

impl<T> PqCompleteHeap<T>
where
    T: Ord,
{
    fn percolate_up(&mut self, mut i: usize) {
        while i > 0 {
            let j = parent!(i);
            if self.elem[j].cmp(&self.elem[i]) == self.compare {
                break;
            }
            self.elem.swap(i, j);
            i = j;
        }
    }
    fn percolate_down(&mut self, mut i: usize) {
        loop {
            let mut largest = i;
            let size = self.elem.len();
            let (left_child, right_child) = (l_child!(largest), r_child!(largest));

            largest = if left_child < size
                && self.elem[left_child].cmp(&self.elem[largest]) == self.compare
            {
                left_child
            } else {
                largest
            };

            largest = if right_child < size
                && self.elem[right_child].cmp(&self.elem[largest]) == self.compare
            {
                right_child
            } else {
                largest
            };

            if largest != i {
                self.elem.swap(i, largest);
                i = largest;
            } else {
                break;
            }
        }
    }

    fn heapify(&mut self) {
        for i in (0..=((self.elem.len() >> 1) - 1)).rev() {
            self.percolate_down(i);
        }
    }
}
