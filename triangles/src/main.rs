use std::io;

struct Jo{
    a : f32,
    b : f32,
    h : f32
}
impl Jo{
    fn area(&self) -> f32{
        self.h * self.a * self.b
    }

}


fn main() {
    loop {
        println!("First triangle:");


        let mut r = String::new();
        println!("please enter h triangle");
        io::stdin().read_line(&mut r).expect("Error");
        let r: f32 = r.trim().parse().expect("Error");

        let mut o = String::new();
        println!("please enter first side triangle");
        io::stdin().read_line(&mut o).expect("Error");
        let o: f32 = o.trim().parse().expect("Error");

        let mut s = String::new();
        println!("please enter second side triangle");
        io::stdin().read_line(&mut s).expect("Error");
        let s: f32 = s.trim().parse().expect("Error");

        let tr = Jo {
            a: s,
            b: o,
            h: r
        };


        println!("S = {}", tr.area());

        let mut ms = String::new();
        println!("If you want to find the area of two triangle's , enter 1 \n In the otherwise enter 2");

        io::stdin().read_line(&mut ms).unwrap();
        let m : i32 = ms.trim().parse().unwrap();
        match m {
            1 => {
                println!("Second triangle");
                let mut i = String::new();
                println!("please enter first side triangle");
                io::stdin().read_line(&mut i).expect("Error");
                let i: f32 = i.trim().parse().expect("Error");

                let mut x = String::new();
                println!("please enter second side triangle");
                io::stdin().read_line(&mut x).expect("Error");
                let x: f32 = x.trim().parse().expect("Error");

                let mut y = String::new();
                println!("please enter h triangle");
                io::stdin().read_line(&mut y).expect("Error");
                let y: f32 = y.trim().parse().expect("Error");

                let tr2 = Jo {
                    a: i,
                    b: x,
                    h: y
                };
                println!("S2 = {}", tr2.area());

                let mut qs = String::new();
                println!("If you want to check whether the second triangle will fit into the first , \
                or the first into the second, enter 1 \nIn the otherwise enter 2  ");
                io::stdin().read_line(&mut qs).expect("ERROR WITH THIRD ITEM");
                let q : i32 = qs.trim().parse().unwrap();
                match q{
                    1 => {
                        if tr.area() > tr2.area() {
                            println!("second triangle included in first")
                        } else if tr.area() < tr2.area() {
                            println!("first triangle included in second")
                        } else if tr.area() == tr2.area(){
                            println!("the areas of the triangles are equal")
                        }
                    },
                    2 => break,
                    _ => {},
                }

            },
            2 => break,
            _ => {},
        }

    }
}