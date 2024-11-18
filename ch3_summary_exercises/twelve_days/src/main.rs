fn main() {
    // Print the lyrics to the Christmas carol "The Twelve Days of Christmas", taking advantage of the repetition in the song.
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for idx in 0..days.len() {
        let day = days[idx];

        println!("On the {day} day of Christmas, my true love gave to me...");

        let day = idx+1;

        for count in (1..=day).rev() {
            let gift = match count {
                1 => "A partridge in a pear tree",
                2 => "Two turtle doves",
                3 => "Three french hens",
                4 => "Four calling birds",
                5 => "Five golden rings",
                6 => "Six geese a-laying",
                7 => "Seven swans a-swimming",
                8 => "Eight maids a-milikng",
                9 => "Nine ladies dancing",
                10 => "Ten lords a-leaping",
                11 => "Eleven pipers piping",
                12 => "Twelve drummers drumming",
                _ => "",
            };            
            println!("{gift}");
        }

        println!()
    }
}
