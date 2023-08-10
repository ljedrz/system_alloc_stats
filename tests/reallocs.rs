use alloc_stats::SystemWithStats;

#[global_allocator]
static SWS: SystemWithStats = SystemWithStats;

#[test]
fn realloc_basics() {
    // Start with a blank slate.
    SWS.reset();

    // Allocate a vector.
    let mut vec = Vec::<u8>::with_capacity(7);
    assert_eq!(SWS.alloc_count(), 1);
    assert_eq!(SWS.alloc_sum(), 7);
    assert_eq!(SWS.realloc_growth_count(), 0);
    assert_eq!(SWS.realloc_growth_sum(), 0);
    assert_eq!(SWS.realloc_growth_avg(), None);
    assert_eq!(SWS.realloc_shrink_count(), 0);
    assert_eq!(SWS.realloc_shrink_sum(), 0);
    assert_eq!(SWS.realloc_shrink_avg(), None);

    // Grow the vector.
    vec.reserve_exact(13);
    assert_eq!(SWS.alloc_count(), 2);
    assert_eq!(SWS.alloc_sum(), 20);
    assert_eq!(SWS.realloc_growth_count(), 1);
    assert_eq!(SWS.realloc_growth_sum(), 6);
    assert_eq!(SWS.realloc_growth_avg(), Some(6));
    assert_eq!(SWS.realloc_shrink_count(), 0);
    assert_eq!(SWS.realloc_shrink_sum(), 0);
    assert_eq!(SWS.realloc_shrink_avg(), None);

    // Grow the vector again.
    vec.reserve_exact(23);
    assert_eq!(SWS.alloc_count(), 3);
    assert_eq!(SWS.alloc_sum(), 43);
    assert_eq!(SWS.realloc_growth_count(), 2);
    assert_eq!(SWS.realloc_growth_sum(), 16);
    assert_eq!(SWS.realloc_growth_avg(), Some(8));
    assert_eq!(SWS.realloc_shrink_count(), 0);
    assert_eq!(SWS.realloc_shrink_sum(), 0);
    assert_eq!(SWS.realloc_shrink_avg(), None);

    // Shrink the vector.
    vec.shrink_to(15);
    assert_eq!(SWS.alloc_count(), 4);
    assert_eq!(SWS.alloc_sum(), 58);
    assert_eq!(SWS.realloc_growth_count(), 2);
    assert_eq!(SWS.realloc_growth_sum(), 16);
    assert_eq!(SWS.realloc_growth_avg(), Some(8));
    assert_eq!(SWS.realloc_shrink_count(), 1);
    assert_eq!(SWS.realloc_shrink_sum(), 8);
    assert_eq!(SWS.realloc_shrink_avg(), Some(8));

    // Shrink the vector again.
    vec.shrink_to(5);
    assert_eq!(SWS.alloc_count(), 5);
    assert_eq!(SWS.alloc_sum(), 63);
    assert_eq!(SWS.realloc_growth_count(), 2);
    assert_eq!(SWS.realloc_growth_sum(), 16);
    assert_eq!(SWS.realloc_growth_avg(), Some(8));
    assert_eq!(SWS.realloc_shrink_count(), 2);
    assert_eq!(SWS.realloc_shrink_sum(), 18);
    assert_eq!(SWS.realloc_shrink_avg(), Some(9));
}
