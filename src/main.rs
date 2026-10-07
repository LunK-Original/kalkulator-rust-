use std::io::{self, Write};

fn pertambahan(angka1 :i128 ,  angka2 : i128 ) -> i128{
    angka1 + angka2
}

fn pengurangan(angka1 :i128 ,  angka2 : i128 ) -> i128{
    angka1 - angka2
}

fn perkalian(angka1 :i128 ,  angka2 : i128 ) -> i128{
    angka1 * angka2
}

fn pembagian(angka1 :i128 ,  angka2 : i128 ) -> i128{
    angka1 / angka2
}

fn main() {
    let mut perulangan : bool = false;
    let mut angka1 : String = String::new();
    let mut angka2: String = String::new();
    let mut operator:String = String::new();

 
    print!("angka pertama: ");
    io::stdout().flush().expect("gagal");
    io::stdin().read_line(&mut angka1).expect("gagal");
    let angka_pertama = angka1.trim().parse::<i128>().unwrap();

    print!("angka kedua: ");
    io::stdout().flush().expect("gagal");
    io::stdin().read_line(&mut angka2).expect("gagal");
    let angka_kedua= angka2.trim().trim().parse::<i128>().unwrap();

    println!("pilih operator");
    println!("1 . +");
    println!("2. -");
    println!("3. *");
    println!("4 . /");
    io::stdin().read_line(&mut operator).unwrap();
    let operator_jadi = operator.trim();

    if operator_jadi == "1"{
        println!("{}" ,pertambahan(angka_pertama, angka_kedua));
    }else if operator_jadi == "2"{
        println!("{}" , pengurangan(angka_pertama, angka_kedua));
    }else if operator_jadi == "3"{
        println!("{}" , perkalian(angka_pertama, angka_kedua));
    }else if operator_jadi == "4"{
        println!("{}" , pembagian(angka_pertama, angka_kedua));
    }else{
        println!("kamu salah pilih");
    }
}
