use std::fmt;
use std::mem;
use std::str::FromStr;
use std::num::ParseIntError;

/// Some events for testing purposes.
enum Event {    
    Add,
    Subtract,
}


/// A four unnamed element struct, no purpose.
struct Matrix(f32, f32, f32, f32);

struct Point {
    x : f32,
    y : f32,
}

/// Print Matrix m in the format ( m.0, m.1, m.2, m.3 )
impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "({} {} {} {})", self.0, self.1, self.2, self.3)
    }
}


/// transpose matrix
fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}


/// sum two signed integers
/// e.g. for fibonacci
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn print_array(array: &[f32]) {
    for i in 0..array.len() {
        println!("a[{}] = {}", i, array[i]);
    }
}

struct My32 {
    i : i32
}

impl FromStr for My32 {
    type Err = ParseIntError;

    fn from_str(s : &str) -> Result<Self, Self::Err> {
        let mut sum = 0_i32;

        for b in s.bytes() {
            sum += (b as i32) - ('0' as i32);
        }

        Ok ( My32 { i : sum } )
    }
}

/// main function is just for experimenting around.
fn main() {
    let flt_const = 1.9999_f32;
    let upper_left : Point = Point { x : 1.0, y : 2.0 };
    let mut last_point = Point { x : 0.0, y : 0.0 };
    let mut a = 1i32;
    let mut b = 1i32;
    let mut h;
    let mut b_test: bool = true;
    let mut f_test: f32 = 100.9;
    let names = vec!["Bob", "Frank", "Ferris"];
    let escaped = b"\x52\x75\x73\x74 as bytes";
    let many_elements: [f32; 5] = [1.0, 2.0, 3.1, 4.0, 5.0];
    
    println!("float 0.25 as uint32: {}", flt_const as u32);
    println!("String as int: {}", My32::from_str("0123").unwrap().i);

    println!("Point {} {}", upper_left.x, upper_left.y);
    println!("Point {} {}", last_point.x, last_point.y);
    last_point = Point { x : 5.5, .. upper_left };
    println!("Point {} {}", last_point.x, last_point.y);


    print_array(&many_elements[1 .. 3]);
    println!("array occupies {} bytes", mem::size_of_val(&many_elements));
    println!("1 - 2 = {} {}", i8::MIN, i16::MAX);
    
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: {:?}", matrix);
    println!("Matrix: {:?}", transpose(matrix));


    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);
    println!("{} {}", Event::Add as i32, Event::Subtract as i32);

    println!("fibo {}", a);

    for n in 0..8 {
        b_test = !b_test;
        f_test *= 0.1;
        if (b > 10) && (b < 20) {
            print!("najo")
        }
        println!("fibo {}", b);
        h = sum(a, b);
        a = b;
        b = h;
        println!("Hello, world {} {}: {}", n, f_test, b_test);
        println!("Many elements: {}", many_elements[n % 5])
    }

    for name in names.into_iter() {
        println!("das ist kck: {}", name)
    }
}
