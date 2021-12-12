/**
Evaluate a mathematical expression in postfix notation ("Reverse Polish Notation") at compile time.

# Returns
The result of the calculation.

# Examples
```
use crate::eval_postfix;
println!("{}", eval_postfix!(1f32 1f32 + 2f32 %)); // Prints "0", because (1 + 1) % 2 = 0.
```
*/
#[macro_export]
macro_rules! eval_postfix {
    (@operator [$b:expr, $a:expr $(,$call_stack:expr)*] $operator:tt $($leftover:tt)*) => {
        eval_postfix!([$a $operator $b $(,$call_stack)*] $($leftover)*)
    };
    
    // Could not apply operator
    (@operator $call_stack:tt $operator:tt $($leftover:tt)*) => {
        compile_error!(concat!("Could not apply operator \"",
                                stringify!($operator),
                                "\" to the current call stack: ",
                                stringify!($call_stack)
                            )
                    )
    };

    // Addition
    ($call_stack:tt + $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack + $($leftover)*)
    };
    
    // Subtraction
    ($call_stack:tt - $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack - $($leftover)*)
    };

    // Multiplication
    ($call_stack:tt * $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack * $($leftover)*)
    };
    
    // Division
    ($call_stack:tt / $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack / $($leftover)*)
    };

    // Modulo
    ($call_stack:tt % $($leftover:tt)*) => {
        eval_postfix!(@operator $call_stack % $($leftover)*)
    };

    // Recursively call macro with the rest of the expression
    ([$($call_stack:expr),*] $num:tt $($leftover:tt)*) => {
        eval_postfix!([$num $(,$call_stack)*] $($leftover)*)
    };
    
    // Return final value
    ([$res:expr]) => {
        $res
    };
    
    // Missed operator
    ([$($call_stack:expr),*]) => {
        compile_error!(concat!("Error: Could not find final value \
                                for the expression. \
                                Maybe you missed an operator? \
                                Final call stack: ",
                               stringify!([$($call_stack),*])
                            )
                    )
    };
    
    // Catch everything else
    ($($tokens:tt)*) => {
        eval_postfix!([] $($tokens)*)
    };
}