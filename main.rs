use std::io::{self, Write};
use std::fs;
use rand::seq::IndexedRandom;
use std::collections::HashSet;

fn main() {
    loop {
        print_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => play_game(),
            2 => {
                print_rules();
                wait_for_enter();
            }
            3 => {
                println!("\nДо свидания!");
                break;
            }
            _ => println!("\nНеверный выбор. Введите 1, 2 или 3."),
        }
    }
}

fn print_menu() {
    println!("\n╔══════════════════════════════════╗");
    println!("║           В И С Е Л И Ц А         ║");
    println!("╠══════════════════════════════════╣");
    println!("║         1 - Играть               ║");
    println!("║         2 - Правила              ║");
    println!("║         3 - Выход                ║");
    println!("╚══════════════════════════════════╝");
    print!("Ваш выбор: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения ввода");
    
    input.trim().parse().unwrap_or(0)
}

fn load_words() -> Vec<String> {
    match fs::read_to_string("words.txt") {
        Ok(content) => {
            content
                .lines()
                .map(String::from)
                .filter(|s| !s.is_empty())
                .collect()
        }
        Err(e) => {
            eprintln!("Не удалось открыть файл {}", e);
            vec!["rust".to_string(), "код".to_string(), "тест".to_string()]
        }
    }
}

fn draw_hangman(mistakes: u32) {
    println!("\n╔════════════════════╗");
    println!("║     В И С Е Л И Ц А  ║");
    println!("╠════════════════════╣");
    
    match mistakes {
        0 => {
            println!("║                    ║");
            println!("║                    ║");
            println!("║                    ║");
            println!("║                    ║");
            println!("║                    ║");
            println!("║                    ║");
            println!("║   ═════════════    ║");
        }
        1 => {
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        2 => {
            println!("║    ┌───│           ║");
            println!("║    │   │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        3 => {
            println!("║    ┌───│           ║");
            println!("║    │   │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        4 => {
            println!("║    ┌───│           ║");
            println!("║    │   │           ║");
            println!("║    ●   │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        5 => {
            println!("║    ┌───│           ║");
            println!("║    │   │           ║");
            println!("║    ●   │           ║");
            println!("║    │   │           ║");
            println!("║    │   │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        6 => {
            println!("║    ┌───│           ║");
            println!("║    │   │           ║");
            println!("║    ●   │           ║");
            println!("║   /│\\  │           ║");
            println!("║   / \\  │           ║");
            println!("║        │           ║");
            println!("║   ═════════════    ║");
        }
        _ => {}
    }
    println!("╚════════════════════╝\n");
}

fn print_rules() {
    println!("\n📜 ПРАВИЛА ИГРЫ «ВИСЕЛИЦА»");
    println!("────────────────────────────────────");
    println!("1. Программа загадывает слово.");
    println!("2. Вы называете по одной букве.");
    println!("3. Если буква есть в слове, она открывается на экране.");
    println!("4. Если буквы нет, к виселице добавляется одна деталь.");
    println!("5. У вас 6 попыток (ошибок).");
    println!("6. Победа: угадать слово до полной сборки виселицы.");
    println!("7. Поражение: виселица собрана, а слово не отгадано.");
    println!("────────────────────────────────────\n");
}

fn play_game() {
    println!("\n=== Играем ===");
    let words = load_words();
    
    let secret_word = words.choose(&mut rand::rng()).unwrap();

    let secret_lower = secret_word.to_lowercase();

    let target_word: Vec<char> = secret_lower.chars().collect();

    let mut guessed_letters: HashSet<char> = HashSet::new(); // Набор угаданных букв
    let mut mistakes = 0;
    let max_mistakes = 6;

    loop {
        draw_hangman(mistakes);

        let current_display: String = target_word.iter().map(|c| {
            if guessed_letters.contains(c) { *c } else { '_' }
        }).collect::<Vec<char>>().into_iter().collect::<String>();

        println!("\nСлово: {}", current_display.split("").filter(|s| !s.is_empty()).collect::<Vec<&str>>().join(" "));
        
        println!("Ошибки: {}/{}", mistakes, max_mistakes);
        
        if !current_display.contains('_') {
            println!("\nК СОЖАЛЕНИЮ ТЫ ВЫИГРАЛ!{}", secret_word);
            break;
        }

        if mistakes >= max_mistakes {
            println!("\nТЫ ПРОИГРАЛ! \nЗагаданное слово: {}", secret_word);
            break;
        }

        let points = 100 + (rand::random::<u32>() % 901);
        println!("{} очков на барабане", points);
        println!("Буква: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода");
        
        let guess = match input.trim().to_lowercase().chars().next() {
            Some(c) => c,
            None => {
                println!("Ты ничего не ввел. Давай снова.");
                continue;
            }
        };

        if guessed_letters.contains(&guess) {
            println!("Ты уже называл эту букву букву!");
            continue;
        }

        guessed_letters.insert(guess);

        if secret_lower.contains(guess) {
            println!("ОТКРОЙТЕ!");
        } else {
            mistakes += 1;
            println!("Нет такой буквы!");
            println!("ВРАЩАЙТЕ БАРАБАН!");
        }
    }

    wait_for_enter();
}

fn wait_for_enter() {
    println!("Нажмите Enter, чтобы вернуться в меню...");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
}