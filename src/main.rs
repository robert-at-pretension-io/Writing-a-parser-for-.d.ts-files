use std::io;
use std::fs;

use nom::{
  IResult,
  sequence::delimited,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  character::complete::{char, alpha1, alpha0, multispace0, one_of, alphanumeric1},
  combinator::recognize,
  bytes::complete::{is_not, take_until},
  // character::is_alphanumeric
  bytes::complete::tag,
  multi::{many1, many0},
  branch::alt,
  combinator::opt,
  sequence::{tuple, preceded, pair,terminated},
  error::ParseError,
};
use std::fmt::Debug;
use std::collections::HashMap;

fn main() {

println!("This folder has the following files: ");
  let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
  
  loop {

  
println!("Enter a filename (or type exit to quit):");
   let mut filename = String::new();
   io::stdin().read_line(&mut filename).expect("failed to readline");
  filename = filename.trim_end().to_string();

 
  
  if filename.contains("exit") {
    break
  }

 match fs::read_to_string(&filename) {
   Ok(unparsed) => {

     type_file(&unparsed);

   },
   Err(err) => {
     println!("error attempting to read file:{:?}", err);
   }
 }
  
  
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
///Todo: look at imported types and recursively read those too
fn type_file(file : &str) -> IResult<&str, Vec<Type>> {
  let result = many0( //hypothetically, the file could contain no interfaces or types
    preceded(
      take_until("interface"), //
      interface_block
    
    )
    
  )(file)?;
    Ok(result)
}

fn label_identifier(input: &str) -> IResult<&str,&str> {
  let (rest,m) = recognize(pair(
    alt((alpha1,tag("_"))),
    many0(alt((alphanumeric1, tag("_"))))
  ))(input)?;
  Ok((rest, m))
}

fn interface_block(input: &str) -> IResult<&str, Type> {
  let result : IResult<&str,(&str, Vec<(&str,&str,&str, Option<&str>)>)>= 
    preceded(
      ws(alt((tag("interface"), 
              tag("type")
              
             
             ))), // These typescript types can either be labeled with type or interface
     
        
      tuple(
      (
        ws(alpha1), // name of the type/interface
        delimited(
        ws(tag("{")), 
        many1( // there can of course be many properties with respective types
          tuple((
            // ws(
            //   take_while(
            //     alt(
            //       alphanumeric0,
            //       one_of("-_")
            //     )
            //   )
            //   ),
            ws(label_identifier), // property name
            tag(":"),
            ws(
              recognize(
                pair(
                  
                    alphanumeric1,
                    opt(
                      tag("[]")
                    )
                  
                )
              )
            ), // property type
            
            opt(
              
                tag(";")                    
          
            ),
            ) 
            
          
        )),
        
        ws(tag("}"))
        ),
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
  println!("The resultant type is:\n{:?}", my_type);
      
    Ok((rest, my_type))
      
    }
    Err(err) => {
      println!("Awe shucks, we have an error: {:?}", err);
      Err(err)
    }
  }
  


  
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