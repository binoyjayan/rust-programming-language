
// Visitor design pattern using compile time polymorphism
// Note that run time polymorphism uses 'dyn'

trait ShapeVisitor {
    fn visit_circle(&mut self, c: &Circle);
    fn visit_rectangle(&mut self, r: &Rectangle);
}

trait Shape {
    fn accept<V: ShapeVisitor>(&self, sv: &mut V);
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Circle {
    center: Point,
    radius: f64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Rectangle {
    lower_left: Point,
    upper_right: Point,
}

impl Shape for Circle {
    fn accept<V: ShapeVisitor>(&self, sv: &mut V) {
        sv.visit_circle(self);
    }
}

impl Rectangle {
    fn length(self) -> f64 { self.lower_left.x.abs() - self.upper_right.x.abs() }
    fn width(self) -> f64 { self.lower_left.y.abs() - self.upper_right.y.abs() }
}

impl Shape for Rectangle {
    fn accept<V: ShapeVisitor>(&self, sv: &mut V) {
        sv.visit_rectangle(self);
    }
}

// runtime dispatch: fn compute_area(s: &dyn Shape) -> f64 {
fn compute_area<S: Shape>(s: &S) -> f64 {
    struct AreaCalculator {
        area: f64,
    }

    impl ShapeVisitor for AreaCalculator {
        fn visit_circle(&mut self, c: &Circle) {
            self.area = std::f64::consts::PI * c.radius * c.radius;
        }
        fn visit_rectangle(&mut self, r: &Rectangle) {
            self.area = r.length() * r.width();
        }
    }
    
    let mut ac = AreaCalculator { area: 0.0 };
    s.accept(&mut ac);
    ac.area
}

fn main() {
    let p1 = Point{x:0.0, y:0.0};
    let p2 = Point{x:2.0, y:2.0};
    let rect = Rectangle {lower_left: p1, upper_right: p2};
    let circle = Circle {center: p1, radius: 3.0};

    let area_rect = compute_area(&rect);
    let area_circle = compute_area(&circle);
    println!("Area of rectangle: {}", area_rect);
    println!("Area of circle: {}", area_circle);
}


