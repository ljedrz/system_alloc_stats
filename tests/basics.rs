use rand::{thread_rng, Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use system_alloc_stats::SystemWithStats;

#[global_allocator]
static SWS: SystemWithStats = SystemWithStats;

#[test]
fn alloc_basics() {
    // A bit of test config.
    let min_alloc_size = 10;
    let max_alloc_size = 100;

    // Prepare counters holding expected values.
    let mut num_allocs = 0;
    let mut sum_allocs = 0;
    let mut num_deallocs = 0;
    let mut sum_deallocs = 0;
    let mut num_realloc_growth = 0;
    let mut sum_realloc_growth = 0;
    let mut num_realloc_shrink = 0;
    let mut sum_realloc_shrink = 0;
    let mut curr_use;
    let max_init_use;
    let mut max_alloc = 0;

    // Prepare a source of randomness.
    let mut rng = XorShiftRng::from_rng(thread_rng()).unwrap();

    // Start with a blank slate.
    SWS.reset();

    // Initialize current and max mem use.
    curr_use = SWS.use_curr();
    max_init_use = SWS.use_max();

    for _ in 0..100 {
        // Create a small allocation of a random size.
        let alloc_size: usize = rng.gen_range(min_alloc_size..=max_alloc_size);
        let mut alloc: Vec<u8> = Vec::with_capacity(alloc_size);

        // Possibly update the max alloc size.
        if alloc_size > max_alloc {
            max_alloc = alloc_size;
        }

        num_allocs += 1;
        sum_allocs += alloc_size;
        curr_use += alloc_size;

        assert_eq!(SWS.alloc_count(), num_allocs);
        assert_eq!(SWS.alloc_sum(), sum_allocs);
        assert_eq!(SWS.alloc_avg(), sum_allocs.checked_div(num_allocs));

        assert_eq!(SWS.dealloc_count(), num_deallocs);
        assert_eq!(SWS.dealloc_sum(), sum_deallocs);
        assert_eq!(SWS.dealloc_avg(), sum_deallocs.checked_div(num_deallocs));

        assert_eq!(SWS.realloc_growth_count(), num_realloc_growth);
        assert_eq!(SWS.realloc_growth_sum(), sum_realloc_growth);
        assert_eq!(
            SWS.realloc_growth_avg(),
            sum_realloc_growth.checked_div(num_realloc_growth)
        );

        assert_eq!(SWS.realloc_shrink_count(), num_realloc_shrink);
        assert_eq!(SWS.realloc_shrink_sum(), sum_realloc_shrink);
        assert_eq!(
            SWS.realloc_shrink_avg(),
            sum_realloc_shrink.checked_div(num_realloc_shrink)
        );

        assert_eq!(SWS.use_curr(), curr_use);
        assert_eq!(SWS.use_max(), max_init_use + max_alloc);

        let realloc_decision: u8 = rng.gen_range(0..3);
        match realloc_decision {
            0 => {
                // Don't reallocate.
                sum_deallocs += alloc_size;
                curr_use -= alloc_size;
            }
            1 => {
                // Grow by realloc_size.
                let realloc_size = rng.gen_range(min_alloc_size..max_alloc_size);
                alloc.reserve_exact(alloc_size + realloc_size);

                // Possibly update the max alloc size.
                if alloc_size * 2 + realloc_size > max_alloc {
                    max_alloc = alloc_size * 2 + realloc_size;
                }

                num_deallocs += 1;
                sum_deallocs += alloc_size;

                num_allocs += 1;
                sum_allocs += alloc_size + realloc_size;

                num_realloc_growth += 1;
                sum_realloc_growth += realloc_size;

                sum_deallocs += alloc_size + realloc_size;

                curr_use += realloc_size;
                assert_eq!(SWS.use_curr(), curr_use);
                curr_use -= alloc_size + realloc_size;
            }
            2 => {
                // Shrink by realloc_size.
                let realloc_size = rng.gen_range(1..min_alloc_size);
                alloc.shrink_to(alloc_size - realloc_size);

                // Possibly update the max alloc size.
                if alloc_size * 2 - realloc_size > max_alloc {
                    max_alloc = alloc_size * 2 - realloc_size;
                }

                num_deallocs += 1;
                sum_deallocs += alloc_size;

                num_allocs += 1;
                sum_allocs += alloc_size - realloc_size;

                num_realloc_shrink += 1;
                sum_realloc_shrink += realloc_size;

                sum_deallocs += alloc_size - realloc_size;

                curr_use -= realloc_size;
                assert_eq!(SWS.use_curr(), curr_use);
                curr_use -= alloc_size - realloc_size;
            }
            _ => unreachable!(),
        }

        // at this point the last alloc of the iteration will be dropped
        num_deallocs += 1;
    }
}
