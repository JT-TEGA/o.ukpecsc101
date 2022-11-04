fn main() {
   let name = "Aisha lawal";
   let uni:&str = "Pan-Atantic-University";
   let addr:&str = "Km 52 Lekki-Epe Expressway, Ibeju-Lekki, lagos";
   println!("Name: {}", name);
   println!("University: {}, \nAddress: {}",uni,addr);



   let department:&'static str = "Computer science";
   let school:&'static str = "School of science and Technology";
   println!("Department: {}, \nSchool: {}",department,school);
}