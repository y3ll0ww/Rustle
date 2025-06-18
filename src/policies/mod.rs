use std::collections::VecDeque;

use crate::api::{ApiResponse, Error, Null};

pub mod projects;
pub mod users;
pub mod workspaces;

enum Rule {
    Or(bool),
    And(bool),
}

pub struct Policy {
    rules: VecDeque<Rule>,
}

impl Policy {
    fn rule(rule: bool) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Rule::Or(rule));
        Policy { rules: queue }
    }

    fn and(mut self, rule: bool) -> Self {
        self.rules.push_back(Rule::And(rule));
        self
    }

    fn or(mut self, rule: bool) -> Self {
        self.rules.push_back(Rule::Or(rule));
        self
    }

    fn is_authorized(&self, msg: &str) -> Result<(), String> {
        let mut authorized = false;

        for rule in &self.rules {
            authorized = match rule {
                Rule::And(pass) => authorized && *pass,
                Rule::Or(pass) => authorized || *pass,
            }
        }

        if authorized {
            return Ok(());
        }

        Err(msg.to_string())
    }

    fn unauthorized(&self, msg: &str) -> Result<(), Error<Null>> {
        self.is_authorized(msg).map_err(ApiResponse::unauthorized)
    }

    fn not_found(&self, msg: &str) -> Result<(), Error<Null>> {
        self.is_authorized(msg).map_err(ApiResponse::not_found)
    }
}
