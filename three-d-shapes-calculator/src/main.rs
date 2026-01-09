use std::f64::consts::PI;
use semire_read::Readable;

// --- Shapes ---
struct Trapezium {
    height: f64,
    base1: f64,
    base2: f64,
}

impl Trapezium {
    fn area(&self) -> f64 {
        self.height / 2.0 * (self.base1 + self.base2)
    }
}

struct Rhombus {
    d1: f64,
    d2: f64,
}

impl Rhombus {
    fn area(&self) -> f64 {
        0.5 * self.d1 * self.d2
    }
}

struct Parallelogram {
    base: f64,
    altitude: f64,
}

impl Parallelogram {
    fn area(&self) -> f64 {
        self.base * self.altitude
    }
}

struct Cube {
    side: f64,
}

impl Cube {
    fn surface_area(&self) -> f64 {
        6.0 * self.side * self.side
    }
}

struct Cylinder {
    radius: f64,
    height: f64,
}

impl Cylinder {
    fn volume(&self) -> f64 {
        PI * self.radius * self.radius * self.height
    }
}

fn main() {
    println!("Select an option:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Surface Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::read();

    match choice.as_str()
     {
        "1" => {
            println!("Enter height:");
            let h = f64::read();
            println!("Enter base 1:");
            let b1 = f64::read();
            println!("Enter base 2:");
            let b2 = f64::read();

            let trap = Trapezium { height: h, base1: b1, base2: b2 };
            println!("Area of Trapezium = {}", trap.area());
        }

        "2" => {
            println!("Enter diagonal 1:");
            let d1 = f64::read();
            println!("Enter diagonal 2:");
            let d2 = f64::read();

            let rhom = Rhombus { d1, d2 };
            println!("Area of Rhombus = {}", rhom.area());
        }

        "3" => {
            println!("Enter base:");
            let base = f64::read();
            println!("Enter altitude:");
            let altitude = f64::read();

            let para = Parallelogram { base, altitude };
            println!("Area of Parallelogram = {}", para.area());
        }

        "4" => {
            println!("Enter side length:");
            let side = f64::read();

            let cube = Cube { side };
            println!("Surface Area of Cube = {}", cube.surface_area());
        }

        "5" => {
            println!("Enter radius:");
            let r = f64::read();
            println!("Enter height:");
            let h = f64::read();

            let cyl = Cylinder { radius: r, height: h };
            println!("Volume of Cylinder = {}", cyl.volume());
        }

        _ => println!("Invalid choice"),
    }
}
