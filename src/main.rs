use std::io::{self, BufRead, Write};

use lrlex::lrlex_mod;
use lrpar::lrpar_mod;

// Using `lrlex_mod!` brings the lexer for `calc.l` into scope. By default the
// module name will be `calc_l` (i.e. the file name, minus any extensions,
// with a suffix of `_l`).
lrlex_mod!("calc.l");

/*
    ####### Role of the lexer file #############
    Lex programs recognize only regular expressions

    This file defines the lexical specification of your language. 
    It describes how to break down the input text into a series of TOKENS (or LEXEMES). 
    Each token is a meaningful collection of characters, such as numbers, operators, or identifiers.
    I can associate for each token a label (E.g. INT for numbers, PLUS for the + operator, etc.)
    manual : https://web.archive.org/web/20220402195947/dinosaur.compilertools.net/lex/index.html

    %% delimiter to mark the beginning of the rules, and one rule.
    [ \t]+$   ;      skip whitespace


                            lexical        grammar
                            rules          rules
                            |              |
                            v              v
                        +---------+    +---------+
                        |   Lex   |    |  Yacc   |
                        +---------+    +---------+
                            |              |
                            v              v
                        +---------+    +---------+
                Input -> |  yylex  | -> | yyparse | -> Parsed input
                        +---------+    +---------+

    General format of Lex source is : 

                            {definitions}       (Often omitted)
                            %%
                            {rules}
                            %%                  (optional)
                            {user subroutines}  (Often omitted)

    The definitions section contains a combination of
                    1) Definitions, in the form ``name space translation''.
                    2) Included code, in the form ``space code''.
                    3) Included code, in the form
                                        %{
                                        code
                                        %}
                    4) Start conditions, given in the form
                                %S name1 name2 ...
                    5) Character set tables, in the form
                            %T
                            number space character-string
                            ...
                            %T
                    6) Changes to internal array sizes, in the form
                                        %x  nnn
                        where nnn is a decimal integer representing an array size and x selects the parameter as follows:
                            Letter          Parameter
                            p      positions
                            n      states
                            e      tree nodes
                            a      transitions
                            k      packed character classes
                            o      output array size


    3. Lex regular expressions


        Symbol	        Function	                                                        Example Usage
        Literal	    Matches exact string	                                            integer matches "integer"
        .	        Matches any character except newline	                            a.c matches "abc", "acc", "a3c"
        []	        Character class: Matches any one of the enclosed characters	        [abc] matches "a", "b", or "c"
        [^]	        Negated character class: Matches any character not enclosed	        [^abc] matches "d", "e", "1", etc.
        *	        Matches 0 or more occurrences of the preceding element	            a* matches "", "a", "aa", "aaa"
        +	        Matches 1 or more occurrences of the preceding element	            a+ matches "a", "aa", "aaa"
        ?	        Matches 0 or 1 occurrence of the preceding element	                ab?c matches "ac", "abc"
        {n,m}	    Matches from n to m repetitions of the preceding element	        a{1,3} matches "a", "aa", "aaa"
        ^	        Matches the beginning of a line	                                    ^abc matches "abc" at a line start
        $	        Matches the end of a line	                                        abc$ matches "abc" at a line end
        `	            `	                                                            Alternation: Matches either the expression before or after the pipe
        ( )     	Groups expressions and captures the content	                        (abc)+ matches "abc", "abcabc"
        \	        Escapes a special character to treat it as literal	                \* matches an asterisk (*)
        .	        Matches any character except newline	                            a.b matches "acb", "adb", etc.

        (Another summary) Regular expressions in Lex use the following operators:

                x        the character "x"
                "x"      an "x", even if x is an operator.
                \x       an "x", even if x is an operator.
                [xy]     the character x or y.
                [x-z]    the characters x, y or z.
                [^x]     any character but x.
                .        any character but newline.
                ^x       an x at the beginning of a line.
                <y>x     an x when Lex is in start condition y.
                x$       an x at the end of a line.
                x?       an optional x.
                x*       0,1,2, ... instances of x.
                x+       1,2,3, ... instances of x.
                x|y      an x or a y.
                (x)      an x.
                x/y      an x but only if followed by y.
                {xx}     the translation of xx from the
                            definitions section.
                x{m,n}   m through n occurrences of x
    
    4. Lex actions

        In Lex, when a regular expression matches, a corresponding action is executed. 
        The default action copies unmatched input directly to the output. 
        Developers can define specific actions like ignoring whitespace with simple commands,
        or more complex actions using Lex's built-in variables and functions. For instance, 
        the text matched by an expression is stored in `yytext`, and its length in `yyleng`. 
        Functions like `yymore()` and `yyless(n)` help manage complex input processing needs. 
        Lex also provides essential I/O functions (`input()`, `output()`, `unput()`), which can be customized
        by the user to manage character handling and integrate with external systems. Lastly, `yywrap()` manages end-of-file scenarios,
        allowing continuation or termination of input processing.

    5.. Ambiguity resolution

        Lex handles ambiguous source rules by prioritizing the longest matching expression. If multiple expressions match the same number of characters,
        the first specified rule is chosen. This principle ensures that Lex efficiently matches patterns without unnecessary backtracking. 
        For instance, between conflicting rules for "integer" and "[a-z]+", the one matching the longest sequence of characters is selected. 
        The REJECT action allows Lex to consider the next best matching rule, useful for overlapping patterns that need to be counted separately,
        enhancing the flexibility in handling complex pattern recognition scenarios

    8. Lex and Yacc

        If you want to use Lex with Yacc, note that what Lex writes is a program named yylex(), the name required by Yacc for its analyzer.
        Normally, the default main program on the Lex library calls this routine, but if Yacc is loaded, and its main program is used, Yacc will call yylex(). 
        In this case each Lex rule should end with return(token); where the appropriate token value is returned. 
        An easy way to get access to Yacc's names for tokens is to compile the Lex output file as part of the Yacc output file by placing 
        the line # include "lex.yy.c" in the last section of Yacc input. 

    9. Sensitivity context

        Lex can adapt its processing based on different input contexts using flags, start conditions, or multiple analyzers. 
        This allows for conditional rule application, making Lex versatile for tasks like distinguishing code sections in compilers 
        or processing varied text formats within the same document.

    Difference with GRMTOOLS
        -Lex has its own regular expression language whereas grmtools uses the well known Rust regex crate for regular expressions.
        -Lex files consist of a sequence of regular expressions and an action for each. grmtools lex files consists of a sequence of regular expressions and a token name.
    and some other.Completed list in the follow link:  https://softdevteam.github.io/grmtools/latest_release/book/lexcompatibility.html




*/

// Using `lrpar_mod!` brings the parser for `calc.y` into scope. By default the
// module name will be `calc_y` (i.e. the file name, minus any extensions,
// with a suffix of `_y`).
lrpar_mod!("calc.y");

/*
        ####### Role of the yacc file #############

        Yacc writes parsers that accept a large class of context free grammars, 
        but require a lower level analyzer to recognize input tokens (Lex)

        This file defines the grammar of your language. It specifies how tokens can be combined 
        to form valid syntactical constructs. It often includes semantic actions that describe 
        what should happen when a particular rule in the grammar is matched.

        -- Difference between Grmtools and Yacc

        Grmtools supports most major Yacc features,but in otherhand there are some differences between them.
        Follow the link for more information : https://softdevteam.github.io/grmtools/latest_release/book/yacccompatibility.html#yacc-compatibility

        -- YaccKinds -- 

        YaccKind::Grmtools is grmtools' own variant of Yacc syntax.
        Rules are annotated with a Rust type to which all their production's actions must adhere to.

        Example: 

        R1 -> Result<i32, ()>:
        'a' { Ok(5) }
        | 'b' { Err(()) }
        ;

        The return type of the actions in this rule must be Result<i32, ()>.
        It will execute the snippet Ok(5) <i32> when the input is 'a' and Err(()) when the input is 'b' <()>.
        

        [see in build.rs file]

*/

/*
        Parser

            Parsing is the act of checking whether a stream of lexemes match a grammar. 
            It is common to execute user-defined actions during parsing.

            The grmtools suite includes libraries like cfgrammar and lrtable to build LR parsers. 
            For most users, the lrpar library offers a Yacc-compatible parser that integrates modern 
            Rust practices, particularly in handling global variables and providing idiomatic Rust features.
*/

fn main() {
    // Get the `LexerDef` for the `calc` language.
    let lexerdef = calc_l::lexerdef();
    // calc_l is the module name of the lexer file generated by lrlex_mod! macro and build.rs file
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        // ask the user to input an expression
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                // Now we create a lexer with the `lexer` method with which
                // we can lex an input.
                let lexer = lexerdef.lexer(l);
                // Pass the lexer to the parser and lex and parse the input.
                let (res, errs) = calc_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &calc_y::token_epp));
                }
                match res {
                    Some(Ok(r)) => println!("Result: {:?}", r),
                    _ => eprintln!("Unable to evaluate expression.")
                }
            }
            _ => break
        }
    }
}