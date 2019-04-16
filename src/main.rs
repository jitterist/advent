mod day_1;
mod day_10;
mod day_11;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let mut sw_all = stopwatch::Stopwatch::start_new();
    let mut sw = stopwatch::Stopwatch::new();

    sw.restart();
    let result_1_1 = day_1::result_part_one();
    sw.stop();
    println!("Result_1_1: {} ({:?})", result_1_1, sw.elapsed());

    sw.restart();
    let result_1_2 = day_1::result_part_two();
    sw.stop();
    println!("Result_1_2: {} ({:?})", result_1_2, sw.elapsed());

    sw.restart();
    let result_2_1 = day_2::result_part_one();
    sw.stop();
    println!("Result_2_1: {} ({:?})", result_2_1, sw.elapsed());

    sw.restart();
    let result_2_2 = day_2::result_part_two();
    sw.stop();
    println!("Result_2_2: {} ({:?})", result_2_2, sw.elapsed());

    sw.restart();
    let result_3_1 = day_3::result_part_one();
    sw.stop();
    println!("Result_3_1: {} ({:?})", result_3_1, sw.elapsed());

    sw.restart();
    let result_3_2 = day_3::result_part_two();
    sw.stop();
    println!("Result_3_2: {} ({:?})", result_3_2, sw.elapsed());

    sw.restart();
    let result_4_1 = day_4::result_part_one();
    sw.stop();
    println!("Result_4_1: {} ({:?})", result_4_1, sw.elapsed());

    sw.restart();
    let result_4_2 = day_4::result_part_two();
    sw.stop();
    println!("Result_4_2: {} ({:?})", result_4_2, sw.elapsed());

    sw.restart();
    let result_5_1 = day_5::result_part_one();
    sw.stop();
    println!("Result_5_1: {} ({:?})", result_5_1, sw.elapsed());

    sw.restart();
    let result_5_2 = day_5::result_part_two();
    sw.stop();
    println!("Result_5_2: {} ({:?})", result_5_2, sw.elapsed());

    sw.restart();
    let result_6_1 = day_6::result_part_one();
    sw.stop();
    println!("Result_6_1: {} ({:?})", result_6_1, sw.elapsed());

    sw.restart();
    let result_6_2 = day_6::result_part_two();
    sw.stop();
    println!("Result_6_2: {} ({:?})", result_6_2, sw.elapsed());

    sw.restart();
    let result_7_1 = day_7::result_part_one();
    sw.stop();
    println!("Result_7_1: {} ({:?})", result_7_1, sw.elapsed());

    sw.restart();
    let result_7_2 = day_7::result_part_two();
    sw.stop();
    println!("Result_7_2: {} ({:?})", result_7_2, sw.elapsed());

    sw.restart();
    let result_8_1 = day_8::result_part_one();
    sw.stop();
    println!("Result_8_1: {} ({:?})", result_8_1, sw.elapsed());

    sw.restart();
    let result_8_2 = day_8::result_part_two();
    sw.stop();
    println!("Result_8_2: {} ({:?})", result_8_2, sw.elapsed());

    sw.restart();
    let result_9_1 = day_9::result_part_one();
    sw.stop();
    println!("Result_9_1: {} ({:?})", result_9_1, sw.elapsed());

    sw.restart();
    let result_9_2 = day_9::result_part_two();
    sw.stop();
    println!("Result_9_2: {} ({:?})", result_9_2, sw.elapsed());

    sw.restart();
    let (result_10_1, result_10_2) = day_10::result();
    sw.stop();
    println!("Result_10_1:\n{}", result_10_1);
    println!("Result_10_2: {} ({:?})", result_10_2, sw.elapsed());

    sw.restart();
    let result_11_1 = day_11::result_part_one();
    sw.stop();
    println!("Result_11_1: {:?} ({:?})", result_11_1, sw.elapsed());

    sw.restart();
    let result_11_2 = day_11::result_part_two();
    sw.stop();
    println!("Result_11_2: {:?} ({:?})", result_11_2, sw.elapsed());

    sw_all.stop();
    println!("\nDuration: {:?}", sw_all.elapsed())
}
