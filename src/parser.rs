use core::num;
use crate::lexer::*;
use crate::ast;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    name: TokenName,
    value: Vec<Token>,
}





impl Node {
    fn new(name: TokenName, value: Vec<Token>) -> Self {
        Node {
            name,
            value,
        }
    }
    pub fn eval(&self) {
        match self.name {
            TokenName::STRING => ast::eval_string(self.clone()),
            _ => {}
                
        }
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

    pub fn add_rule(&mut self, token_names: Vec<TokenName>, result_type: TokenName){
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

    pub fn get_next_nodes(&self, anzahl: usize, ast: &Vec<Node>) -> Vec<Node> {
        let end = if self.position + anzahl > self.tokens.len() {
            self.tokens.len()
        } else {
            self.position + anzahl
        };
        let nodes = ast[self.position..end].to_vec();
        nodes
    }
    
    

    pub fn parse(&mut self) -> Node { // return ast
        let mut ast: Vec<Node> = Vec::new();
    
        for token in &self.tokens {
            ast.push(Node::new(token.name.clone(), vec![token.clone()]));
        }
    
        let mut last_ast: Vec<Node> = Vec::new();
    
        loop {
            for importance_level in &self.rules {
                for rule in importance_level {
                    let num_of_required_tokens = rule.token_names.len();
    
                    let next_nodes: Vec<Node> = self.get_next_nodes(num_of_required_tokens, &ast);
    
                    let next_nodes_names: Vec<TokenName> = next_nodes.iter().map(|node| node.name.clone()).collect();
                    let next_tokens: Vec<Token> = next_nodes.iter().flat_map(|x| x.value.clone()).collect();
    
                    if next_nodes_names == rule.token_names {
    
                        ast.drain(self.position .. self.position + num_of_required_tokens);
                        
                        ast.insert(self.position, Node::new(rule.result_type.clone(), next_tokens));
    
                        self.position += 1; // Update position by 1 instead of num_of_required_tokens
                    }
                }
            }
    
            if ast == last_ast {
                break;
            }
    
            last_ast = ast.clone();
        }

        for node in ast.clone() {
            println!("{:?}", node);
        }
        
        ast[0].clone()
    }
     
}