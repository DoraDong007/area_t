// 定义一个可以计算面积的特质
trait Area {
    fn area(&self) -> f64;
}

// 为圆形实现特质
struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

// 为正方形实现特质
struct Square {
    side: f64,
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 为三角形实现特质
struct Triangle {
    base: f64,
    height: f64,
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 泛型函数，打印出实现了Area特质类型的面积
fn print_area<T: Area>(shape: &T) {
    println!("The area of the shape is {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.2 };
    let square = Square { side: 3.0 };
    let triangle = Triangle { base: 4.0, height: 5.0 };

    // 调用print_area函数打印出每个图形的面积
    print_area(&circle);
    print_area(&square);
    print_area(&triangle);
}

