use rand::seq::IteratorRandom;

pub struct Rule {
    pub head: String,
    pub tokens: String,
}

/// Function returns the tokens of the rule if theres a rule
/// that matches the name, if not it returns none
fn is_terminal(rules: &[Rule], name: &str) -> Option<String> {
    for rule in rules {
        if rule.head == name {
            return Some(String::from(&rule.tokens));
        }
    }

    return None;
}

/// Fuction check with theres a branch char or '|'
/// returning part of the rule that matches lookahead
pub fn find_token(tokens: String, lookahead: String) -> Option<String> {
    for sentence in tokens.split('|') {
        for word in sentence.split(" ") {
            if word == lookahead
            {
                return Some(String::from(sentence));
            }
        }    
    }

    return None;
}

/// Fuction check with theres a branch char or '|'
/// returning one of the elements in tokens using the classic rng
fn choose_token(tokens: String) -> String {
    let mut secret_number = rand::thread_rng();

    return match tokens.split('|').choose(&mut secret_number) {
        Some(token) => String::from(token),
        None => String::from(""),
    };
}

/// Function used to expand an expression
/// returns the transformed expression and a bool if all tokens are terminal
pub fn expand(rules: &[Rule], expr: String) -> (String, bool) {
    let mut is_all_terminal = true;
    let mut aux = String::from("");
    let tokens = expr.split(' ');

    for token in tokens {
        let val = match is_terminal(&rules, &token) {
            Some(val) => {
                is_all_terminal = false;

                //checking if have another branch
                if val.contains("|") {
                    choose_token(val)
                } else {
                    String::from(val)
                }
            }
            None => String::from(token),
        };

        aux.push_str(&val);
        aux.push_str(" ");
    }

    return (aux, is_all_terminal);
}

pub fn expand_all(rules: &[Rule]) -> String
{
    let mut expr: String = String::from(&rules[0].head);
    let mut is_all_terminal: bool = false;

    while is_all_terminal == false {
        let (result, all_term) = expand(&rules, String::from(&expr));
        is_all_terminal = all_term;
        expr = result;
    };

    return expr;
}
