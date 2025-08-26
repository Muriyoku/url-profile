use std::collections::HashMap;
use std::process;

mod url;

fn main() {
    let mut pocket = url::UrlPocket { urls: HashMap::new() };
    
    loop {
        let mut pocket_name: String = String::new();
        let mut action: String = String::new();
        println!("=> Type some of these commands: add, exit, show-all or get-url");
        
        url::bind_user_input(&mut action);

        if action.trim().is_empty() {
            println!("Bad input");
            process::exit(1);
        }

        if action.trim() == "exit" {
            println!("Quiting of program");
            process::exit(0);
        };

        if action.trim() == "add" {
            url::add_url_profile(&mut pocket, &mut pocket_name);

            continue;
        };

        if action.trim() == "show-all" {
            url::show_profiles(&pocket);

            continue;
        };

        if action.trim() == "get-url" {
            let mut pocket_target: String = String::new();
            let mut url_target: String = String::new();

            println!("=> Please, type the pocket name");
            url::bind_user_input(&mut pocket_target);

            println!("=> Please, type the url domain name");
            url::bind_user_input(&mut url_target);

            let pocket_op: Option<&HashMap<String, url::UrlProfile>> = pocket.urls.get(pocket_target.trim());

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