use cfgrammar::yacc::YaccKind;
use lrlex::CTLexerBuilder;

/*  YACC + PARSING

    The goal is to produce a modle that can be imported with lrex_mod!("token_map")

    The module will contain one constant, prefixed with T_ per token identifiers in the grammar.

    In the grammar:

    0)    Expr -> Result<u64, ()>:
    1)    Expr '+' Term { Ok($1? + $3?) }
    2)    | Term { $1 }
    .     ;

    1)  This rule called 'Expr' has two productions, one for addition and one for a single term.
        The first production is working when the input is an expression followed by a '+', followed by a term.
        In this case, the result is the sum of the two expressions.

    2)  The second production is working when the input is a single term.

    the module will contain const T_PLUS: u8 = ...;.

    Since Yacc grammars can contain token identifiers which are not valid Rust identifiers,
    ct_token_map allows you to provide a map from the token identifier to a "Rust friendly" variant.

        Expr -> Result<u64, ()>:
        Expr '+' Term { Ok($1? + $3?) }
        | Term { $1 }
        ;

    we would provide a map '+' => 'PLUS' leading, again, to a constant T_PLUS being defined.

    Explanation of the code:

    --Special Variables--

    $1 to $n: These variables are placeholders that refer to symbols in a production.
    The number corresponds to the position of the symbol within the production.

    If the symbol is a rule, the variable holds an instance of the rule's type. 
    If it's a lexeme, the variable holds a Result<Lexeme<StorageT>, Lexeme<StorageT>>, 
                                where 
    - the Ok variant represents a lexeme derived from user input
    - the Err variant is used for lexemes inserted during error recovery.

    --Lexer Access--

    This allows access to the lexer and its functions.
    Some function:
    - span_str: Returns the string corresponding to the span. (&'input str s from a Span reference)
    - '$span' Represents how much of the input the current production has matched and is captured in a cfgrammar::Span type.
    - $$ is the equivent of $ in Rust.
    - Other variables begining with $ are treated as errors.

        Factor -> Result<u64, ()>:
        '(' Expr ')' { $2 }
        | 'INT' {
            let v = $1.map_err(|_| ())?;
            parse_int($lexer.span_str(v.span()))
        }
        ;

    --Return types--
    
    Arbitrary Rust Types: Productions can return any Rust type.
    Lexeme Storage (StorageT): This is a generic parameter used for the lexeme type, enabling rules to return lexeme instances directly.
    Lifetime 'input: This is crucial for managing string lifetimes directly tied to the lexer,by avoiding unnecessary string copies.

    --- Return Spans when possible ---
    A raccomended practice is to return spans when possible, as they are more efficient than strings.
    This is because spans are references to the original input string, and not needing to copy the string and waste memory.
    In other hand we can use 'input to extract &str

    --- Have rules return Result type ---
    It is recommended to have rules return Result types, as this allows for error handling and recovery.
        - Err(()) : stop creating the tree when encounter an important inserted lexem.without throwing an exception or unwinding the stack.
                - We can use a map_err, but more efficent use a custom map_error
        - Err(Box<dyn Error>) : stop creating the tree when encounter an important inserted lexem and want explain something to the user..
                - In this way i can explain the error to the user and convert the error in a Result<_, Box<dyn Error>>
                    And give more information about the error: example.

                        Expr -> Result<u64, Box<dyn Error>>:
                    Expr '+' Term
                    {
                        Ok($1?.checked_add($3?)
                            .ok_or(Box::<dyn Error>::from("Overflow detected."))?)
                    }
                    | Term { $1 }
                    ;

    --Additional Parse Parameter-- 
    Parse Parameters (%parse-param): Additional parameters can be defined for the parser, accessible in all action code. 
    These parameters must implement the Copy trait, which includes references. 
    
    For example, adding 
    
    Declaration:
        %parse-param p: u64 
    
    allows an additional u64 parameter p to be available in the action code, providing extra context or control.

    Action code:
        R -> ...:
            'ID' { format!("{}{}", p, ...) }
            ;

    --Flatten Function -- 
        The flatten function simplifies handling lists in grammar rules by consolidating the pattern of 
        extending a list with new elements. It takes two results, merges them if both are Ok, and propagates
        errors otherwise.

    Example: 
        ListOfAs -> Result<Vec<A>, ()>:
            A { Ok(vec![$1?]) }                 // Converts a single element into a vector
            | ListOfAs A { flatten($1, $2) }  <==  Calls the flatten function to merge the list and the element
            ;

        A -> Result<A, ()>: ... ; // Definition of how a single element A is parsed
        %%

        // Flatten function to simplify handling lists of elements in grammar rules.
        fn flatten<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>)
                -> Result<Vec<T>, ()>
        {
            let mut flt = lhs?;  // Unwrap or return the error
            flt.push(rhs?);      // Push the new element or return the error
            Ok(flt)              // Return the updated vector if no errors occurred
        }          

        !! flatten, map_err, and Lexeme can be used together to create a powerful error handling system. !!


        */

fn main() {
    CTLexerBuilder::new()
        .lrpar_config(|ctp| {
            ctp.yacckind(YaccKind::Grmtools)
                .grammar_in_src_dir("calc.y")
                .unwrap()
        })
        .lexer_in_src_dir("calc.l")
        .unwrap()
        .build()
        .unwrap();
}
