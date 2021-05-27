pub use anony::alloc;
pub use anony::frame_allocator_test;
pub use anony::MemFrame;

mod anony {
    use super::super::address::PhysAddr;
    use super::super::address::PhysPageNum;
    use crate::config::MEMORY_END;
    use alloc::vec::Vec;
    use lazy_static::*;
    use log::debug;
    use spin::Mutex;
    type FrameAllocatorImpl = StackFrameAllocator;
    extern "C" {
        fn ekernel();
    }
    lazy_static! {
        pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocatorImpl> =
            Mutex::new(FrameAllocatorImpl::new(
                PhysAddr::from(ekernel as usize).ceil(),
                PhysAddr::from(MEMORY_END).floor()
            ));
    }
    pub fn alloc() -> Option<MemFrame> {
        FRAME_ALLOCATOR.lock().alloc()
    }

    fn dealloc(mem_frame: &MemFrame) {
        FRAME_ALLOCATOR.lock().dealloc(mem_frame)
    }

    #[derive(Debug)]
    pub struct MemFrame(pub PhysPageNum);
    impl Drop for MemFrame {
        fn drop(&mut self) {
            dealloc(self);
        }
    }

    pub trait FrameAllocator {
        fn new(begin: PhysPageNum, end: PhysPageNum) -> Self;
        fn alloc(&mut self) -> Option<MemFrame>;
        fn dealloc(&mut self, frame: &MemFrame);
    }

    struct Stack {
        bottom: usize,
        top: usize,
    }
    impl Stack {
        pub fn new(begin: PhysPageNum, end: PhysPageNum) -> Self {
            Self {
                bottom: end.0,
                top: begin.0,
            }
        }
        pub fn pop(&mut self) -> Option<PhysPageNum> {
            if self.top == self.bottom {
                None
            } else {
                self.top += 1;
                Some((self.top - 1).into())
            }
        }
    }
    pub struct StackFrameAllocator {
        raw: Stack,
        recycled: Vec<PhysPageNum>,
    }
    impl FrameAllocator for StackFrameAllocator {
        fn new(begin: PhysPageNum, end: PhysPageNum) -> Self {
            StackFrameAllocator {
                raw: Stack::new(begin, end),
                recycled: Vec::new(),
            }
        }
        fn alloc(&mut self) -> Option<MemFrame> {
            self.recycled.pop().or(self.raw.pop()).map(MemFrame)
        }
        fn dealloc(&mut self, frame: &MemFrame) {
            let ppn = frame.0;
            if self.recycled.iter().find(|&&x| x == ppn).is_some() {
                panic!("Frame ppn={:#x} has not been allocated!", ppn.0);
            }
            self.recycled.push(ppn);
        }
    }
    #[allow(unused)]
    pub fn frame_allocator_test() {
        let mut v = Vec::new();
        for i in 0..5 {
            let frame = alloc().unwrap();
            debug!("{:?}", frame);
            v.push(frame);
        }
        v.clear();
        for i in 0..5 {
            let frame = alloc().unwrap();
            debug!("{:?}", frame);
            v.push(frame);
        }
        drop(v);
        debug!("frame_allocator_test passed!");
    }
}
