//Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
use std::io;

const LYRICS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const NUMBERS: [&str; 12] = [
    "a partridge in a pear tree.",
    "two turtle doves,",
    "three french hens,",
    "four calling birds,",
    "five gold rings,",
    "six geese-a-laying,",
    "seven swans-a-swimming,",
    "eight maids-a-milking,",
    "nine Ladies Dancing,",
    "ten Lords-a-Leaping,",
    "eleven Pipers Piping,",
    "twelve Drummers Drumming,",
];

fn get_line(number: usize) -> String {
    format!("On the {} day of Christmas my true love gave to me:{}", LYRICS[number], NUMBERS[number])
}

fn print_line(number: usize) {
    println!("{}", get_line(number));
}

fn main() {
    for i in 0..12 {
        print_line(i);
    }

    println!("Please enter a number between 1 and 12");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number: usize = input.trim().parse().expect("Please type a number!");
    print_line(number - 1);    
}