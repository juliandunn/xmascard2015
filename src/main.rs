extern crate time;

fn main() {
    let gifts = ["Partridge in a Pear Tree",
                 "Turtle Doves",
                 "French Hens",
                 "Calling Birds",
                 "Gold Rings",
                 "Geese a-Laying",
                 "Swans a-Swimming",
                 "Maids a-Milking",
                 "Ladies Dancing",
                 "Lords a-Leaping",
                 "Pipers Piping",
                 "Drummers Drumming"];

    println!("The time is now {:?}", time::now_utc());
}

// Return a concatenated string of the gifts for this day
//fn process_gifts(day: i32) -> str {
//}
