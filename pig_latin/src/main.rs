/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog and eats an apple");
    let mut pig_text = String::new();
    let vogais = vec!['a', 'e', 'i', 'o', 'u'];

    let mut i: i32;
    for word in text.split_ascii_whitespace() {
        let mut new_word = String::new();
        i = 0;
        let mut last_letter = '0';
        for c in word.chars() {
            if i == 0 {
                if vogais.contains(&c) && i == 0 {
                    new_word.push_str(word);
                    new_word.push_str("-h");
                    break;
                } else {
                    last_letter = c;
                }
            } else {
                new_word.push(c);
            }
            
            i += 1;
        }
        if last_letter != '0'{
            new_word.push('-');
            new_word.push(last_letter);
        }
            
        new_word.push_str("ay ");
        pig_text.push_str(&new_word);
    }
    println!("{pig_text}");
}
