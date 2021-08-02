# Rust - Context Free Grammar
uma lib de gramatica sem contexto em rust

abaixo exemplo de uso que pode ser encontrado em src/main.rs

``` sh
let rules: [Rule; 4] = [
        Rule {
            head: String::from("s"),
            tokens: String::from("a b"),
        },
        Rule {
            head: String::from("a"),
            tokens: String::from("the"),
        },
        Rule {
            head: String::from("b"),
            tokens: String::from("c cat"),
        },
        Rule {
            head: String::from("c"),
            tokens: String::from("happy|sad|meow|angry|yummain"),
        },
    ];
    
    let expr: String = expand_all(&rules);
    println!("{}", expr);
```
