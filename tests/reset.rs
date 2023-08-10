use alloc_stats::SystemWithStats;

#[global_allocator]
static SWS: SystemWithStats = SystemWithStats;

#[test]
fn reset_basics() {
    // There are always some initial allocations.
    assert_ne!(SWS.alloc_count(), 0);
    assert_ne!(SWS.alloc_sum(), 0);
    assert!(SWS.alloc_avg().is_some());

    // Reset them.
    SWS.reset();

    // Check that all the relevant counters are 0.
    assert_eq!(SWS.alloc_count(), 0);
    assert_eq!(SWS.alloc_sum(), 0);
    assert_eq!(SWS.alloc_avg(), None);
    assert_eq!(SWS.dealloc_count(), 0);
    assert_eq!(SWS.dealloc_sum(), 0);
    assert_eq!(SWS.dealloc_avg(), None);
    assert_eq!(SWS.realloc_growth_count(), 0);
    assert_eq!(SWS.realloc_growth_sum(), 0);
    assert_eq!(SWS.realloc_growth_avg(), None);
    assert_eq!(SWS.realloc_shrink_count(), 0);
    assert_eq!(SWS.realloc_shrink_sum(), 0);
    assert_eq!(SWS.realloc_shrink_avg(), None);
}
