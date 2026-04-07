use std::io::{self, Write};

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
    println!("\n=== Начало игры ===");
    println!("\n...\n");
    wait_for_enter();
}

fn wait_for_enter() {
    println!("Нажмите Enter, чтобы вернуться в меню...");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).unwrap();
}