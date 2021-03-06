mod data;
mod part_one;
mod part_two;
mod types;
mod utils;

pub fn result_part_one() -> i32 {
    let guard_events = utils::get_guard_events();
    let guard_id = part_one::get_guard_with_most_sleep_time(&guard_events);
    let most_minute_slept = part_one::get_most_slept_minute_for_guard(guard_id, &guard_events) as i32;

    guard_id * most_minute_slept
}

pub fn result_part_two() -> i32 {
    let (guard_id, minute) = part_two::get_guard_and_most_slept_minute();
    guard_id * minute
}
