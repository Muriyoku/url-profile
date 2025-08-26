use std::io;
use crate::{url::UrlPocket};
pub struct UrlProfile {
    pub path: String,
    pub domain: String,
    pub category: String,
}

impl UrlProfile {
    pub fn show_profile(&self) {
      println!(
        "Domain: {}, path: {}, category: {}",
        self.domain, self.path, self.category
      );
    }
}

pub fn add_url_profile(pocket: &mut UrlPocket, pocket_name: &mut String) {
    let mut category: String = String::new();
    let mut domain: String = String::new();
    let mut path: String = String::new();
    let mut url = UrlProfile {
        path: String::new(),
        domain: String::new(),
        category: String::new(),
    };

    println!("=> Please, type the domain of url");
    bind_user_input(&mut domain);

    println!("=> Please, type the path of url");
    bind_user_input(&mut path);

    println!("=> Please, type the category of url");
    bind_user_input(&mut category);

    println!("=> Please, add to a pocket");
    bind_user_input(pocket_name);

    url.category = category.trim().to_string();
    url.domain = domain.trim().to_string();
    url.path = path.trim().to_string();

    pocket
        .urls
        .entry(pocket_name.trim().to_string())
        .or_default()
        .entry(domain.trim().to_string())
        .or_insert_with(|| url);
}

pub fn bind_user_input(target: &mut String) {
  io::stdin().read_line(target)
  .expect("Invalid Input");
}