use std::io;

use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::{char, alpha1, multispace0},
  combinator::recognize,
  bytes::complete::is_not,
  // character::is_alphanumeric
  bytes::complete::tag,
  multi::many1,
  branch::alt,
  combinator::opt,
  sequence::{tuple, preceded},
  error::ParseError,
};
use std::fmt::Debug;
use std::collections::HashMap;

fn main() {
loop {
println!("Enter a string to be parsed (or type exit to quit):");
   let mut unparsed = String::new();
   io::stdin().read_line(&mut unparsed).expect("failed to readline");
  unparsed = unparsed.trim_end().to_string();
  
  if unparsed.contains("exit") {
    break
  }
  
  parse2(&unparsed, &[ interface_block]);

}
  
}

#[derive(Debug)]
struct Type {
  name: String,
  prop_type : HashMap<String,String>
}

impl Type {
  fn new() -> Type {
    Type{
    name : String::from(""),
    prop_type : HashMap::<String,String>::new()  
    }
    
  }
  /// This function doesn't check to see that the property is already inside the type.. This should conform to the rust standard
  fn add_property(&mut self, property: &str, my_type: &str) {
   
  self.prop_type.insert(String::from(property), String::from(my_type));
  }

  fn add_name(&mut self, name: &str) {
    self.name = String::from(name);
  }
}

// fn unquote(input: &str)  {
//        let result =  delimited(char('"'), is_not("\""), char('"'))(input);

//   if let Ok(str) = result {
//     println!("result: {:?}", str)
//   }
  
// }


fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    multispace0,
    inner,
    multispace0
  )
}


fn interface_block(input: &str) {
  let result : IResult<&str,(&str, Vec<(&str,&str,&str, Option<&str>)>)>= 
    preceded(
      ws(alt((tag("interface"), tag("type")))), // These typescript types can either be labeled with type or interface
      tuple(
      (
        ws(alpha1), // name of the type/interface
        delimited(
        ws(tag("{")), 
        many1( // there can of course be many properties with respective types
          tuple((
            ws(alpha1), // property name
            tag(":"),
            ws(alpha1), // property type
            opt(alt((tag(","),tag(";"))))
            )
          
        )),
        ws(tag("}"))
      )
      )  
      )
      
    )
    
  (input);

  let mut my_type = Type::new();

  match result {
    Ok(tupled) => {
      let (rest, (name, prop_type_hash)) = tupled.clone();

      println!("{:?}", tupled);
      
    my_type.add_name(name);
    for x in prop_type_hash.into_iter() {
      let (prop, _ , type_name, _ ) = x;
      my_type.add_property(prop, type_name);
    }      
    }
    Err(err) => {
      println!("Awe shucks, we have an error: {:?}", err);
    }
  }
  


  println!("The resultant type is:\n{:?}", my_type)
}



fn parse2(input: &str, parsers: &[fn(&str)]) {

  println!("================");
  println!("INPUT:'{}'", input);

  for (i,p) in parsers.iter().enumerate(){
    p(input);
     println!("Applying parser #{:?}:", i); 
    
    println!("----");
  }

  println!("================");
  
  
}