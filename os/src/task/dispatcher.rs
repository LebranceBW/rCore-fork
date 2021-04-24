type TaskID = usize;
use alloc::collections::BinaryHeap;
use alloc::collections::VecDeque;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::cell::RefCell;
use lazy_static::*;
pub trait Dispatcher: Sync {
    fn set_priority(&mut self, task_id: TaskID, prio: usize);
    fn pop(&mut self) -> Option<TaskID>;
    fn push(&mut self, id: TaskID);
}

#[derive(PartialOrd, PartialEq)]
struct StrideElem {
    id: TaskID,
    m: Rc<RefCell<Vec<usize>>>,
}

impl Eq for StrideElem {}

impl Ord for StrideElem {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let proi = self.m.borrow();
        proi[self.id].cmp(&proi[other.id]).reverse()
    }
}

pub struct Stride {
    priority_map: Rc<RefCell<Vec<usize>>>,
    heap: BinaryHeap<StrideElem>,
}

unsafe impl Sync for Stride {}

impl Stride {
    pub fn new(task_num: usize) -> Self {
        let priority_map = Rc::new(RefCell::new(vec![16; task_num]));
        let mut heap = BinaryHeap::new();
        for id in 0..task_num {
            heap.push(StrideElem {
                m: Rc::clone(&priority_map),
                id: id,
            });
        }
        Self { priority_map, heap }
    }
}

impl Dispatcher for Stride {
    fn set_priority(&mut self, task_id: TaskID, proi: usize) {
        self.priority_map.borrow_mut()[task_id] = proi;
    }

    fn pop(&mut self) -> Option<TaskID> {
        self.heap.pop().map(|ele| ele.id)
    }

    fn push(&mut self, id: TaskID) {
        self.heap.push(StrideElem {
            id,
            m: Rc::clone(&self.priority_map),
        })
    }
}
