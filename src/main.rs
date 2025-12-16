use regex::Regex;

#[derive(Clone, Debug)]
struct Item<'a> {
    hour: u8,
    minute: u8,
    name: String,
    source: &'a str,
}

impl<'a> Item<'a> {
    fn new(hour: u8, minute: u8, name: String, source: &'a str) -> Self {
        Self {
            hour,
            minute,
            name,
            source,
        }
    }
}

#[derive(Clone, Debug)]
struct Program<'a> {
    date: String,
    items: Vec<Item<'a>>,
}

impl<'a> Program<'a> {
    fn new(date: String, items: Vec<Item<'a>>) -> Self {
        Self { date, items }
    }
}

fn parse_date(linestr: &str) -> String {
    let tokens = linestr.split_whitespace();
    let tokencount = tokens.clone().count();
    assert!(
        tokencount == 2,
        "{} 不符合格式：播出日期 2025/12/15",
        linestr
    );
    tokens.last().unwrap().to_string()
}

#[derive(Debug)]
enum MyError {
    IoError(std::io::Error),
    Utf8Error(core::str::Utf8Error),
}

impl From<std::io::Error> for MyError {
    fn from(value: std::io::Error) -> Self {
        MyError::IoError(value)
    }
}

impl From<core::str::Utf8Error> for MyError {
    fn from(value: core::str::Utf8Error) -> Self {
        MyError::Utf8Error(value)
    }
}

fn parse(contents: String) -> Result<bool, MyError> {
    let list_re = Regex::new(r"^\b(\d{2})\b\s*:\s*\b(\d{2})(.*)").unwrap();

    let lines = contents.lines();

    let mut programs: Vec<Program> = vec![];
    let mut program = Program::new("".to_string(), Vec::new());

    for line in lines {
        /*
        if line.trim().starts_with("播出日期") {
            let line = line.trim();
            if !program.items.is_empty() {
                programs.push(program.clone());
                program = Program::new("".to_string(), Vec::new());
            }

            program.date = parse_date(line);
        } else if let line_trimed = line.trim()
            && list_re.is_match(line_trimed)
        {
            let Some((_, [hour, minute, name])) =
                list_re.captures(line_trimed).map(|caps| caps.extract())
            else {
                panic!("Invalid format");
            };
            let hour = hour.parse::<u8>().expect("Invalid hour");
            let minute = minute.parse::<u8>().expect("Invalid minute");
            let name = name.trim();
            assert!(hour < 24, "{hour}");
            assert!(minute < 60, "{minute}");
            assert!(!name.is_empty(), "{name}");
            let item = Item::new(hour, minute, name.to_string(), line);
            program.items.push(item);
        } else {
            println!("throw the useless line: \n{}", line);
        }
        */

        match line {
            line if line.trim().starts_with("播出日期") => {
                let line = line.trim();
                if !program.items.is_empty() {
                    programs.push(program.clone());
                    program = Program::new("".to_string(), Vec::new());
                }

                program.date = parse_date(line);
            }
            line if line.trim() != "" && list_re.is_match(line.trim()) => {
                let line_trimed = line.trim();
                let Some((_, [hour, minute, name])) =
                    list_re.captures(line_trimed).map(|caps| caps.extract())
                else {
                    panic!("Invalid format");
                };
                let hour = hour.parse::<u8>().expect("Invalid hour");
                let minute = minute.parse::<u8>().expect("Invalid minute");
                let name = name.trim();
                assert!(hour < 24, "{hour}");
                assert!(minute < 60, "{minute}");
                assert!(!name.is_empty(), "{name}");
                let item = Item::new(hour, minute, name.to_string(), line);
                program.items.push(item);
            }
            _line => {
                // println!("throw the useless line: \n{}", line);
            }
        }
    }
    programs.push(program.clone());

    for program in programs {
        println!("date: {}", program.date);
        program.items.iter().for_each(|item| {
            println!("{}", item.source);
            println!("{},{},{}", item.hour, item.minute, item.name);
        });
    }

    Ok(true)
}

fn parse_file(path: &str) -> Result<bool, MyError> {
    let contents = std::fs::read_to_string(path)?;
    parse(contents)
}

fn main() {
    let path = "1.txt";
    match parse_file(path) {
        Ok(v) => println!("{:?}", v),
        Err(e) => match e {
            MyError::IoError(error) => {
                println!("io error: {}", error);
            }
            MyError::Utf8Error(utf8_error) => {
                println!("{}", utf8_error);
            }
        },
    }
}
