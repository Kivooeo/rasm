#[derive(Debug)]
enum Types {
    Int(usize),
    Float(f64),
    Str(String),
}

pub struct Programm<'a> {
    // Name: Value
    variables: std::collections::HashMap<&'a str, Types>,

    // Name: Line where it starts
    functions: std::collections::HashMap<&'a str, usize>,

    // Source code itself, maybe we'll change it a little
    pub source: String,
}

impl<'a> Programm<'a> {
    // This method creates an instance of "runtime"?
    pub fn try_new(path: &str) -> Option<Self> {
        // Try to read file
        let source = std::fs::read_to_string(path);
        match source {
            // If it's successful then we are create an instance of our Self
            Ok(source) => Some(Self {
                variables: std::collections::HashMap::default(),
                functions: std::collections::HashMap::default(),
                source,
            }),
            // Otherwise just panic, simple
            Err(e) => panic!("{e:?}"),
        }
    }

    // This method will found all functions and main itself and full-filled functions HashMap
    // mut here requiers because we will mutating self.functions
    pub fn initialize(&'a mut self) {
        self.functions = self
            .source
            .lines()
            .enumerate()
            .filter_map(|x| match x.1.ends_with(":") {
                true => Some((x.1, x.0)),
                false => None,
            })
            .collect::<std::collections::HashMap<&'a str, usize>>();
        //
        //
        // dbg!(&self.functions);
        let start_point = *self.functions.get("main:").unwrap();
        self.source
            .lines()
            .skip(start_point + 1)
            .take_while(|x| !x.contains("ret;"))
            .enumerate()
            .for_each(|(line, x)| {
                match &x.trim().split([' ', ',', ';']).collect::<Vec<_>>()[..] {
                    ["mov", a, _, b, _] => {
                        self.variables.insert(&a, parse_string_to_types(b));
                    }
                    ["add", a, _, b, _] => match self.variables.contains_key(a) {
                        true => {
                            self.variables.insert(
                                &a,
                                // Types::Int(
                                //     self.variables.get(a).unwrap().to_int()
                                //         + b.parse::<usize>().unwrap(),
                                // ),
                                match self.variables.get(a) {
                                    Some(value) => match value {
                                        Types::Float(x) => match b.parse::<f64>() {
                                            Ok(value1) => Types::Float(x + value1),
                                            Err(_) => panic!(
                                                "value {b} is not numeric type, line: {}", line + 1
                                            ),
                                        },
                                        Types::Int(x) => match b.parse::<usize>() {
                                            Ok(value1) => Types::Int(x + value1),
                                            Err(_) => panic!(
                                                "value {b} is not numeric type, line: {}", line + 1
                                            ),
                                        },
                                        Types::Str(x) => panic!("you are trying to add string to something, line: {}", line + 1),
                                    },
                                    None => panic!("variable {a}, doesnt exist, line: {line}"),
                                },
                            );
                        }
                        false => (),
                    },
                    ["print", a, _] => {
                        println!("{}", self.variables.get(a).unwrap())
                    }
                    a => {
                        println!("non recognized command: {a:?}");
                    }
                }
                // Main logic of programm
            });
        // Main ends here
        // dbg!(&self.variables);
    }

    // This method will run main function
    // mut requeirs here because we will mutate self.variables
    // pub fn run(&'a mut self) {}
}

fn parse_string_to_types(s: &str) -> Types {
    match s {
        // To Int
        s if s.chars().all(|x| x.is_numeric()) => Types::Int(s.parse::<usize>().unwrap()),

        // To Flot
        s if s.chars().all(|x| x.is_numeric() || x == '.') => {
            Types::Float(s.parse::<f64>().unwrap())
        }

        // Otherwise, idk
        _ => Types::Str(s.to_string()),
    }
}

impl Types {
    fn to_int(&self) -> usize {
        // dbg!(&self);
        if let Types::Int(x) = self {
            *x
        } else {
            panic!("?")
        }
    }
    fn to_float(&self) -> f64 {
        if let Types::Float(x) = self {
            *x
        } else {
            panic!("?")
        }
    }
    fn to_str(&self) -> String {
        if let Types::Str(x) = self {
            x.to_owned()
        } else {
            panic!("?")
        }
    }
}

impl std::fmt::Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Float(x) => write!(f, "{x}"),
            Types::Int(x) => write!(f, "{x}"),
            Types::Str(x) => write!(f, "{}", x.replace("\"", "")),
        }
    }
}
