use std::collections::HashMap;
use repl_rs::{Command, 
              // Error, 
              Parameter, Result, Value};
use repl_rs::{Convert, Repl};

use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::char,
  combinator::recognize,
  bytes::complete::is_not,
  // character::is_alphanumeric
  bytes::complete::tag,
  multi::many0,
  branch::alt,
};

fn unquote(input: &str) -> IResult<&str, Vec<&str>> {
        delimited(char('"'), many0(is_not("\"")), char('"'))(input)
  
}


// Add two numbers.
fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
   let first: i32 = args["first"].convert()?;
   let second: i32 = args["second"].convert()?;

   Ok(Some((first + second).to_string()))
}

// Parse the input between the double quotes
fn parse<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
  let unparsed_text : String = args["unparsed_text"].convert()?;

  match unquote(&unparsed_text) {
    Ok(str) => {
      Ok(Some( format!("Matched:'{}'\nRest:'{}'",str.1.join(""), str.0)))
    }
    Err(err) => {
      // repl_rs::Error::IllegalRequiredError("uhhh")
      Ok(Some(err.to_string()))
      
    }
  }
  


}

// Write "Hello"
// fn hello<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
//    Ok(Some(format!("Hello, {}", args["who"])))
// }

fn main() -> Result<()> {
   let mut repl = Repl::new(())
       .with_name("Parsing Example")
       .with_version("v0.1.0")
       .with_description("Using the latest version of nom (currently 7.1.0) to parse .d.ts files.")
       .add_command(
           Command::new("add", add)
               .with_parameter(Parameter::new("first").set_required(true)?)?
               .with_parameter(Parameter::new("second").set_required(true)?)?
               .with_help("Add two numbers together"),
       )
     .add_command(Command::new("parse", parse).
                  with_parameter(Parameter::new("unparsed_text").set_required(true)?)?
                  .with_help("Make sure to surround the unparsed text with double quotes!"), );
                  
       // .add_command(
       //      Command::new("hello", hello)
       //          .with_parameter(Parameter::new("who").set_required(true)?)?
       //          .with_help("Greetings!"),
   // );
   repl.run()
}