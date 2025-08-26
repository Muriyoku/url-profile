use crate::url::UrlProfile;
use std::collections::HashMap;

pub struct UrlPocket {
    pub urls: HashMap<String, HashMap<String, UrlProfile>>,
}

pub fn show_profiles(pocket: &UrlPocket) {
    for (_p_key, p_value) in &pocket.urls {
        for (_u_key, u_value) in p_value {
            u_value.show_profile();
        }
    }
}
