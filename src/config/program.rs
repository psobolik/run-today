/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-08
 */

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Program {
    id: usize,
    name: String,
    args: Option<Vec<String>>,
}

impl Program {
    pub fn new(id: usize, name: &str, args: Option<Vec<String>>) -> Program {
        Program {
            id,
            name: name.to_owned(),
            args: args.to_owned(),
        }
    }

    pub fn from_string(value: String) -> Result<Program, &'static str> {
        let mut words = value.split_whitespace();
        if let Some(name) = words.next() {
            let rest: Vec<&str> = words.collect();
            let args: Option<Vec<String>> = if !rest.is_empty() {
                Some(rest.iter().map(|f| f.trim().to_string()).collect())
            } else {
                None
            };
            Ok(Program::new(0, name, args))
        } else {
            Err("Can't parse program string")
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn args(&self) -> Option<Vec<String>> {
        self.args.clone()
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(args) = self.args() {
            for arg in args {
                write!(f, " {}", arg)?;
            }
        }
        Ok(())
    }
}
