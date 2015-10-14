use std::io;

struct PrioData {
    lists: Vec<PrioList>,
}

struct PrioList {
    name: String,
    items: Vec<String>,
}
impl PrioList {
    fn new(name: &str, items: Vec<String>) -> PrioList {
        PrioList {
            name: name.to_string(),
            items: items.clone(),
        }
    }

    fn add_item(&mut self, new_item: &str) {
        self.items.push(new_item.to_string());
    }
}

fn main() {
    let mut data = PrioData {
        lists: vec![PrioList::new("High", vec!["something".to_string(), "something else".to_string()])]
    };

    loop {
        let prompt = "What do you want to do? (Type \"back\" at any time to return to the previos menu)";
        let choices = vec![
            "1. View priority lists".to_string(),
            "2. Create a new priority".to_string(),
        ];

        let desire = switch_board(prompt, choices);

        match desire.as_ref() {
            "1" => {view_priority_lists(&mut data);},
            "2" => {create_priority_list(&mut data);},
            "back" => {break;},
            _ => {println!("Please enter a valid number!");},
        }
    }
}

fn view_priority_lists(data: &mut PrioData) {
    if data.lists.len() == 0 {
        println!("You have no lists!");
    }
    else {
        loop {
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
            if desire == "back" {
                break;
            }

            let mut index: usize = desire.parse().unwrap();
            index -= 1;
            view_list(&mut data.lists[index]);
        }
    }
}

fn view_list(list: &mut PrioList) {
    loop {
        println!("\n--{}--", list.name);

        for item in &list.items {
            println!("{}", item);
        }

        println!("\nEnter a new item");
        let new_item = read_line();

        if new_item == "back" {
            break;
        }

        list.add_item(&new_item);
    }
}

fn create_priority_list(data: &PrioData) {
    println!("create list!");
}

fn switch_board(prompt: &str, choices: Vec<String>) -> String {
    println!("\n{}", prompt);
    for choice in choices {
        println!("{}", choice);
    }

    let choice = read_line();

    choice.to_string()
}

fn read_line() -> String{
    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .ok()
        .expect("Failed to read line");

    line.trim().to_string()
}
