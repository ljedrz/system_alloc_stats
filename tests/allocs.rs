use system_alloc_stats::SystemWithStats;

#[global_allocator]
static SWS: SystemWithStats = SystemWithStats;

#[test]
fn alloc_basics() {
    // Start with a blank slate.
    SWS.reset();

    // Check current and max mem use.
    let initial_use_curr = SWS.use_curr();
    let initial_use_max = SWS.use_max();
    assert_eq!(initial_use_curr, initial_use_max);

    // Allocate 1B.
    let alloc_1 = Box::new(0u8);
    assert_eq!(SWS.alloc_count(), 1);
    assert_eq!(SWS.alloc_sum(), 1);
    assert_eq!(SWS.alloc_avg(), Some(1));
    assert_eq!(SWS.dealloc_count(), 0);
    assert_eq!(SWS.dealloc_sum(), 0);
    assert_eq!(SWS.use_curr(), initial_use_curr + 1);
    assert_eq!(SWS.use_max(), initial_use_max + 1);

    // Allocate 4B.
    let alloc_4 = Box::new(0u32);
    assert_eq!(SWS.alloc_count(), 2);
    assert_eq!(SWS.alloc_sum(), 5);
    assert_eq!(SWS.alloc_avg(), Some(2));
    assert_eq!(SWS.dealloc_count(), 0);
    assert_eq!(SWS.dealloc_sum(), 0);
    assert_eq!(SWS.use_curr(), initial_use_curr + 5);
    assert_eq!(SWS.use_max(), initial_use_max + 5);

    // Allocate a Vec.
    let alloc_95 = Vec::<u8>::with_capacity(95);
    assert_eq!(SWS.alloc_count(), 3);
    assert_eq!(SWS.alloc_sum(), 100);
    assert_eq!(SWS.alloc_avg(), Some(33));
    assert_eq!(SWS.dealloc_count(), 0);
    assert_eq!(SWS.dealloc_sum(), 0);
    assert_eq!(SWS.use_curr(), initial_use_curr + 100);
    assert_eq!(SWS.use_max(), initial_use_max + 100);

    // Register what is expected to be the max heap use.
    let use_max = SWS.use_max();

    // Deallocate 1B.
    drop(alloc_1);
    assert_eq!(SWS.alloc_count(), 3);
    assert_eq!(SWS.alloc_sum(), 100);
    assert_eq!(SWS.dealloc_count(), 1);
    assert_eq!(SWS.dealloc_sum(), 1);
    assert_eq!(SWS.use_curr(), use_max - 1);
    assert_eq!(SWS.use_max(), use_max);

    // Deallocate 4B.
    drop(alloc_4);
    assert_eq!(SWS.alloc_count(), 3);
    assert_eq!(SWS.alloc_sum(), 100);
    assert_eq!(SWS.dealloc_count(), 2);
    assert_eq!(SWS.dealloc_sum(), 5);
    assert_eq!(SWS.use_curr(), use_max - 5);
    assert_eq!(SWS.use_max(), use_max);

    // Deallocate 95B.
    drop(alloc_95);
    assert_eq!(SWS.alloc_count(), 3);
    assert_eq!(SWS.alloc_sum(), 100);
    assert_eq!(SWS.dealloc_count(), 3);
    assert_eq!(SWS.dealloc_sum(), 100);
    assert_eq!(SWS.use_curr(), use_max - 100);
    assert_eq!(SWS.use_max(), use_max);
}
