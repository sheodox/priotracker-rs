use std::io;

struct PrioData {
    lists: Vec<PrioItem>,
}
struct PrioItem {
    name: String,
    items: Vec<String>,
}

fn main() {
    let mut data = PrioData {
        lists: vec![PrioItem {name: "High".to_string(), items: vec!["something".to_string(), "something else".to_string()]}]
    };

    loop {
        let prompt = "What do you want to do?";
        let choices = vec![
            "1. View priority lists".to_string(),
            "2. Create a new priority".to_string(),
        ];

        let desire = switch_board(prompt, choices);

        match desire.as_ref() {
            "1" => {view_priority_lists(&data);},
            "2" => {create_priority_list(&data);},
            _ => {println!("Please enter a valid number!");}
        }
    }
}

fn view_priority_lists(data: &PrioData) {
    if data.lists.len() == 0 {
        println!("You have no lists!");
    }
    else {
        let mut choices: Vec<String> = vec![];
        let mut index = 0;

        for list in &data.lists {
            index += 1;
            let mut text = index.to_string();
            text.push_str(". ");
            text.push_str(&list.name);
            choices.push(text);
        }

        println!("");

        let prompt = "Which list do you want to view?";

        let desire = switch_board(prompt, choices);
        let mut index: usize = desire.parse().unwrap();
        index -= 1;
        view_list(index, &data);
    }
}

fn view_list(list_index: usize, data: &PrioData) {
    let list = &data.lists[list_index];
    println!("\n--{}--", list.name);

    for item in &list.items {
        println!("{}", item);
    }
    
}

fn create_priority_list(data: &PrioData) {
    println!("create list!");
}

fn switch_board(prompt: &str, choices: Vec<String>) -> String {
    println!("{}", prompt);
    for choice in choices {
        println!("{}", choice);
    }

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .ok()
        .expect("Failed to read line.");

    choice.trim().to_string()
}
