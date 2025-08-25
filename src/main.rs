use std::collections::HashMap;
use std::io;
use std::process;

struct UrlProfile {
    path: String,
    domain: String,
    category: String, 
}

struct UrlPocket {
    urls: HashMap<String, HashMap<String, UrlProfile>>
}

impl UrlProfile {
    fn show_profile(&self) {
        println!(
            "Domain: {}, path: {}, category: {}", 
            self.domain, self.path, self.category
        );
    }
}

fn main() {
    let mut pocket: UrlPocket = UrlPocket { urls: HashMap::new() };
    
    loop {
        let mut pocket_name: String = String::new();
        let mut action: String = String::new();

        println!("=> Type some of these commands: add, exit, show-all or get-url");

        io::stdin().read_line(&mut action).expect("Invalid Input");
        
        if action.trim().is_empty() {
            println!("Bad input");
            process::exit(1);
        }

        if action.trim() == "exit" {
            println!("Quiting of program");
            process::exit(0);
        };

        if action.trim() == "add" {
            let mut category: String = String::new();
            let mut domain: String = String::new();
            let mut path: String = String::new(); 
            let mut url: UrlProfile = UrlProfile { 
                path: String::new(), 
                domain: String::new(), 
                category: String::new(),
            };
    
            println!("=> Please, type the domain of url");
            io::stdin().read_line(&mut domain).expect("Invalid input");
            
            println!("=> Please, type the path of url");
            io::stdin().read_line(&mut path).expect("Invalid Input");
    
            println!("=> Please, type the category of url");
            io::stdin().read_line(&mut category).expect("Invalid Input");
    
            println!("=> Please, add to a pocket");
            io::stdin().read_line(&mut pocket_name).expect("Invalid Input");
    
            url.category = category.trim().to_string();
            url.domain = domain.trim().to_string();
            url.path = path.trim().to_string();
    
            pocket.urls.entry(pocket_name.trim().to_string())
            .or_default()
            .entry(domain.trim().to_string())
            .or_insert_with(|| url);

            continue;
        };

        if action.trim() == "show-all" {
            for (_p_key, p_value) in &pocket.urls {
                for (_u_key, u_value) in p_value {
                    u_value.show_profile();
                };
            };
        };

        if action.trim() == "get-url" {
            let mut pocket_target: String = String::new();
            let mut url_target: String = String::new();

            println!("=> Please, type the pocket name");
            io::stdin().read_line(&mut pocket_target).expect("Invalid Input");

            println!("=> Please, type the url domain name");
            io::stdin().read_line(&mut url_target).expect("Invalid Input");

            let pocket_op = pocket.urls.get(pocket_target.trim());

            match pocket_op {
                Some(url) => {
                    match url.get(url_target.trim()) {
                        Some(profile) => profile.show_profile(),
                        None => panic!("The profile does not exists"),
                    }
                },
                None => panic!("The pocket does not exists"),
            }
        }

        continue;
    };
}
