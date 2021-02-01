# UTF8Normalizer 

``` rust
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
```
