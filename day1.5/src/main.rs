use std::io::stdin;
use text_io::scan;

fn main() {
   let mut list1: Vec<i32> = Vec::new();
   let mut list2: Vec<i32> = Vec::new();

   for line in stdin().lines() {
        let (l1, l2);
        let line = line.unwrap();
        scan!(line.bytes() => "{} {}", l1, l2);
        list1.push(l1);
        list2.push(l2);
   }
   list1.sort();
   list2.sort();

   let mut similarity_sum = 0;
   for i in list1.iter() {
        similarity_sum += i * list2.iter().filter(|&x| x == i).count() as i32;
   }
   println!("{}", similarity_sum);
}
