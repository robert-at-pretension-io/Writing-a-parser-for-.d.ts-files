use config::Config;
use std::collections::HashSet;
use std::fs;

use nom::{
    branch::alt,
    // character::is_alphanumeric
    bytes::complete::tag,
    bytes::complete::take_until,
    // see the "streaming/complete" paragraph lower for an explanation of these submodules
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::opt,
    combinator::recognize,
    error::ParseError,
    multi::{many0, many1},
    sequence::delimited,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::fmt::Debug;

use console::{Term, style};
use dialoguer::{theme::ColorfulTheme, MultiSelect};

fn main() {
    // Read in the configuration from config.json
    let hash_config = config_as_hash();

    // Get a some nice tools for manipulating the terminal
    let term = Term::stdout();


    // Clear out the terminal.
    term.clear_screen().unwrap();
    term.write_line("-----------Welcome!-----------\n").unwrap();

    // print out the keys and values in hash_config
    println!("The following values are set up in the config file:");
    for (key, value) in hash_config.iter() {
        println!("'{}' : '{}'", key, value);
    }

    term.write_line("\n-----------/Welcome!-----------\n").unwrap();


    let default_parse_option_text = format!("Parse the default file: ({})", style(hash_config.get("default_parse_file").unwrap()).cyan().bold().italic());
    let default_parse_option_function = Box::new(
      |filename : String| {
        let g = Graph::init(filename.as_str());
        println!("{:?}", g);
      }
    );

    // list out the files in the current directory
    let mut files = fs::read_dir("./src/").unwrap();
    // create a list of options for the user to select from based on the files in the directory
    let mut options = Vec::new();
    let mut option_functions = Vec::new();
    while let Some(file) = files.next() {
        let file = file.unwrap();
        let file_name = file.file_name().into_string().unwrap();
        if file_name.ends_with(".d.ts") {
            options.push(format!("Choose this file: ({})", style(file_name).cyan().bold().italic()));
            option_functions.push(Box::new(
              |filename : String| {
                let g = Graph::init(filename.as_str());
                println!("{:?}", g);
              }
            ));
        }
    }



    let choose_file_option_text = format!("Choose a file to parse: ({})", style(hash_config.get("default_parse_file").unwrap()).cyan().bold().italic());

    type OptionFunction = Box<dyn Fn(String)>;

    let functional_choices : Vec<(String, OptionFunction)> = vec![
        (default_parse_option_text, default_parse_option_function),
    ];

    // We do this just to guarantee that there will in fact be a paired function to the selected choice!
    let text_choices = functional_choices.iter().map(|(text, _)| text.to_string()).collect::<Vec<String>>();

    loop {
        let result = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .defaults(&[true])
            .items(&text_choices)
            .interact_opt();
            

        // Clear out the terminal.
        term.clear_screen().unwrap();

        match result {
            Ok(maybe_vector_of_choices) => match maybe_vector_of_choices {
                Some(vector_of_choices) => {
                    for index_choice in vector_of_choices.clone() {
                        // println!("You selected: {}", functional_choices[index_choice]);
                        functional_choices[index_choice].1(hash_config.get("default_parse_file").unwrap().to_string());
                    }
                    if vector_of_choices.is_empty() {
                        println!("You didn't select anything!");
                    }
                }
                None => {
                    println!("You quit!");
                }
            },
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    // println!("This folder has the following files: ");
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //   println!("Name: {}", path.unwrap().path().display())
    // }

    // if let Some(filename) = hash_config.get("default_parse_file") {
    //   let g = Graph::init(filename.clone().as_str());
    //   println!("{:?}", g);
    // }
}

fn config_as_hash() -> HashMap<String, String> {
    let config = Config::builder()
    .add_source(config::File::with_name("./src/config.json"))
    .build()
    .unwrap();
let hash_config = config.try_deserialize::<HashMap<String, String>>().unwrap();
hash_config
}

#[derive(Debug, Clone)]
struct Graph {
    edges: Vec<(String, String)>,
    elemental_types: HashSet<Type>,
    /// These types are used to determine if the node is primative or not. Elemental types are types like "string", "number", etc. They are the basic building blocks of the rest of the application. These will be initialized by a configuration file that loads when the program loads.
    primative_types: HashSet<Type>,
    composite_types: HashSet<Type>,
}

impl Graph {
    fn init(filename: &str) -> Self {
        // we first read the contents of the file into a string which we will parse.
        let contents = fs::read_to_string(filename)
            .expect("Unable to read file")
            .to_owned();

        // parse the contents
        let types = type_file(&contents);

        let mut graph = Graph {
            edges: Vec::new(),
            elemental_types: HashSet::new(),
            primative_types: HashSet::new(),
            composite_types: HashSet::new(),
        };

        // we then iterate through the types and add them to the graph.
        for type_ in types {
            if Self::is_primative(&type_) {
                graph.primative_types.insert(type_.clone());
            } else {
                graph.composite_types.insert(type_.clone());
            }
        }

        // we then iterate through the types and add the edges to the graph.

        // for type_ in types {
        //   for edge in type_.edges.iter() {
        //     graph.edges.push((type_, edge));
        //   }
        // }

        // passing the object back to the caller ALSO passes ownership of the object... That makes sense!
        graph
    }

    /// This function takes in a type and returns if the type is a primitive type.
    /// It does this by checking to see if the type is contained in the set of elemental_types
    fn is_primative(my_type: &Type) -> bool {
        // To implement this algorithm, we will need to check if my_type is contained in the elemental_types set.
        // todo!()
        true
    }
}

#[derive(Debug, Clone, Eq)]
struct Type {
    name: String,
    prop_type: HashMap<String, String>,
}

impl std::hash::Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Type {
    fn new() -> Type {
        Type {
            name: String::from(""),
            prop_type: HashMap::<String, String>::new(),
        }
    }



    fn add_name(&mut self, name: &str) {
        self.name = String::from(name);
    }
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
///Todo: look at imported types and recursively read those too
fn type_file(file: &str) -> Vec<Type> {
    let result = many0(
        //hypothetically, the file could contain no interfaces or types
        alt((
            preceded(
                take_until("interface"), //
                interface_block,
            ),
            preceded(
                take_until("type"), //
                interface_block,
            ),
        )),
    )(file);

    // need to look inside result to make sure it is a success
    match result {
        Ok((_, types)) => {
            // println!("{:?}", types);
            types
        }
        Err(e) => {
            println!("{:?}", e);
            Vec::new()
        }
    }
}

fn label_identifier(input: &str) -> IResult<&str, &str> {
    let (rest, m) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;
    Ok((rest, m))
}


fn interface_block(input: &str) -> IResult<&str, Type> {
  
    let result: IResult<&str, (&str, Vec<(&str, &str, &str, Option<&str>)>)> = preceded(
        ws(alt((tag("interface"), tag("type")))), // These typescript types can either be labeled with type or interface
        tuple((
            // name of the type/interface
            terminated(ws(alpha1), opt(ws(tag("=")))),
            delimited(
                ws(tag("{")),
                many1(
                    // there can of course be many properties with respective types
                    tuple((
                        ws(label_identifier), // property name
                        tag(":"),
                        ws(recognize(pair(alphanumeric1, opt(tag("[]"))))), // property type
                        opt(tag(";")),
                    )),
                ),
                ws(tag("}")),
            ),
        )),
    )(input);

    let my_type: &mut Type = &mut Type::new();

    match result {
        Ok(tupled) => {
            let (rest, (name, prop_type_hash)) = tupled.clone();
            my_type.add_name(name);
            // get all the properties and their types
            // grow two vectors by looping through prop_type_hash
            let mut prop_names = Vec::<String>::new();
            let mut prop_types = Vec::<String>::new();
            for (prop, _, type_name, _) in prop_type_hash {
                prop_names.push(prop.into());
                prop_types.push(String::from(type_name));
            }

            my_type.prop_type = prop_names.into_iter().zip(prop_types.into_iter()).collect();

            Ok((rest, my_type.to_owned()))
        }
        Err(err) => {
            println!("Awe shucks, we have an error: {:?}", err);
            Err(err)
        }
    }
}
