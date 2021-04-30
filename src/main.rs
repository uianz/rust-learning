use std::fs::File;

fn main() {
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);

    // let fuxk = Fuxk {
    //     id: 1,
    //     name: "aaa".to_string(),
    //     money: 9,
    // };
    let fuxk = Fuxk::create(1, "aaa".to_string(), 9);
    println!("{:?}", fuxk);
    println!("fuck account is {}", fuxk.account());
    //Fuxk { id: 1, name: "aaa", money: 0 }

    let book = Book::Papery(1001);

    match book {
        Book::Papery(i) => {
            println!("{}", i);
        },
        Book::Electronic { url } => {
            println!("{}", url);
        }
    }

    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    //编译错误
    // fn longer(s1: &str, s2: &str) -> &str {
    //     if s2.len() > s1.len() {
    //         s2
    //     } else {
    //         s1
    //     }
    // }
    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str  {
        if s2.len() > s1.len() {
            s2
        } else {
            s1
        }
    }

    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

}

enum Book {
    Papery(u32),
    Electronic {url: String},
}


fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

//必须要有#[derive(Debug)]  才能输出一个完整的结构体
#[derive(Debug)]
struct Fuxk {
    id: i32,
    name: String,
    money: u32,
}

impl Fuxk {
    fn account(&self) -> u32 {
        self.money * self.money
    }
    fn create(id:i32, name:String, money:u32) -> Fuxk{
        Fuxk{id, name, money}
    }
}