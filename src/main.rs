// #[derive(Debug)]
enum Element {
    Integer(i32),
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let mut v_new: Vec<Element> = Vec::new();
    v_new.push(Element::Integer(10));
    v_new.push(Element::Int(100));
    v_new.push(Element::Float(10.11));
    v_new.push(Element::Text(String::from("Super")));

    for element in &v_new {
        match element {
            Element::Integer(value) => println!("{}", value),
            Element::Int(value) => println!("{}", value),
            Element::Float(value) => println!("{}", value),
            Element::Text(value) => println!("{}", value),
        }
    }

    // println!("{:?}", v_new);
    let v_t = vec![
        Element::Integer(1),
        Element::Int(100),
        Element::Float(10.11),
        Element::Text(String::from("Super")),
    ];

    for element in &v_t {
        match element {
            Element::Integer(value) => println!("{}", value),
            Element::Int(value) => println!("{}", value),
            Element::Float(value) => println!("{}", value),
            Element::Text(value) => println!("{}", value),
        }
    }
    // println!("{:?}", v_t)
}
