/*fn main() {
// 1.lecke valtozok
    let name = "Peirnger Akos";
    println!("A name valtozo erteke: {name}");
    let mut kor = 40;
    println!("A mostani korom :{kor}");
    kor = 43;
    println!("Hazudtam, korom valojaban: {kor}");
//  2.lecke valtozo tipusok
    let x: i32 = -88;
    let _y: u32 = 23; // _ kell a nem hasznalt valtozo ele, kulonben a fordito szol
    let pi: f64 = 3.14159;
    let _is_rust_cool: bool = true;
    let _betu: char = 'R';
    let _sziv: char = '❤';
//  
    let sum = (x as f64) + pi;
    println!("Lebegőpontos eredmény: {sum}");
    let sum = x + (pi as i32);
    println!("Egész számú eredmény: {sum}"); // Ha a pi 3.14 volt, itt csak 3-at fog hozzáadni!
//  3.lecke osszetett valtozok, tombok
    let varos: (&str, i32) = ("Tapolca",13500);
    println!("Varos: {}, Lakossag: {}", varos.0, varos.1);
    let tomb: [i32; 5] = [10,20,30,40,50];
    println!("A harmadik szam: {}", tomb[2]); // nullatol indul az indexeles
//    println!("A tomb 10. eleme pedig: {}", tomb[9]); //ez nem mukodhet                                               
}*/

//   4. lecke fugvenyek

use core::f64;

fn teglalap_terulete(szelesseg: f64, magassag: f64) -> f64 {
    szelesseg*magassag // Nincs pontosvesszo = ezt adja vissza!
}

fn koszontes(nev: &str){
    println!("Szia, {}! Orulok, hogy ma is tanulsz.", nev);
}

fn negyzet_kerulete(oldal: f64) -> f64 {
    oldal * 4.0
}

fn main() {
    //  1. Udvozles meghivasa
    koszontes("Akos");

    //  2. Terulet szamitas
    let a = 5.5;
    let b = 10.0;
    let terulet = teglalap_terulete(a, b);

    println!("A {a} es {b} oldalu teglalap terulete: {terulet}");

    // 3. kerulet szamitas
    let c = 12.5;
    let kerulet = negyzet_kerulete(c);
    println!("A negyzet keruletenek erteke: {kerulet}");

}

