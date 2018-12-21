use simd_bench::*;

fn main() {
    const TEST_SIZE: usize = 1 << 16;
    println!("Generating data for {} entities...", TEST_SIZE);
    let (mut initial_pos, velocities) = generate_data(TEST_SIZE);
    println!("Done.");

    if let Some(first) = initial_pos.first() {
        println!("initial first pos: {:#?}", first);
    }
    let new_pos = pos_update_iter_return(&initial_pos, &velocities);
    if let Some(first) = new_pos.first() {
        println!("updated first pos - return: {:#?}", first);
    }

    if let Some(first) = initial_pos.first() {
        println!("before update - seq out param: {:#?}", first);
    }
    pos_update_iter_mut_outparam(&mut initial_pos, &velocities);
    if let Some(first) = initial_pos.first() {
        println!("after update - seq out param: {:#?}", first);
    }
    pos_update_par_iter_mut_outparam(&mut initial_pos, &velocities);
    if let Some(first) = initial_pos.first() {
        println!("updated first pos - par out param: {:#?}", first);
    }
}
