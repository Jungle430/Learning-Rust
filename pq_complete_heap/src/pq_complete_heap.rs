use std::cmp::Ordering::Less;

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
pub struct PqCompleteHeap<T>
where
    T: Ord,
{
    elem: Vec<T>,
}

#[allow(dead_code)]
impl<T> PqCompleteHeap<T>
where
    T: Ord,
{
    #[inline]
    pub fn new() -> Self {
        PqCompleteHeap { elem: vec![] }
    }

    #[inline]
    pub fn new_with_vec(elem: Vec<T>) -> Self {
        let mut pq = Self { elem };
        pq.heapify();
        pq
    }

    pub fn push(&mut self, e: T) {
        self.elem.push(e);
        self.percolate_up(self.elem.len() - 1);
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.elem.first_mut()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elem.first()
    }

    pub fn pop(&mut self) -> Option<T> {
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
}

impl<T> PqCompleteHeap<T>
where
    T: Ord,
{
    fn percolate_up(&mut self, mut i: usize) {
        while i > 0 {
            let j = parent!(i);
            if self.elem[j].cmp(&self.elem[i]) != Less {
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

            largest = if left_child < size && self.elem[left_child].cmp(&self.elem[largest]) != Less
            {
                left_child
            } else {
                largest
            };

            largest =
                if right_child < size && self.elem[right_child].cmp(&self.elem[largest]) != Less {
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
