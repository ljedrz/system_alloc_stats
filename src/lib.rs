#[cfg(feature = "fmt")]
use std::fmt;
use std::{
    alloc::{GlobalAlloc, Layout, System},
    cmp, ptr,
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(feature = "fmt")]
use humansize::{format_size, BINARY};
#[cfg(feature = "fmt")]
use num_format::{Locale, ToFormattedString};

#[derive(Debug, Default, Clone, Copy)]
pub struct SystemWithStats;

#[derive(Debug, Default, Clone)]
pub struct SystemStats {
    pub alloc_count: usize,
    pub alloc_avg: Option<usize>,
    pub realloc_growth_count: usize,
    pub realloc_growth_avg: Option<usize>,
    pub realloc_shrink_count: usize,
    pub realloc_shrink_avg: Option<usize>,
    pub use_curr: usize,
    pub use_max: usize,
}

#[cfg(feature = "fmt")]
impl fmt::Display for SystemStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "SystemStats {{")?;
        writeln!(
            f,
            "\talloc_count: {}",
            self.alloc_count.to_formatted_string(&Locale::en)
        )?;
        if let Some(alloc_avg) = self.alloc_avg {
            writeln!(f, "\talloc_avg: {}", format_size(alloc_avg, BINARY))?;
        }
        writeln!(
            f,
            "\trealloc_growth_count: {}",
            self.realloc_growth_count.to_formatted_string(&Locale::en)
        )?;
        if let Some(realloc_growth_avg) = self.realloc_growth_avg {
            writeln!(
                f,
                "\trealloc_growth_avg: {}",
                format_size(realloc_growth_avg, BINARY)
            )?;
        }
        writeln!(
            f,
            "\trealloc_shrink_count: {}",
            self.realloc_shrink_count.to_formatted_string(&Locale::en)
        )?;
        if let Some(realloc_shrink_avg) = self.realloc_shrink_avg {
            writeln!(
                f,
                "\trealloc_shrink_avg: {}",
                format_size(realloc_shrink_avg, BINARY)
            )?;
        }
        writeln!(f, "\tuse_curr: {}", format_size(self.use_curr, BINARY))?;
        writeln!(f, "\tuse_max: {}", format_size(self.use_max, BINARY))?;
        writeln!(f, "}}")
    }
}

static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOC_SUM: AtomicUsize = AtomicUsize::new(0);
static REALLOC_GROWTH_COUNT: AtomicUsize = AtomicUsize::new(0);
static REALLOC_GROWTH_SUM: AtomicUsize = AtomicUsize::new(0);
static REALLOC_SHRINK_COUNT: AtomicUsize = AtomicUsize::new(0);
static REALLOC_SHRINK_SUM: AtomicUsize = AtomicUsize::new(0);
static USE_CURR: AtomicUsize = AtomicUsize::new(0);
static USE_MAX: AtomicUsize = AtomicUsize::new(0);

impl SystemWithStats {
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

    pub fn alloc_avg(&self) -> Option<usize> {
        let sum = ALLOC_SUM.load(Ordering::Relaxed);
        let count = ALLOC_COUNT.load(Ordering::Relaxed);
        sum.checked_div(count)
    }

    pub fn realloc_growth_avg(&self) -> Option<usize> {
        let sum = REALLOC_GROWTH_SUM.load(Ordering::Relaxed);
        let count = REALLOC_GROWTH_COUNT.load(Ordering::Relaxed);
        sum.checked_div(count)
    }

    pub fn realloc_shrink_avg(&self) -> Option<usize> {
        let sum = REALLOC_SHRINK_SUM.load(Ordering::Relaxed);
        let count = REALLOC_SHRINK_COUNT.load(Ordering::Relaxed);
        sum.checked_div(count)
    }

    pub fn use_curr(&self) -> usize {
        USE_CURR.load(Ordering::Relaxed)
    }

    pub fn use_max(&self) -> usize {
        USE_MAX.load(Ordering::Relaxed)
    }

    pub fn reset(&self) {
        ALLOC_SUM.store(0, Ordering::Relaxed);
        ALLOC_COUNT.store(0, Ordering::Relaxed);
        REALLOC_GROWTH_COUNT.store(0, Ordering::Relaxed);
        REALLOC_GROWTH_SUM.store(0, Ordering::Relaxed);
        REALLOC_SHRINK_COUNT.store(0, Ordering::Relaxed);
        REALLOC_SHRINK_SUM.store(0, Ordering::Relaxed);
        USE_MAX.store(self.use_curr(), Ordering::Relaxed);
    }

    pub fn stats(&self) -> SystemStats {
        let alloc_count = self.alloc_count();
        let alloc_sum = self.alloc_sum();
        let alloc_avg = alloc_sum.checked_div(alloc_count);

        let realloc_growth_count = self.realloc_growth_count();
        let realloc_growth_sum = self.realloc_growth_sum();
        let realloc_growth_avg = realloc_growth_sum.checked_div(realloc_growth_count);

        let realloc_shrink_count = self.realloc_shrink_count();
        let realloc_shrink_sum = self.realloc_shrink_sum();
        let realloc_shrink_avg = realloc_shrink_sum.checked_div(realloc_shrink_count);

        let use_curr = self.use_curr();
        let use_max = self.use_max();

        SystemStats {
            alloc_count,
            alloc_avg,
            realloc_growth_count,
            realloc_growth_avg,
            realloc_shrink_count,
            realloc_shrink_avg,
            use_curr,
            use_max,
        }
    }
}

unsafe impl GlobalAlloc for SystemWithStats {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            let size = layout.size();
            ALLOC_SUM.fetch_add(size, Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
            let curr = USE_CURR.fetch_add(size, Ordering::Relaxed) + size;
            USE_MAX.fetch_max(curr, Ordering::Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        USE_CURR.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            if new_size > layout.size() {
                let diff = new_size - layout.size();
                REALLOC_GROWTH_COUNT.fetch_add(1, Ordering::Relaxed);
                REALLOC_GROWTH_SUM.fetch_add(diff, Ordering::Relaxed);
            } else {
                let diff = layout.size() - new_size;
                REALLOC_SHRINK_COUNT.fetch_add(1, Ordering::Relaxed);
                REALLOC_SHRINK_SUM.fetch_add(diff, Ordering::Relaxed);
            }

            unsafe {
                ptr::copy_nonoverlapping(ptr, new_ptr, cmp::min(layout.size(), new_size));
                self.dealloc(ptr, layout);
            }
        }
        new_ptr
    }
}
