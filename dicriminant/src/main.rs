use std::io;

fn main() {
    loop {
        let mut a_str = String::new();
        let mut b_str = String::new();
        let mut c_str = String::new();

        println!("Please enter a");
        io::stdin().read_line(&mut a_str).expect("Error with input a");

        println!("Please enter b");
        io::stdin().read_line(&mut b_str).expect("Error with input b");

        println!("Please enter c");
        io::stdin().read_line(&mut c_str).expect("Error with input c");

        let a: f32 = a_str.trim().parse().unwrap();
        let b: f32 = b_str.trim().parse().unwrap();
        let c: f32 = c_str.trim().parse().unwrap();

        let mut es = String::new();
        println!("EXAMINATION! \n a = {a} \n b = {b} \n c = {c} \n IF EVERYTHING IS CORRECT , ENTER 1 \n OTHERWISE ENTER 2");

        io::stdin().read_line(&mut es).expect("ERROR WITH EXAMINATION");
        let e = es.trim().parse().unwrap();
        match e{
            1 => { let d = (b * b) - 4.0 * (a * c);
                if d > 0.0 {
                    let x1 = ((-1.0 * b) + d.sqrt()) / (2.0 * a);
                    let x2 = ((-1.0 * b) - d.sqrt()) / (2.0 * a);
                    println!("D = {} \nx1 = {} \nx2 = {}", d, x1, x2)
                } else if d == 0.0 {
                    let x = (-1.0 * b) / (2.0 * a);
                    println!("D = 0 \nx = {}", x)
                } else if d < 0.0 {
                    println!("THIS EQUATION HAS NO ROOTS, BECAUSE IT'S D = 0")
                }
            }
            2 => break,
            _ => { println!("Error unknown")}
        }

    }
}