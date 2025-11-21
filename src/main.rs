use std::io::{stdin, stdout, Write};

enum Mode {
    Long,
    Short,
}

fn main() {
    loop {
        let mode = choose_mode();

        println!("Введите текст, чтобы выйти введите \"exit\"");

        let mut input = String::new();

        stdin().read_line(&mut input).expect("Ошибка ввода");

        input = input.trim().to_string();

        if input == "exit" {
            return;
        }

        let result = match mode {
            Mode::Long => convert_text_to_woof_long(input),
            Mode::Short => convert_text_to_woof_short(input),
        };

        println!("{}", result);
    }
}

fn choose_mode() -> Mode {
    let mut input = String::new();

    print!("Выберите режим 1 - длинный формат, 2 - короткий: ");
    stdout().flush().expect("Ошибка вывода");

    stdin()
        .read_line(&mut input)
        .expect("Не подходящий ответ");

    let input = input.trim();

    match input {
        "1" => Mode::Long,
        "2" => Mode::Short,
        _ => Mode::Long,
    }
}

fn convert_text_to_woof_long(text: String) -> String {
    let punctuation_marks = [",", ".", "!", "?"];
    let mut new_woof_vec: Vec<String> = vec![];

    for word in text.split_whitespace() {
        let mut new_word = String::new();

        if word.chars().next().unwrap().is_uppercase() {
            new_word += "W"
        } else {
            new_word += "w"
        }

        if word.chars().count() > 4 {
            let mut modifier: usize = 2;
            for d in punctuation_marks {
                if word.contains(d) {
                    modifier += 1;
                }
            }

            let count_o = word.chars().count() - modifier;
            for _ in 0..count_o {
                new_word += "o";
            }
        } else {
            new_word += "oo";
        }

        new_word += "f";
        for d in punctuation_marks {
            if word.contains(d) {
                new_word += d;
            }
        }

        new_woof_vec.push(new_word);
    }

    new_woof_vec.join(" ")
}

fn convert_text_to_woof_short(text: String) -> String {
    let vowels = ['a', 'e', 'u', 'o', 'i', 'а', 'е', 'ё', 'и', 'о', 'у', 'ы'];
    let mut new_woof_vec: Vec<String> = vec![];

    for word in text.split_whitespace() {
        let mut new_word = String::new();

        for d in vowels {
            if word.to_lowercase().starts_with(d) {
                new_word += "Woof";
            }
        }

        let mut i: usize;

        if word.chars().count() > 0 {
            i = 1;
        } else {
            new_word += "Woof";
            i = 2;
        }

        while i < word.chars().count() {
            for d in vowels {
                if word.to_lowercase().chars().nth(i) == Some(d) {
                    new_word += "woof";
                }
            }
            i += 1;
        }

        new_woof_vec.push(new_word);
    }

    new_woof_vec.join(" ")
}
