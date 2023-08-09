use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct AllocStats;

static SUM: AtomicUsize = AtomicUsize::new(0);
static COUNT: AtomicUsize = AtomicUsize::new(0);
static CURRENT: AtomicUsize = AtomicUsize::new(0);
static MAX: AtomicUsize = AtomicUsize::new(0);

impl AllocStats {
    pub fn alloc_count(&self) -> usize {
        COUNT.load(Ordering::Relaxed)
    }

    pub fn total_allocated(&self) -> usize {
        SUM.load(Ordering::Relaxed)
    }

    pub fn current_use(&self) -> usize {
        CURRENT.load(Ordering::Relaxed)
    }

    pub fn max_use(&self) -> usize {
        MAX.load(Ordering::Relaxed)
    }

    pub fn avg_alloc_size(&self) -> usize {
        let sum = SUM.load(Ordering::Relaxed);
        let count = COUNT.load(Ordering::Relaxed);
        sum / count
    }

    pub fn reset(&self) {
        SUM.store(0, Ordering::Relaxed);
        COUNT.store(0, Ordering::Relaxed);
        MAX.store(self.current_use(), Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for AllocStats {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            let size = layout.size();
            SUM.fetch_add(size, Ordering::Relaxed);
            let curr = CURRENT.fetch_add(size, Ordering::Relaxed);
            MAX.fetch_max(curr, Ordering::Relaxed);
            COUNT.fetch_add(1, Ordering::Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        CURRENT.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}
