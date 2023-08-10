use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct AllocStats;

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOC_SUM: AtomicUsize = AtomicUsize::new(0);
static CURR_HEAP: AtomicUsize = AtomicUsize::new(0);
static MAX_HEAP: AtomicUsize = AtomicUsize::new(0);
static REALLOC_GROWTH_COUNT: AtomicUsize = AtomicUsize::new(0);
static REALLOC_GROWTH_SUM: AtomicUsize = AtomicUsize::new(0);
static REALLOC_SHRINK_COUNT: AtomicUsize = AtomicUsize::new(0);
static REALLOC_SHRINK_SUM: AtomicUsize = AtomicUsize::new(0);

impl AllocStats {
    pub fn alloc_count(&self) -> usize {
        ALLOC_COUNT.load(Ordering::Relaxed)
    }

    pub fn alloc_sum(&self) -> usize {
        ALLOC_SUM.load(Ordering::Relaxed)
    }

    pub fn realloc_growth_count(&self) -> usize {
        REALLOC_GROWTH_COUNT.load(Ordering::Relaxed)
    }

    pub fn realloc_growth_sum(&self) -> usize {
        REALLOC_GROWTH_SUM.load(Ordering::Relaxed)
    }

    pub fn realloc_shrink_count(&self) -> usize {
        REALLOC_SHRINK_COUNT.load(Ordering::Relaxed)
    }

    pub fn realloc_shrink_sum(&self) -> usize {
        REALLOC_SHRINK_SUM.load(Ordering::Relaxed)
    }

    pub fn avg_alloc_size(&self) -> usize {
        let sum = ALLOC_SUM.load(Ordering::Relaxed);
        let count = ALLOC_COUNT.load(Ordering::Relaxed);
        sum / count
    }

    pub fn avg_realloc_growth_size(&self) -> usize {
        let sum = REALLOC_GROWTH_SUM.load(Ordering::Relaxed);
        let count = REALLOC_GROWTH_COUNT.load(Ordering::Relaxed);
        sum / count
    }

    pub fn avg_realloc_shrink_size(&self) -> usize {
        let sum = REALLOC_SHRINK_SUM.load(Ordering::Relaxed);
        let count = REALLOC_SHRINK_COUNT.load(Ordering::Relaxed);
        sum / count
    }

    pub fn current_use(&self) -> usize {
        CURR_HEAP.load(Ordering::Relaxed)
    }

    pub fn max_use(&self) -> usize {
        MAX_HEAP.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        ALLOC_SUM.store(0, Ordering::Relaxed);
        ALLOC_COUNT.store(0, Ordering::Relaxed);
        MAX_HEAP.store(self.current_use(), Ordering::Relaxed);
        REALLOC_GROWTH_COUNT.store(0, Ordering::Relaxed);
        REALLOC_GROWTH_SUM.store(0, Ordering::Relaxed);
        REALLOC_SHRINK_COUNT.store(0, Ordering::Relaxed);
        REALLOC_SHRINK_SUM.store(0, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for AllocStats {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            let size = layout.size();
            ALLOC_SUM.fetch_add(size, Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            let curr = CURR_HEAP.fetch_add(size, Ordering::Relaxed);
            MAX_HEAP.fetch_max(curr, Ordering::Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        CURR_HEAP.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let ret = System.realloc(ptr, layout, new_size);
        if !ret.is_null() {
            if new_size > layout.size() {
                REALLOC_GROWTH_COUNT.fetch_add(1, Ordering::Relaxed);
                REALLOC_GROWTH_SUM.fetch_add(new_size - layout.size(), Ordering::Relaxed);
            } else {
                REALLOC_SHRINK_COUNT.fetch_add(1, Ordering::Relaxed);
                REALLOC_SHRINK_SUM.fetch_add(layout.size() - new_size, Ordering::Relaxed);
            }
        }
        ret
    }
}
