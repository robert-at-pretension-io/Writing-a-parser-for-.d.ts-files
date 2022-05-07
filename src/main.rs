use config::Config;
use std::collections::HashSet;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::{fs, string};

use std::time::{Duration, Instant};

use std::collections::HashMap;
use std::fmt::{Debug, Display};

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

use std::process::Command;

#[tokio::main]
async fn main() {
    // Read in the configuration from config.json
    let hash_config = config_as_hash();

    let current_path = std::env::current_dir().unwrap();

    let term = Term::stdout();

    let parse_file = choose_file_from_submenu(
        "Choose the file to parse, this is also known as a test file.".to_string(),
        current_path.clone(),
        &term,
        &hash_config,
        &"default_parse_file".to_string()
    )
    .unwrap();

    let pest_grammar_file = choose_file_from_submenu(
        "Choose Pest File Parser File".to_string(),
        current_path,
        &term,
        &hash_config,
        &"default_pest_grammar_file".to_string()
    );

    // using the pest_grammar_file, we can parse the parse_file.

    //Any changes to the pest_grammar_file will be watched, recompiled with wasm and re-run against the test

    check_that_correct_build_tools_are_on_system();

    tokio::spawn(async move {
        check_if_pest_grammar_file_changed(pest_grammar_file.clone(), Duration::from_secs(1)).await
    });

    // In order for this to work correctly, the configuration file should also be edit-able and readable by both files.
    // try to compile file to wasm

    // Clear out the terminal.
    //term.clear_screen().unwrap();
    term.write_line("-----------Welcome!-----------\n").unwrap();

    // print out the keys and values in hash_config
    println!("The following values are set up in the config file:");
    for (key, value) in hash_config.iter() {
        println!("'{}' : '{}'", key, value);
    }

    term.write_line("\n-----------/Welcome!-----------\n")
        .unwrap();

    let default_parse_option_text = format!(
        "Parse the default file: ({})",
        style(hash_config.get("default_parse_file").unwrap())
            .cyan()
            .bold()
            .italic()
    );
    let default_parse_option_function = Box::new(|filename: String| {
        let g = Graph::init(filename.as_str());
        println!("{:?}", g);
    });

    type OptionFunction = Box<dyn Fn(String)>;

    let functional_choices: Vec<(String, OptionFunction)> =
        vec![(default_parse_option_text, default_parse_option_function)];

    // We do this just to guarantee that there will in fact be a paired function to the selected choice!
    let text_choices = functional_choices
        .iter()
        .map(|(text, _)| text.to_string())
        .collect::<Vec<String>>();

    loop {
        let result = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .defaults(&[true])
            .items(&text_choices)
            .interact_opt();

        // Clear out the terminal.
        //term.clear_screen().unwrap();

        match result {
            Ok(maybe_vector_of_choices) => match maybe_vector_of_choices {
                Some(vector_of_choices) => {
                    for index_choice in vector_of_choices.clone() {
                        // println!("You selected: {}", functional_choices[index_choice]);
                        functional_choices[index_choice].1(
                            hash_config.get("default_parse_file").unwrap().to_string(),
                        );
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
}

async fn check_if_pest_grammar_file_changed(
    src_filepath: Option<PathBuf>,
    check_every_duration: Duration,
) {
    use crc32fast::Hasher;

    // every (check_every_duration) seconds read the checksum of the file and determine if it's different from the old checksum

    let mut old_checksum = 0;

    loop {
        let mut hasher = Hasher::new();
        let mut file_string = String::new();
        let mut file = fs::File::open(src_filepath.as_ref().unwrap()).unwrap();
        file.read_to_string(&mut file_string).unwrap();
        hasher.update(file_string.as_bytes());
        let new_checksum = hasher.finalize();

        if new_checksum != old_checksum {
            println!("The file has changed!");

            use_cargo_to_compile_file_to_wasm(
                src_filepath.as_ref().unwrap().to_str().unwrap().to_string(),
            );

            println!("New checksum is: {}", new_checksum);
            old_checksum = new_checksum;
        }

        tokio::time::sleep(check_every_duration).await;
    }
}

fn config_as_hash() -> HashMap<String, String> {
    let config = Config::builder()
        .add_source(config::File::with_name("./src/config.json"))
        .build()
        .unwrap();
    config.try_deserialize::<HashMap<String, String>>().unwrap()
}

fn check_that_correct_build_tools_are_on_system() {
    let rust_target_triple = "wasm32-unknown-unknown";

    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .arg("--installed")
        .output()
        .expect("failed to execute process");

    // check to see if the target is already installed seeing if the target triple is in the output

    let output_string = String::from_utf8(output.stdout).unwrap();
    // println!("status: {}", output.status);
    // println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    // println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    if !output_string.contains(rust_target_triple) {
        println!(
            "You don't have the wasm target installed. Please run `rustup target add {}`",
            rust_target_triple
        );
        println!("The string only contains: {}", output_string);
        // std::process::exit(1);

        //use rustup to get the wasm32-unknown-unknown target
        Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .output()
            .expect("Couldn't launch the rustup command");
    } else {
        println!(
            "You have the wasm target '{}' installed! ✅ ✅ ✅ ",
            style(&rust_target_triple).green()
        );
    }
}
/// This is somewhat tricky because all of the files are relative to directory of the executable. Also, not sure how pathbufs work when there are relative files  🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷 🤷
fn choose_file_from_submenu(
    message_prompt: String,
    current_path: PathBuf,
    term: &Term,
    config: &HashMap<String, String>,
    default_file_config_name: &String,
) -> Option<PathBuf> {
    // if the default_file_config_name is in the config, use that as the default file and return that as the file buffer without asking
    if config.contains_key(default_file_config_name) {
        let file_buf = PathBuf::from(config.get(default_file_config_name).unwrap());
        return Some(file_buf);
    }

    println!("{}", &message_prompt);

    // if the current_path is a directory, return it as a the result
    if !current_path.is_dir() {
        return Some(current_path);
    }

    let initial_path = std::env::current_dir().unwrap();
    println!("Current Path: {}", initial_path.display());
    println!("To exit this menu, press esc. To select a file or subdirectory, press enter.");

    // save the files in a vector
    let mut files = Vec::new();

    //get parent directory
    let parent_dir = current_path.parent().unwrap();

    //add parent directory pathbuff to files
    files.push(parent_dir.to_path_buf());

    for entry in fs::read_dir(current_path).unwrap() {
        let entry = entry.unwrap();
        files.push(entry.path());
    }

    // provide these files as as choices in a menu
    let mut choices = Vec::new();

    for file in &files {
        choices.push(file.display().to_string());
    }

    // get the selected item
    let selected = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a file or directory:")
        .default(0)
        .items(&choices)
        .interact()
        .unwrap();

    // check if the selected file, if so return the filename
    if !files[selected].is_dir() {
        // if it is, return the selected file
        println!("You selected: {}", files[selected].display());
        return Some(files[selected].clone());
    }

    // clear the terminal
    term.clear_screen().unwrap();

    // instead of looping, we just do a recursive function call
    choose_file_from_submenu(
        message_prompt,
        files[selected].clone(),
        term,
        config,
        default_file_config_name,
    )
}

//derive display
#[derive(Debug)]
enum MyError {
    CargoPathMissing(String),
    ProgramDidntCompile(String),
}

fn use_cargo_to_compile_file_to_wasm(cargo_file_path: String) {
    // check that the cargo_file_path is a Cargo.toml file that exists
    let cargo_file_path_as_string = cargo_file_path.clone();
    let cargo_file_path_as_string_as_path = Path::new(&cargo_file_path_as_string);
    if !cargo_file_path_as_string_as_path.exists() {
        println!("The file {} does not exist", cargo_file_path_as_string);
        // return Err(MyError::CargoPathMissing(cargo_file_path_as_string));
    }

    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target=wasm32-unknown-unknown")
        .arg("--manifest-path")
        .arg(cargo_file_path)
        .output();

    match output {
        Err(err) => {
            println!(
                "The following error occurred while compiling the program: {}",
                err
            );
            // return Err(MyError::ProgramDidntCompile(err.to_string()))
        }
        Ok(ok_output) => {
            match ok_output.status.code() {
                Some(code) => {
                    if code == 0 {
                        // no errors!
                        println!("No errors! Should be able to run the generated wasm file against the test files.");
                        // return Ok(())
                    }
                }
                None => {
                    // not sure it can get here since the program will crash if there are errors...
                    println!("This will only show to the console if the program is terminated while the code is compiling");
                }
            }
        }
    }
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
        let contents = fs::read_to_string(filename).expect("Unable to read file");

        // parse the contents
        // let types = type_file(&contents);

        let mut graph = Graph {
            edges: Vec::new(),
            elemental_types: HashSet::new(),
            primative_types: HashSet::new(),
            composite_types: HashSet::new(),
        };

        // we then iterate through the types and add them to the graph.
        // for type_ in types {
        //     if Self::is_primative(&type_) {
        //         graph.primative_types.insert(type_.clone());
        //     } else {
        //         graph.composite_types.insert(type_.clone());
        //     }
        // }

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

// fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
//     inner: F,
// ) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
// where
//     F: FnMut(&'a str) -> IResult<&'a str, O, E>,
// {
//     delimited(multispace0, inner, multispace0)
// }
//Todo: look at imported types and recursively read those too
// fn type_file(file: &str) -> Vec<Type> {
//     let result = many0(
//         //hypothetically, the file could contain no interfaces or types
//         alt((
//             preceded(
//                 take_until("interface"), //
//                 interface_block,
//             ),
//             preceded(
//                 take_until("type"), //
//                 interface_block,
//             ),
//         )),
//     )(file);

//     // need to look inside result to make sure it is a success
//     match result {
//         Ok((_, types)) => {
//             // println!("{:?}", types);
//             types
//         }
//         Err(e) => {
//             println!("{:?}", e);
//             Vec::new()
//         }
//     }
// }

// fn label_identifier(input: &str) -> IResult<&str, &str> {
//     let (rest, m) = recognize(pair(
//         alt((alpha1, tag("_"))),
//         many0(alt((alphanumeric1, tag("_")))),
//     ))(input)?;
//     Ok((rest, m))
// }

// fn interface_block(input: &str) -> IResult<&str, Type> {
//     let result: IResult<&str, (&str, Vec<(&str, &str, &str, Option<&str>)>)> = preceded(
//         ws(alt((tag("interface"), tag("type")))), // These typescript types can either be labeled with type or interface
//         tuple((
//             // name of the type/interface
//             terminated(ws(alpha1), opt(ws(tag("=")))),
//             delimited(
//                 ws(tag("{")),
//                 many1(
//                     // there can of course be many properties with respective types
//                     tuple((
//                         ws(label_identifier), // property name
//                         tag(":"),
//                         ws(recognize(pair(alphanumeric1, opt(tag("[]"))))), // property type
//                         opt(tag(";")),
//                     )),
//                 ),
//                 ws(tag("}")),
//             ),
//         )),
//     )(input);

//     let my_type: &mut Type = &mut Type::new();

//     match result {
//         Ok(tupled) => {
//             let (rest, (name, prop_type_hash)) = tupled.clone();
//             my_type.add_name(name);
//             // get all the properties and their types
//             // grow two vectors by looping through prop_type_hash
//             let mut prop_names = Vec::<String>::new();
//             let mut prop_types = Vec::<String>::new();
//             for (prop, _, type_name, _) in prop_type_hash {
//                 prop_names.push(prop.into());
//                 prop_types.push(String::from(type_name));
//             }

//             my_type.prop_type = prop_names.into_iter().zip(prop_types.into_iter()).collect();

//             Ok((rest, my_type.to_owned()))
//         }
//         Err(err) => {
//             println!("Awe shucks, we have an error: {:?}", err);
//             Err(err)
//         }
//     }
// }
