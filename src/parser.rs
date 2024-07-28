use core::num;
use crate::lexer::*;
#[derive(Debug)]
pub enum EvalValue {
    Int(i64),
    Float(f64),
    String(String),
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: TokenName,
    pub value: Vec<Node>,
    pub token: Option<Token>, // optional token field for leaf nodes
}

impl Node {
    pub fn new(name: TokenName, value: Vec<Node>, token: Option<Token>) -> Self {
        Node { name, value, token }
    }

    // Eval function
    pub fn eval(&self) {

    }
}


#[derive(Debug, Clone)]
pub struct Rule {
    token_names: Vec<TokenName>,
    result_type: TokenName,
}

pub struct Parser {
    tokens: Vec<Token>,
    rules: Vec<Vec<Rule>>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            rules: vec![vec![]],
            position: 0,
        }
    }

    pub fn add_rule(&mut self, token_names: Vec<TokenName>, result_type: TokenName) {
        let rule = Rule {
            token_names,
            result_type,
        };
        let last_rule = self.rules.last_mut().unwrap();
        last_rule.push(rule);
    }

    pub fn decrease_importance(&mut self) {
        self.rules.push(vec![]);
    }

    pub fn get_next_nodes(&self, count: usize, ast: &Vec<Node>) -> Vec<Node> {
        let end = if self.position + count > ast.len() {
            ast.len()
        } else {
            self.position + count
        };
        ast[self.position..end].to_vec()
    }

    pub fn parse(&mut self) -> Node {
        let mut ast: Vec<Node> = Vec::new();

        // Initialize the AST with token nodes
        for token in &self.tokens {
            ast.push(Node::new(token.name.clone(), vec![], Some(token.clone())));
        }

        let mut progress = true;
        
        while progress {
            progress = false;
            for importance_level in &self.rules {
                self.position = 0;
                while self.position < ast.len() {
                    for rule in importance_level {
                        let num_of_required_tokens = rule.token_names.len();

                        // Get the next nodes according to the rule length
                        let next_nodes: Vec<Node> = self.get_next_nodes(num_of_required_tokens, &ast);
                        let next_nodes_names: Vec<TokenName> = next_nodes.iter().map(|node| node.name.clone()).collect();

                        if next_nodes_names == rule.token_names {
                            // Use the nodes directly to preserve the structure
                            let next_node_values: Vec<Node> = next_nodes.clone();

                            // Remove the old nodes and insert the new combined node
                            ast.drain(self.position..self.position + num_of_required_tokens);
                            ast.insert(self.position, Node::new(rule.result_type.clone(), next_node_values.clone(), None));
                            progress = true;
                            break;
                        }
                    }

                    if !progress {
                        self.position += 1;
                    } else {
                        break;
                    }
                }

                if progress {
                    break;
                }
            }
        }

        ast[0].clone()
    }
}
