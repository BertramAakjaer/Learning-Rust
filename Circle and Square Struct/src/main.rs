struct Rectangle {
    width: f64,
    height: f64,
    xcord: i32,
    ycord: i32
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height;
    }

    fn scale(&mut self, scaler: f64) {
        self.width *= scaler;
        self.height *= scaler;
    }

    fn new(w: f64, h: f64, x: i32, y: i32) -> Rectangle {
        Rectangle {
            width: w,
            height: h,
            xcord: x,
            ycord: y
        }
    }
}

struct Circle {
    radius: f64,
    xcord: i32,
    ycord: i32
}

impl Circle {
    fn get_area(&self) -> f64 {
        return self.radius.powf(2.0) * std::f64::consts::PI;
    }

    fn scale(&mut self, scaler: f64) {
        self.radius *= scaler;
    }

    fn new(r: f64, x: i32, y: i32) -> Circle {
        Circle {
            radius: r,
            xcord: x,
            ycord: y
        }
    }
}

fn main() {
    let mut rect: Rectangle = Rectangle::new(10.2, 20.1, 3, 22);
    let mut circle1: Circle = Circle::new(10.4, 22, 41);

    println!("Square's area before scale is {:.3} and circle's is {:.3}", rect.get_area(), circle1.get_area());

    rect.scale(10.3);
    circle1.scale(9.6);

    println!("Square's area after scale is {:.3} and circle's is {:.3}", rect.get_area(), circle1.get_area());

    println!("Distancte between the rectangle and circle is: {:.3}", get_distance(rect, circle1));
}

fn get_distance(r: Rectangle, c: Circle) -> f64 {
    return f64::sqrt((r.xcord as f64 - c.xcord as f64).powf(2.0) + (r.ycord as f64 - c.ycord as f64).powf(2.0));
}