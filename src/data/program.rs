/*
 * Copyright (c) 2023 Paul Sobolik
 * Created 2023-12-08
 */

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use unicode_segmentation::UnicodeSegmentation;

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

    pub fn parse_value(value: String) -> Vec<String> {
        let mut parts: Vec<String> = vec![];
        let mut acc: Vec<&str> = vec![];
        let mut in_quotes = false;
        for g in value.graphemes(true) {
            if g == "'" {
                in_quotes = !in_quotes;
            } else if in_quotes {
                acc.push(g);
            } else if g == " " {
                // Unquoted space starts a new part
                parts.push(acc.join("").to_string());
                acc = vec![];
            } else {
                acc.push(g);
            }
        }
        if !acc.is_empty() {
            parts.push(acc.join("").to_string());
        }
        parts
    }
    pub fn from_string(value: String) -> Result<Program, &'static str> {
        let parts = Self::parse_value(value);
        if parts.is_empty() {
            Err("Can't parse input")
        } else {
            let args = if parts.len() > 1 {
                Some(parts[1..].to_vec())
            } else {
                None
            };
            Ok(Program::new(0, parts[0].as_str(), args))
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
