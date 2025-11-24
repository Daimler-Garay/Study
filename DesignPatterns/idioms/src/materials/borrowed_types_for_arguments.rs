// https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html
// Always prefer using borrowed type over borrowing the owned type.
// Such as &str over &String, &[T] over &Vec<T>, or &T over &Box<T>.
// Using borrowed types can avoid layers of indirection for instances where
// the owned type already has a layer of indirection.
// Consider an example where we wish to determine if a word contains three
// consecutive vowels.
fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

fn example_one() {
    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));
}

// This works fine because we are passing a &String type as a parameter.
// The last two lines are considered &str, which doesn't coerce to a &String.
// We can easily fix this by changing the fucntion declaration to
// -> fn three_vowels(word: &str) -> bool {}

// In a scenario where we are given a sentence and want to determine if
// any of the words inside contain three consecutive vowels. We can simply
// reuse the previous function and feed in each word from the sentence.

fn example_two() {
    let sentence_string =
        "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence_string.split(' ') {
        if three_vowels(word) {
            println!("{word} has three consecutive vowels!"); // This will fail if function is declared with &String.
        }
    }
}

// https://archive.ph/LBpD0
// When to use String vs &str?
// According to the article by Steve Klabnik, don't bother thinking about it.
// We can always use 'String' and never '&str', this means we'll need to occasionally
// add '.to_string()' or '.clone()' for things to work. This is okay, because the
// compiler exists and will let us know when its needed.

// Prefer '&str' for function parameters.
// Always use 'String' in structs, and for functions, use '&str' for parameters and 'String'
// types for return values. This results in code that looks like so:

struct Person {
    name: String,
}

fn first_word(words: &str) -> String {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
        .to_string()
}

fn example_three() {
    let sentence = "Hello, world!";

    println!("{}", first_word(sentence));
    
    let owned = String::from("A string");

    println!("{}", first_word(&owned)); // Its necessary to add '&' to '&String' values, but 
    println!("{}", first_word(&owned)); // the compiler will remind us when we forget.
}

// Return '&str' sometimes
// Always use 'String' in structs, '&str' for function parameters.
// If the return type for your function is derived from an argument
// and isn't mutated by the body, return '&str'. If you run into any
// trouble here, return 'String' instead. This results in this:

struct Person2 {
    name: String
}

fn first_word2(words: &str) -> &str { 
    words
        .split_whitespace()
        .next()
        .expect("Words should not be empty")
}

fn example_four() {
    let sentence = "Hello, world!";

    println!("{}", first_word(sentence));

    let owned = String::from("A string");

    println!("{}", first_word(&owned));
    println!("{}", first_word(&owned));
}

// This lets us remove '.to_string()' from 'first_word'.
// This doesn't always work though:

// Because 'to_uppercase()' results in a 'String', our return
// type can't be &str anymore. We are creating our own new
// string.
fn first_word_uppercase(words: &str) -> String {
    words
        .split_whitespace()
        .next()
        .expect("words should not be empty")
        .to_uppercase()
}

fn example_five() {
    let sentence = "Hello, world!";

    println!("{}", first_word_uppercase(sentence));

    let owned = String::from("A string");

    println!("{}", first_word(&owned));
    println!("{}", first_word(&owned));
}
