///! # Type inference in less than 100 lines of Rust
///!
///! - Do with it what you will
///! - Licensed under (https://en.wikipedia.org/wiki/WTFPL)
///!
///! ~ zesterer

use std::collections::HashMap;

/// A concrete type that has been fully inferred
#[derive(Debug)]
enum Type {
    Num,
    Bool,
    List(Box<Type>),
    Func(Box<Type>, Box<Type>),
}

/// A identifier to uniquely refer to our type terms
pub type TypeId = usize;

/// Information about a type term
#[derive(Clone, Debug)]
enum TypeInfo {
    // No information about the type of this type term
    Unknown,
    // This type term is the same as another type term
    Ref(TypeId),
    // This type term is definitely a number
    Num,
    // This type term is definitely a boolean
    Bool,
    // This type term is definitely a list
    List(TypeId),
    // This type term is definitely a function
    Func(TypeId, TypeId),
}

#[derive(Default)]
struct Engine {
    id_counter: usize, // Used to generate unique IDs
    vars: HashMap<TypeId, TypeInfo>,
}

impl Engine {
    /// Create a new type term with whatever we have about its type
    pub fn insert(&mut self, info: TypeInfo) -> TypeId {
        // Generate a new ID for our type term
        let id = self.id_counter;
        self.id_counter += 1;
        self.vars.insert(id, info);
        id
    }
    
    /// Make the types of two type terms equivalent (or produce an error if
    /// there is a conflict between them)
    pub fn unify(&mut self, a: TypeId, b: TypeId) -> Result<(), String> {
        use TypeInfo::*;
        match (&self.vars[&a].clone(), &self.vars[&b].clone()) {
            // Follow any references
            (Ref(a), _) => self.unify(a, b),
            (_, Ref(b)) => self.unify(a, b),
            
            // When we don't know anything about either term, assume that
            // they match and make the one we know nothing about reference the
            // one we may know something about
            (Unknown, _) => {
            	self.vars.insert(a, TypeInfo::Ref(b));
        		Ok(())
    		}
            (_, Unknown) => {
            	self.vars.insert(b, TypeInfo::Ref(a));
            	Ok(())
        	}
            
            // Primitives are trivial to unify
            (Num, Num) => Ok(()),
            (Bool, Bool) => Ok(()),
            
            // When unifying complex types, we must check their sub-types. This
            // can be trivially implemented for tuples, sum types, etc.
            (List(a_item), List(b_item)) => self.unify(a_item, b_item),
            
            (Func(a_i, a_o), Func(b_i, b_o)) => self.unify(a_i, b_i)
                .and_then(|_| self.unify(a_o, b_o)),
            
            // If no previous attempts to unify were successful, raise an error
            (a, b) => Err(format!("Conflict between {:?} and {:?}", a, b)),
        }
    }
    
    /// Attempt to reconstruct a concrete type from the given type term ID. This
    /// may fail if we don't yet have enough information to figure out what the
    /// type is.
    pub fn reconstruct(&self, id: TypeId) -> Result<Type, String> {
        use TypeInfo::*;
        match self.vars[&id] {
            Unknown => Err(format!("Cannot infer")),
            Ref(id) => self.reconstruct(id),
            Num => Ok(Type::Num),
            Bool => Ok(Type::Bool),
            List(item) => Ok(Type::List(Box::new(self.reconstruct(item)?))),
            Func(i, o) => Ok(Type::Func(
                Box::new(self.reconstruct(i)?),
                Box::new(self.reconstruct(o)?),
            )),
        }
    }
}

// # Example usage
// In reality, the most common approach will be to walk your AST, assigning type
// terms to each of your nodes with whatever information you have available. You
// will also need to call `engine.unify(x, y)` when you know two nodes have the
// same type, such as in the statement `x = y;`.

fn main() {
    let mut engine = Engine::default();
    
    // A function with an unknown input
    let i = engine.insert(TypeInfo::Unknown);
    let o = engine.insert(TypeInfo::Num);
    let f0 = engine.insert(TypeInfo::Func(i, o));
    
    // A function with an unknown output
    let i = engine.insert(TypeInfo::Bool);
    let o = engine.insert(TypeInfo::Unknown);
    let f1 = engine.insert(TypeInfo::Func(i, o));
    
    // Unify them together...
    engine.unify(f0, f1).unwrap();
    
    // A list of the aforementioned function
    let list = engine.insert(TypeInfo::List(f1));
    
    // ...and compute the resulting type
    println!("Final type = {:?}", engine.reconstruct(list));
}
