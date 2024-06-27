// fn main() {
//     fn find_nemo(sentence: &str) {
//         let word_to_find = "Nemo";
//         match sentence.find(word_to_find) {
//             Some(index) => println!("I found Nemo at {}!", index),
//             None => println!("I can't find Nemo :("),
//         }
//     }

//     find_nemo("I am finding Nemo !");

//     find_nemo("Nemo is me");

//     find_nemo("I Nemo am");
// }

fn main() {
    fn find_nemo(sentence: &str) {
        let word_to_find = "Nemo";

        let mut found = false;

        for (index, word) in sentence.split_whitespace().enumerate() {
            if word == word_to_find {
                println!("I found Nemo at {}!", index + 1);
                found = true;
                break;
            }
        }
        if found == false {
            println!("I can't find Nemo :(")
        }
    }

    find_nemo("I am finding Nemo !");

    find_nemo("Nemo is me");

    find_nemo("I Nemo am");
    find_nemo("I NI Nemo amemoI NeI Nemo ammo am I Nemo amam");
}
