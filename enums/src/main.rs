enum MyOption<T> {
    Some(T),
    None
}

impl MyOption<u32> {
    fn add(&self, other: u32) -> MyOption<u32> {
        match self {
            MyOption::Some(x) => MyOption::Some(x+other),
            MyOption::None => MyOption::None,
        }
    }
} 

fn print_option_u32(option_u32: MyOption<u32>) {
    match option_u32 {
        MyOption::Some(x) => println!("{}", x),
        MyOption::None => println!("None"),
    };
}

fn print_only_if_not_none(option_u32: MyOption<u32>) {
    if let MyOption::Some(x) = option_u32 {
        println!("Some {}", x);
    }
}

fn main() {
    print_option_u32(MyOption::Some(10));
    print_option_u32(MyOption::None);
    print_only_if_not_none(MyOption::Some(1024));
    print_only_if_not_none(MyOption::None);
    print_option_u32(MyOption::Some(10).add(10));
    print_option_u32(MyOption::None.add(10));
}
