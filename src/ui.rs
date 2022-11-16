use serde::{Serialize, Deserialize};


/// The point of the PK enum is to capture the various types that might be used for a primary key,
/// with String and i32 being the most common by far
#[derive(Serialize, Deserialize)]
pub enum PK {
    String(String),         // for things that have a string natural key
    Int16(i16),
    Int32(i32),             // for most things
    Int64(i64),
    Tup2(i32, i32),       // for subdomains etc.
    Tup3(i32, i32, i32),  // for urls etc.
    ISIO(i32, String, i32, Option<String>), // for addresses
}


/// The DisplayUI trait is intended to identify the 'Who What Where' of an object
/// This allows an objet to be displayed in a user iterface
pub trait DisplayUI {

    /// return the name of this object, i.e. "Chicago" or "Cool Blue Inc."
    fn name(&self) -> String; 

    /// return the primary key for an object
    /// This can often be implemented by calling Id32 or IdString if it has been implemented
    fn pk(&self) -> PK;   

    /// return a static string reflecting the 'data type'
    fn data_type() -> &'static str; 

    /// optional subtype, default implementation is to return None
    fn sub_type(&self) -> Option<&'static str> {
        None
    }
}

/// Many structs have an i32 primary key
/// You can define this trait on them 
pub trait PK32 {
    fn pk_32(&self) -> i32;
}

/// When autocompleting things etc., you often just want the name and the PK
/// This struct captures that
#[derive(Serialize)]
pub struct Identifier {
    pub data_type: &'static str,
    pub name: String,
    pub pk: PK,
}
