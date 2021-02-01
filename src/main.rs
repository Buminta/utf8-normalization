extern crate tb_normalization;
use tb_normalization::unicode::TbNormalization;

fn main() {
  let s = "số 22 ngách 63/30/16 lê đức thọ , mỹ đình 2  Được chưa nhỉ  --";
  println!("{}", s.tb_normalization());
  println!("{}", s.tb_normalization().remove_special_characters());
}
