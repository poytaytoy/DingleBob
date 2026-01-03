use std::collections::HashMap;
use crate::ast::Value;
use crate::ast::BreakResult;
use crate::token::Token;
use std::rc::Rc;
use std::cell::RefCell;
use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use std::fs;

pub struct Environment {
    env_superior: Option<Rc<RefCell<Environment>>>,
    pub hashMap: HashMap<String, Value>,
    repl: bool
}

impl Environment {
    pub fn new(env_superior: Option<Rc<RefCell<Environment>>>, repl: bool) -> Self {
        Environment {
            env_superior,
            hashMap: HashMap::new(),
            repl: repl
        }
    }

    pub fn clone(&self) -> Self {
        let mut dummy_env_superior = None;

        if !self.env_superior.is_none() {
            dummy_env_superior = Some(Rc::new(RefCell::new(
                self.env_superior.clone().unwrap().borrow().clone(),
            )));
        }

        Environment {
            env_superior: dummy_env_superior,
            hashMap: self.hashMap.clone(),
            repl: self.repl
        }
    }

    pub fn define(&mut self, var: Token, value: Value) -> Result<Value, BreakResult> {
        if self.hashMap.contains_key(&var.lexeme) {
            return Err(BreakResult::Error(self.handle_error(
                &format!("Name '{}' is already defined in this scope.", &var.lexeme),
                &var,
            )));
        }

        self.hashMap.insert(var.lexeme.clone(), value);
        Ok(Value::None)
    }

    pub fn define_from_execute(&mut self, var: String, value: Value) -> Result<Value, BreakResult> {
        // No token/span info here, so we can't Ariadne-highlight.
        if self.hashMap.contains_key(&var) {
            return Err(BreakResult::Error(format!(
                "Interpreter Error: Name '{}' is already defined",
                &var
            )));
        }

        self.hashMap.insert(var, value);
        Ok(Value::None)
    }

    pub fn define_default(&mut self, name: &str, value: Value) {
        self.hashMap.insert(String::from(name), value);
    }

    fn retrieve(&self, token: &Token) -> Option<Value> {
        if let Some(v) = self.hashMap.get(&token.lexeme) {
            return Some(v.clone());
        }

        match &self.env_superior {
            Some(env) => env.borrow().retrieve(token),
            None => None,
        }
    }

    pub fn get(&self, token: Token) -> Result<Value, BreakResult> {
        match self.retrieve(&token) {
            Some(v) => Ok(v),
            None => Err(BreakResult::Error(self.handle_error(
                &format!(
                    "Undefined variable '{}': no binding found in this scope (or any enclosing scope).",
                    token.lexeme
                ),
                &token,
            ))),
        }
    }

    pub fn get_at(&self, token: Token, steps: i32) -> Result<Value, BreakResult> {
        if steps == 0 {
            return match self.hashMap.get(&token.lexeme) {
                Some(v) => Ok(v.clone()),
                None => Err(BreakResult::Error(self.handle_error(
                    &format!(
                        "Resolved variable '{}' not found at the expected scope depth (resolver bug or stale locals map).",
                        &token.lexeme
                    ),
                    &token,
                ))),
            };
        }

        match &self.env_superior {
            Some(env) => env.borrow().get_at(token, steps - 1),
            None => Err(BreakResult::Error(self.handle_error(
                &format!("Resolver depth {} exceeds the available environment chain.", steps),
                &token,
            ))),
        }
    }

    pub fn assign(&mut self, token: Token, value: Value) -> Result<Value, BreakResult> {
        if self.hashMap.contains_key(&token.lexeme) {
            self.hashMap.insert(token.lexeme.clone(), value);
            Ok(Value::None)
        } else {
            match &mut self.env_superior {
                Some(env) => env.borrow_mut().assign(token, value),
                None => Err(BreakResult::Error(self.handle_error(
                    &format!(
                        "Assignment to undefined variable '{}'. Declare it before assigning.",
                        &token.lexeme
                    ),
                    &token,
                ))),
            }
        }
    }

    fn handle_error(&self, msg: &str, token: &Token) -> String {

        let line = token.line;
        let file = token.file.clone();
        let start = token.id as usize;
        let end = token.id_end as usize;

        if !self.repl{ 
            let mut colors = ColorGenerator::new();
            let a = colors.next();

            let src = fs::read_to_string(&file)
                .unwrap_or_else(|_| "<could not read source file>".to_string());

            Report::build(
                ReportKind::Error,
                (&file, (line.saturating_sub(1)) as usize..3),
            )
            .with_message("Interpreter Error")
            .with_label(
                Label::new((&file, start..end))
                    .with_message(msg)
                    .with_color(a),
            )
            .finish()
            .print((&file, Source::from(&src)))
            .unwrap();
        }

        format!("Interpreter Error: {}", msg)
    }
}
