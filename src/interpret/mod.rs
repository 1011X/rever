//use std::fs::File;

mod value;

pub use self::value::Value;

pub type EvalResult = Result<Value, &'static str>;
pub type Scope = Vec<(String, Value)>;

#[derive(Clone)]
pub struct StackFrame {
    args: Vec<Value>,
    locals: Vec<(String, Value)>,
}

pub type Stack = Vec<StackFrame>;

/*
TODO: create some sort of trait(s) or generic struct(s) that can:
+ Take an input device and store data that's given back.
  + Data that's given back is stored in a stack.
  + Data from the stack is used before data from the device.
+ Take an output device and return data that was passed to it.
  + Data that's given back is stored in a stack.
  + Data from the stack can be passed back until it's empty.
  + Should stdout be written to immediately? or on program close? or have a flush procedure that can
    be called?

The goal is to have stdin and stdout behave as close to files as possible.
*/
