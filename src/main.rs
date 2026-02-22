fn main() {
/*   let name = "Peirnger Akos";
     println!("A name valtozo erteke: {name}");
     let mut kor = 40;
     println!("A mostani korom :{kor}");
     kor = 43;
     println!("Hazudtam, korom valojaban: {kor}");*/
    let x: i32 = -88;
    let _y: u32 = 23; // _ kell a nem hasznalt valtozo ele, kulonben a fordito szol
    let pi: f64 = 3.14159;
    let _is_rust_cool: bool = true;
    let _betu: char = 'R';
    let _sziv: char = '❤';
    
/*    let mut sum = (x as f64) + pi;
    println!("Az x+pi (lebegopontos) eredmenye = {sum}");
    sum = x + (pi as i32);
    println!("Egész számú eredmény: {sum}"); // Ha a pi 3.14 volt, itt csak 3-at fog hozzáadni!*/
    let sum = (x as f64) + pi;
    println!("Lebegőpontos eredmény: {sum}");
    let sum = x + (pi as i32);
    println!("Egész számú eredmény: {sum}"); // Ha a pi 3.14 volt, itt csak 3-at fog hozzáadni!


    let varos: (&str, i32) = ("Tapolca",13500);
    println!("Varos: {}, Lakossag: {}", varos.0, varos.1);
    let tomb: [i32; 5] = [10,20,30,40,50];
    println!("A harmadik szam: {}", tomb[2]); // nullatol indul az indexeles
//    println!("A tomb 10. eleme pedig: {}", tomb[9]); //ez nem mukodhet                                               
}
