use std::error::Error;

macro_rules! tuple_map {
    ($f:expr => $($x:expr),*) => {
        ($( $f($x) ),*)
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let lines = input.lines();

    let mut part_1_total = 0;

    let mut part_2_total = 0;

    for line in lines {
        let (range_1, range_2) = line.split_once(',').unwrap();

        let (range_1, range_2) =
            tuple_map!(|x: &'static str| x.split_once('-').unwrap() => range_1, range_2);

        let (range_1, range_2) = tuple_map!(
            |x: (&'static str, &'static str)| tuple_map!(
                |y: &'static str| y.parse::<u32>().unwrap() =>
                x.0,
                x.1
            ) =>
            range_1,
            range_2
        );

        let (range_1, range_2) = tuple_map!(
            |(min, max): (u32, u32)| (min..=max) =>
            range_1,
            range_2
        );

        if range_1.contains(range_2.start()) && range_1.contains(range_2.end())
            || range_2.contains(range_1.start()) && range_2.contains(range_1.end())
        {
            part_1_total += 1;
        }

        if range_1.contains(range_2.start()) || range_1.contains(range_2.end())
            || range_2.contains(range_1.start()) || range_2.contains(range_1.end())
        {
            part_2_total += 1;
        }
    }

    println!("Part 1: {part_1_total}");
    println!("Part 2: {part_2_total}");

    Ok(())
}
