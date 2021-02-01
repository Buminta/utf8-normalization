extern crate unicode_normalization;
extern crate regex;

use regex::Regex;
use unicode_normalization::UnicodeNormalization;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn tb_normalization(s: String) -> String {
    let c = string_to_static_str(s).clone().nfd().collect::<String>();
    let mut re = Regex::new(r"[\u0300-\u036f]").unwrap();
    let d = re.replace_all(string_to_static_str(c), "").replace("đ", "d").replace("Đ", "D").to_string();
    re = Regex::new(r"\s+").unwrap();
    let e = re.replace_all(string_to_static_str(d), " ");
    e.trim().to_string()
}

fn remove_special_characters(s: String) -> String {
    let mut re = Regex::new(r"[^0-9a-zA-Z]").unwrap();
    let d = re.replace_all(string_to_static_str(s), " ").to_string();
    re = Regex::new(r"\s+").unwrap();
    let e = re.replace_all(string_to_static_str(d), " ");
    e.trim().to_string()
}

pub trait TbNormalization {
    fn tb_normalization(&self) -> String; 
    fn remove_special_characters(&self) -> String;
}

impl TbNormalization for String {
    fn tb_normalization(&self) -> String {
        tb_normalization(self.clone())
    }
    fn remove_special_characters(&self) -> String {
        remove_special_characters(self.clone())
    }
}

impl<'a> TbNormalization for &'a str {
    fn tb_normalization(&self) -> String {
        tb_normalization(self.clone().to_string())
    }
    fn remove_special_characters(&self) -> String {
        remove_special_characters(self.clone().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_static_str() {
        let s = "số 22 ngách 63/30/16 lê đức thọ , mỹ đình 2  Được chưa nhỉ  --";
        assert_eq!(s.tb_normalization(), "so 22 ngach 63/30/16 le duc tho , my dinh 2 Duoc chua nhi --".to_string());
    }
    #[test]
    fn test_for_string() {
        let s = "số 22 ngách 63/30/16 lê đức thọ , mỹ đình 2  Được chưa nhỉ  --";
        assert_eq!(s.to_string().tb_normalization(), "so 22 ngach 63/30/16 le duc tho , my dinh 2 Duoc chua nhi --".to_string());
    }
    #[test]
    fn test_with_remove_special() {
        let s = "số 22 ngách 63/30/16 lê đức thọ , mỹ đình 2  Được chưa nhỉ  --";
        assert_eq!(s.tb_normalization().remove_special_characters(), "so 22 ngach 63 30 16 le duc tho my dinh 2 Duoc chua nhi".to_string());
    }
}
