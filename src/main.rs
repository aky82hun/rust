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

/*fn teglalap_terulete(szelesseg: f64, magassag: f64) -> f64 {
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

//  5. feladat if-else es loop,while,for
fn main (){

let szam = 7;
if szam < 10 {
println!("Egyjegyu szam.");
} else if szam < 100 {
println!("Ketjegyu szam.");
} else {
println!("Nagy szam.");
}

let feltetel = true;
let szam = if feltetel {5} else {6};
println!("A feltetel if ag eredmenye utan a szam : {szam}");

// Számtartomány: 1-től 5-ig (a 6 már nem tartozik bele)
for szam in 1..6 {
println!("{szam}...");
}

// Végighaladás egy tömbön:
let kosar = ["alma", "körte", "szilva"];
for gyumolcs in kosar {
println!("A kosárban van egy: {gyumolcs}");
}

let tomb = [12 , 25, 66, 82, 23];
for i in tomb {
println!("{i}...");
if paros_e(i) {
println!("Ez a paros szamos");
} else {
println!("Ez a szam paratlan");
}
}

for j in 1..11 {
if oszthato_harommal(j) {
println!("Ez a szam oszthato harommal : {j}");
}
}
}
fn paros_e (szam: i32) -> bool { szam % 2 == 0 }
//    if szam % 2 == 0 { true  // ide mehetne csak szam % 2 == 0 az if helyett 
//    } else { false }
//}

fn oszthato_harommal (szam: i32) -> bool { szam % 2 == 0 }
//    if szam % 3 == 0 { true
//    } else { false }
//}
*/

// 6. feladat Ownership

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // itt az adatok atkoltoznek az s2-be

    // println!("{s1}"); Ez hibat dob hisz az adatok mar az s2-ben vannak
    println!("{s2}");

    let text = String::from("Teszt szoveg amivel meghivom a fuggvenyt");
    hossz_kiiras(&text); // itt is & jelzi hogy csak kolcson adom 
                         //println!("{text}"); itt sem megy mar az ujboli kiiratas az ownership miatt
    println!("{text}"); //igy mar megy mivel & kolcsonbe ment csak at
    elso_karakter(&text);
}

fn hossz_kiiras(szoveg: &String) {  //A & miatt csak kolcson veszi az erteket , 
    println!("A szoveg amit megadtam : {} es a hossza : {}",szoveg,szoveg.len());
}

fn elso_karakter(szoveg2: &String) {
    println!("A szoveg elso karaktere : {}",szoveg2.chars().next().unwrap());
}
