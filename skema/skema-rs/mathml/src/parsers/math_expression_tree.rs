//! Pratt parsing module to construct S-expressions from presentation MathML.
//! This is based on the nice tutorial at https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
use crate::ast::VectorNotation;
use crate::{
    ast::{
        operator::{
            Derivative, DerivativeNotation, Gradient, Hat, Int, Logarithm, LogarithmNotation,
            Operator, Summation,
        },
        Math, MathExpression, Mi, Mrow,
    },
    parsers::interpreted_mathml::interpreted_math,
};
use derive_new::new;
use nom::error::Error;
use regex::Regex;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
use utoipa::ToSchema;

#[cfg(test)]
use crate::parsers::first_order_ode::{first_order_ode, FirstOrderODE};
///New whitespace handler before parsing

/// An S-expression like structure to represent mathematical expressions.
#[derive(
    Debug,
    Ord,
    PartialOrd,
    PartialEq,
    Eq,
    Clone,
    Hash,
    new,
    Deserialize,
    Serialize,
    ToSchema,
    JsonSchema,
)]
pub enum MathExpressionTree {
    Atom(MathExpression),
    Cons(Operator, Vec<MathExpressionTree>),
}

/// Implementation of Display for MathExpressionTree, in order to have a compact string
/// representation to work with --- this is useful both for human inspection and writing unit
/// tests.
impl fmt::Display for MathExpressionTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathExpressionTree::Atom(MathExpression::Ci(x)) => {
                write!(f, "{}", x.content)
            }
            MathExpressionTree::Atom(MathExpression::Mo(Operator::Derivative(x))) => {
                write!(f, "{:?}", x)
            }
            MathExpressionTree::Atom(MathExpression::Msqrt(x)) => {
                write!(f, "{}", x)
            }
            MathExpressionTree::Atom(i) => {
                write!(f, "{}", i)
            }
            MathExpressionTree::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

/// Converts Unicode, Greek letters, their symbols, and English representations of Greek letters in an input string to their respective LaTeX expressions.
#[allow(unreachable_patterns)] // Lambda in this twice?
fn unicode_to_latex(input: &str) -> String {
    // Step 1: Handle English representations of Greek letters
    let re_english_greek = Regex::new(r"Lambda|lambda|Alpha|alpha|Beta|beta|Gamma|gamma|Delta|delta|Epsilon|epsilon|Zeta|zeta|Eta|eta|Theta|theta|Iota|iota|Kappa|kappa|Lambda|lambda|Mu|mu|Nu|nu|Xi|xi|Omicron|omicron|Pi|pi|Rho|rho|Sigma|sigma|Tau|tau|Upsilon|upsilon|Phi|phi|Chi|chi|Psi|psi|Omega|omega").unwrap();
    let replaced_english_greek =
        re_english_greek.replace_all(input, |caps: &regex::Captures| match &caps[0] {
            "Lambda" => "\\Lambda".to_string(),
            "lambda" => "\\lambda".to_string(),
            "Alpha" => "\\Alpha".to_string(),
            "alpha" => "\\alpha".to_string(),
            "Beta" => "\\Beta".to_string(),
            "beta" => "\\beta".to_string(),
            "Gamma" => "\\Gamma".to_string(),
            "gamma" => "\\gamma".to_string(),
            "Delta" => "\\Delta".to_string(),
            "delta" => "\\delta".to_string(),
            "Epsilon" => "\\Epsilon".to_string(),
            "epsilon" => "\\epsilon".to_string(),
            "Zeta" => "\\Zeta".to_string(),
            "zeta" => "\\zeta".to_string(),
            "Eta" => "\\Eta".to_string(),
            "eta" => "\\eta".to_string(),
            "Theta" => "\\Theta".to_string(),
            "theta" => "\\theta".to_string(),
            "Iota" => "\\Iota".to_string(),
            "iota" => "\\iota".to_string(),
            "Kappa" => "\\Kappa".to_string(),
            "kappa" => "\\kappa".to_string(),
            "Lambda" => "\\Lambda".to_string(),
            "lambda" => "\\lambda".to_string(),
            "Mu" => "\\Mu".to_string(),
            "mu" => "\\mu".to_string(),
            "Nu" => "\\Nu".to_string(),
            "nu" => "\\nu".to_string(),
            "Xi" => "\\Xi".to_string(),
            "xi" => "\\xi".to_string(),
            "Omicron" => "\\Omicron".to_string(),
            "omicron" => "\\omicron".to_string(),
            "Pi" => "\\Pi".to_string(),
            "pi" => "\\pi".to_string(),
            "Rho" => "\\Rho".to_string(),
            "rho" => "\\rho".to_string(),
            "Sigma" => "\\Sigma".to_string(),
            "sigma" => "\\sigma".to_string(),
            "Tau" => "\\Tau".to_string(),
            "tau" => "\\tau".to_string(),
            "Upsilon" => "\\Upsilon".to_string(),
            "upsilon" => "\\upsilon".to_string(),
            "Phi" => "\\Phi".to_string(),
            "phi" => "\\phi".to_string(),
            "Chi" => "\\Chi".to_string(),
            "chi" => "\\chi".to_string(),
            "Psi" => "\\Psi".to_string(),
            "psi" => "\\psi".to_string(),
            "Omega" => "\\Omega".to_string(),
            "omega" => "\\omega".to_string(),
            _ => caps[0].to_string(),
        });

    // Step 2: Handle Greek letters represented in Unicode
    let re_unicode = Regex::new(r"&#x([0-9A-Fa-f]+);").unwrap();
    let replaced_unicode =
        re_unicode.replace_all(&replaced_english_greek, |caps: &regex::Captures| {
            let unicode = u32::from_str_radix(&caps[1], 16).unwrap();
            match unicode {
                0x0391 => "\\Alpha".to_string(),
                0x03B1 => "\\alpha".to_string(),
                0x0392 => "\\Beta".to_string(),
                0x03B2 => "\\beta".to_string(),
                0x0393 => "\\Gamma".to_string(),
                0x03B3 => "\\gamma".to_string(),
                0x0394 => "\\Delta".to_string(),
                0x03B4 => "\\delta".to_string(),
                0x0395 => "\\Epsilon".to_string(),
                0x03B5 => "\\epsilon".to_string(),
                0x0396 => "\\Zeta".to_string(),
                0x03B6 => "\\zeta".to_string(),
                0x0397 => "\\Eta".to_string(),
                0x03B7 => "\\eta".to_string(),
                0x0398 => "\\Theta".to_string(),
                0x03B8 => "\\theta".to_string(),
                0x0399 => "\\Iota".to_string(),
                0x03B9 => "\\iota".to_string(),
                0x039A => "\\Kappa".to_string(),
                0x03BA => "\\kappa".to_string(),
                0x039B => "\\Lambda".to_string(),
                0x03BB => "\\lambda".to_string(),
                0x039C => "\\Mu".to_string(),
                0x03BC => "\\mu".to_string(),
                0x039D => "\\Nu".to_string(),
                0x03BD => "\\nu".to_string(),
                0x039E => "\\Xi".to_string(),
                0x03BE => "\\xi".to_string(),
                0x039F => "\\Omicron".to_string(),
                0x03BF => "\\omicron".to_string(),
                0x03A0 => "\\Pi".to_string(),
                0x03C0 => "\\pi".to_string(),
                0x03A1 => "\\Rho".to_string(),
                0x03C1 => "\\rho".to_string(),
                0x03A3 => "\\Sigma".to_string(),
                0x03C3 => "\\sigma".to_string(),
                0x03A4 => "\\Tau".to_string(),
                0x03C4 => "\\tau".to_string(),
                0x03A5 => "\\Upsilon".to_string(),
                0x03C5 => "\\upsilon".to_string(),
                0x03A6 => "\\Phi".to_string(),
                0x03C6 => "\\phi".to_string(),
                0x03A7 => "\\Chi".to_string(),
                0x03C7 => "\\chi".to_string(),
                0x03A8 => "\\Psi".to_string(),
                0x03C8 => "\\psi".to_string(),
                0x03A9 => "\\Omega".to_string(),
                0x03C9 => "\\omega".to_string(),
                _ => caps[0].to_string(),
            }
        });

    // Step 3: Handle other Unicode representations
    let re_other_unicode = Regex::new(r"&#x([0-9A-Fa-f]+);").unwrap();
    let replaced_other_unicode =
        re_other_unicode.replace_all(&replaced_unicode, |caps: &regex::Captures| {
            format!(
                "\\unicode{{U+{:X}}}",
                u32::from_str_radix(&caps[1], 16).unwrap()
            )
        });

    // Step 4: Handle Greek letter symbols
    let re_greek_symbols = Regex::new(r"Λ|λ|Α|α|Β|β|Γ|γ|Δ|δ|Ε|ε|Ζ|ζ|Η|η|Θ|θ|Ι|ι|Κ|κ|Λ|λ|Μ|μ|Ν|ν|Ξ|ξ|Ο|ο|Π|π|Ρ|ρ|Σ|σ|ς|Τ|τ|Υ|υ|Φ|φ|Χ|χ|Ψ|ψ|Ω|ω").unwrap();
    let replaced_greek_symbols =
        re_greek_symbols.replace_all(&replaced_other_unicode, |caps: &regex::Captures| {
            match &caps[0] {
                "Λ" => "\\Lambda".to_string(),
                "λ" => "\\lambda".to_string(),
                "Α" => "\\Alpha".to_string(),
                "α" => "\\alpha".to_string(),
                "Β" => "\\Beta".to_string(),
                "β" => "\\beta".to_string(),
                "Γ" => "\\Gamma".to_string(),
                "γ" => "\\gamma".to_string(),
                "Δ" => "\\Delta".to_string(),
                "δ" => "\\delta".to_string(),
                "Ε" => "\\Epsilon".to_string(),
                "ε" => "\\epsilon".to_string(),
                "Ζ" => "\\Zeta".to_string(),
                "ζ" => "\\zeta".to_string(),
                "Η" => "\\Eta".to_string(),
                "η" => "\\eta".to_string(),
                "Θ" => "\\Theta".to_string(),
                "θ" => "\\theta".to_string(),
                "Ι" => "\\Iota".to_string(),
                "ι" => "\\iota".to_string(),
                "Κ" => "\\Kappa".to_string(),
                "κ" => "\\kappa".to_string(),
                "Λ" => "\\Lambda".to_string(),
                "λ" => "\\lambda".to_string(),
                "Μ" => "\\Mu".to_string(),
                "μ" => "\\mu".to_string(),
                "Ν" => "\\Nu".to_string(),
                "ν" => "\\nu".to_string(),
                "Ξ" => "\\Xi".to_string(),
                "ξ" => "\\xi".to_string(),
                "Ο" => "\\Omicron".to_string(),
                "ο" => "\\omicron".to_string(),
                "Π" => "\\Pi".to_string(),
                "π" => "\\pi".to_string(),
                "Ρ" => "\\Rho".to_string(),
                "ρ" => "\\rho".to_string(),
                "Σ" => "\\Sigma".to_string(),
                "σ" => "\\sigma".to_string(),
                "ς" => "\\varsigma".to_string(), // 处理 sigma 的 final form
                "Τ" => "\\Tau".to_string(),
                "τ" => "\\tau".to_string(),
                "Υ" => "\\Upsilon".to_string(),
                "υ" => "\\upsilon".to_string(),
                "Φ" => "\\Phi".to_string(),
                "φ" => "\\phi".to_string(),
                "Χ" => "\\Chi".to_string(),
                "χ" => "\\chi".to_string(),
                "Ψ" => "\\Psi".to_string(),
                "ψ" => "\\psi".to_string(),
                "Ω" => "\\Omega".to_string(),
                "ω" => "\\omega".to_string(),
                _ => caps[0].to_string(),
            }
        });

    replaced_greek_symbols.to_string()
}

fn is_unary_operator(op: &Operator) -> bool {
    matches!(
        op,
        Operator::Sqrt
            | Operator::Factorial
            | Operator::Exp
            | Operator::Logarithm(_)
            | Operator::Power
            | Operator::Gradient(_)
            | Operator::Summation(_)
            | Operator::Hat(_)
            | Operator::Div
            | Operator::Abs
            | Operator::Laplacian
            | Operator::Derivative(_)
            | Operator::Int(_)
            | Operator::SurfaceInt
            | Operator::Sin
            | Operator::Cos
            | Operator::Tan
            | Operator::Sec
            | Operator::Csc
            | Operator::Cot
            | Operator::Arcsin
            | Operator::Arccos
            | Operator::Arctan
            | Operator::Arcsec
            | Operator::Arccsc
            | Operator::Arccot
            | Operator::Mean
    )
}

// Process parentheses in an expression and update the LaTeX string when connected by arithmatic operators like +, -, *, /.
// If the expression is a unary or multiplication or division operator, it is added to the LaTeX string as is.
// If the expression is not a unary operator, it is wrapped in parentheses before being added to the LaTeX string.
// If the expression is an atom, it is added to the LaTeX string directly.
fn process_expression_parentheses(expression: &mut String, met: &MathExpressionTree) {
    // Check if the rest vector is not empty and contains a MathExpressionTree::Cons variant.
    if let MathExpressionTree::Cons(op, _args) = met {
        // Check if the operator is a unary operator.
        if is_unary_operator(op) || matches!(op, Operator::Multiply | Operator::Divide) {
            // If it is a unary operator, add it to the LaTeX string as is.
            expression.push_str(&met.to_latex().to_string());
        } else {
            // If it is not a unary or multiplication or division operator, wrap it in parentheses before adding it to the LaTeX string.
            expression.push_str(&format!("({})", met.to_latex()));
        }
    } else {
        // If the expression is an atom, add it to the LaTeX string directly.
        expression.push_str(&met.to_latex().to_string());
    }
}

// Process parentheses in an expression and update the LaTeX string when connected by connectors like "_", "^", etc.
// If the expression is a unary operator, it is added to the LaTeX string as is.
// If the expression is not a unary operator, it is wrapped in parentheses before being added to the LaTeX string.
// If the expression is an atom, it is added to the LaTeX string directly.
fn process_atoms_cons_parentheses(expression: &mut String, met: &MathExpressionTree) {
    // Check if the rest vector is not empty and contains a MathExpressionTree::Cons variant.
    if let MathExpressionTree::Cons(op, _args) = met {
        // Check if the operator is a unary operator.
        if is_unary_operator(op) {
            // If it is a unary operator, add it to the LaTeX string as is.
            expression.push_str(&met.to_latex().to_string());
        } else {
            // If it is not a unary operator, wrap it in parentheses before adding it to the LaTeX string.
            expression.push_str(&format!("({})", met.to_latex()));
        }
    } else {
        // If the expression is an atom, add it to the LaTeX string directly.
        expression.push_str(&met.to_latex().to_string());
    }
}

/// Processes a MathExpression under the type of MathExpressionTree::Atom and appends
/// the corresponding LaTeX representation to the provided String.
fn process_math_expression(expr: &MathExpression, expression: &mut String) {
    match expr {
        // If it's a Ci variant, recursively process its content
        MathExpression::Ci(x) => {
            if let Some(vec_comp) = &x.notation {
                if VectorNotation::Arrow == *vec_comp {
                    expression.push_str("\\vec{");
                    process_math_expression(&x.content, expression);
                    expression.push('}');
                    if let Some(func_of_vec) = &x.func_of {
                        if !func_of_vec.is_empty() && !func_of_vec[0].content.to_string().is_empty()
                        {
                            expression.push('(');
                            for (index, func_ci) in func_of_vec.iter().enumerate() {
                                process_math_expression(&func_ci.content, expression);
                                if index < func_of_vec.len() - 1 {
                                    expression.push(',');
                                }
                            }
                            expression.push(')');
                        }
                    }
                } else if VectorNotation::Bold == *vec_comp {
                    expression.push_str("\\bold{");
                    process_math_expression(&x.content, expression);
                    expression.push('}');
                    if let Some(func_of_vec) = &x.func_of {
                        if !func_of_vec.is_empty() && !func_of_vec[0].content.to_string().is_empty()
                        {
                            expression.push('(');
                            for (index, func_ci) in func_of_vec.iter().enumerate() {
                                process_math_expression(&func_ci.content, expression);
                                if index < func_of_vec.len() - 1 {
                                    expression.push(',');
                                }
                            }
                            expression.push(')');
                        }
                    }
                }
            } else {
                process_math_expression(&x.content, expression);
                if let Some(func_of_vec) = &x.func_of {
                    if !func_of_vec.is_empty() && !func_of_vec[0].content.to_string().is_empty() {
                        expression.push('(');
                        for (index, func_ci) in func_of_vec.iter().enumerate() {
                            process_math_expression(&func_ci.content, expression);
                            if index < func_of_vec.len() - 1 {
                                expression.push(',');
                            }
                        }
                        expression.push(')');
                    }
                }
            }
        }
        MathExpression::Mi(Mi(id)) => {
            expression.push_str(unicode_to_latex(&id.to_string()).as_str());
        }
        MathExpression::Mn(number) => {
            expression.push_str(&number.to_string());
        }
        MathExpression::Msqrt(x) => {
            expression.push_str("\\sqrt{");
            process_math_expression(x, expression);
            expression.push('}');
        }
        MathExpression::Mfrac(x1, x2) => {
            expression.push_str("\\frac{");
            process_math_expression(x1, expression);
            expression.push_str("}{");
            process_math_expression(x2, expression);
            expression.push('}');
        }
        MathExpression::Msup(x1, x2) => {
            process_math_expression(x1, expression);
            expression.push_str("^{");
            process_math_expression(x2, expression);
            expression.push('}');
        }
        MathExpression::Msub(x1, x2) => {
            process_math_expression(x1, expression);
            expression.push_str("_{");
            process_math_expression(x2, expression);
            expression.push('}');
        }
        MathExpression::Msubsup(x1, x2, x3) => {
            process_math_expression(x1, expression);
            expression.push_str("_{");
            process_math_expression(x2, expression);
            expression.push_str("}^{");
            process_math_expression(x3, expression);
            expression.push('}');
        }
        MathExpression::Munder(x1, x2) => {
            expression.push_str("\\underset{");
            process_math_expression(x2, expression);
            expression.push_str("}{");
            process_math_expression(x1, expression);
            expression.push('}');
        }
        MathExpression::Mover(x1, x2) => {
            expression.push_str("\\overset{");
            process_math_expression(x2, expression);
            expression.push_str("}{");
            process_math_expression(x1, expression);
            expression.push('}');
        }
        MathExpression::Mtext(x) => {
            expression.push_str(x);
        }
        MathExpression::Mspace(x) => {
            expression.push_str(x);
        }
        MathExpression::AbsoluteSup(x1, x2) => {
            expression.push_str("\\left| ");
            process_math_expression(x1, expression);
            expression.push_str(" \\right|_{");
            process_math_expression(x2, expression);
            expression.push('}');
        }
        MathExpression::Mrow(vec_me) => {
            for me in vec_me.0.iter() {
                process_math_expression(me, expression);
            }
        }
        MathExpression::DownArrow(x) => match (&x.sub, &x.sup) {
            (Some(low), Some(up)) => {
                process_math_expression(&x.comp, expression);
                expression.push_str("{\\downarrow}_{");
                expression.push_str(&format!("{}", low));
                expression.push_str("}^{");
                expression.push_str(&format!("{}", up));
                expression.push('}');
            }
            (Some(low), None) => {
                process_math_expression(&x.comp, expression);
                expression.push_str("{\\downarrow}_{");
                expression.push_str(&format!("{}", low));
                expression.push('}');
            }
            (None, Some(up)) => {
                process_math_expression(&x.comp, expression);
                expression.push_str("{\\downarrow}");
                expression.push_str("^{");
                expression.push_str(&format!("{}", up));
                expression.push('}');
            }
            (None, None) => {
                process_math_expression(&x.comp, expression);
                expression.push_str("{\\downarrow}");
            }
        },
        MathExpression::SurfaceIntegral(row) => {
            process_math_expression(row, expression);
        }
        MathExpression::Mo(Operator::Hat(x)) => {
            expression.push_str("\\hat{");
            process_math_expression(&x.comp, expression);
            expression.push('}');
        }
        MathExpression::Mo(Operator::Other(x)) => {
            expression.push_str(x);
        }
        t => panic!("Unhandled MathExpression: {:?}", t),
    }
}

impl MathExpressionTree {
    /// Translates a MathExpressionTree struct to a content MathML string.
    pub fn to_cmml(&self) -> String {
        let mut content_mathml = String::new();
        match self {
            MathExpressionTree::Atom(i) => match i {
                MathExpression::Ci(x) => {
                    content_mathml.push_str(&format!("<ci>{}</ci>", x.content));
                }
                MathExpression::Mi(Mi(id)) => {
                    content_mathml.push_str(&format!("<ci>{}</ci>", id));
                }
                MathExpression::Mn(number) => {
                    content_mathml.push_str(&format!("<cn>{}</cn>", number));
                }
                MathExpression::Mrow(_) => {
                    panic!("All Mrows should have been removed by now!");
                }
                t => panic!("Unhandled MathExpression: {:?}", t),
            },
            MathExpressionTree::Cons(head, rest) => {
                content_mathml.push_str("<apply>");
                match head {
                    Operator::Add => content_mathml.push_str("<plus/>"),
                    Operator::Subtract => content_mathml.push_str("<minus/>"),
                    Operator::Multiply => content_mathml.push_str("<times/>"),
                    Operator::Equals => content_mathml.push_str("<eq/>"),
                    Operator::Divide => content_mathml.push_str("<divide/>"),
                    Operator::Power => content_mathml.push_str("<power/>"),
                    Operator::Exp => content_mathml.push_str("<exp/>"),
                    Operator::Abs => content_mathml.push_str("<abs/>"),
                    Operator::Gradient(x) => match &x.subscript {
                        Some(sub) => {
                            content_mathml.push_str("<grad/>");
                            content_mathml.push_str(&format!("<bvar>{}</bar>", sub));
                        }
                        None => content_mathml.push_str("<grad/>"),
                    },
                    Operator::Div => content_mathml.push_str("<divergence/>"),
                    Operator::Cos => content_mathml.push_str("<cos/>"),
                    Operator::Sin => content_mathml.push_str("<sin/>"),
                    Operator::Derivative(Derivative {
                        order,
                        var_index,
                        bound_var,
                        notation,
                    }) if (*order, *var_index) == (1_u8, 1_u8) => {
                        if *notation == DerivativeNotation::LeibnizTotal {
                            content_mathml.push_str("<diff/>");
                            content_mathml.push_str(&format!("<bvar>{}</bar>", bound_var));
                        } else if *notation == DerivativeNotation::LeibnizPartialStandard {
                            content_mathml.push_str("<partialdiff/>");
                            content_mathml.push_str(&format!("<bvar>{}</bar>", bound_var));
                        }
                    }
                    _ => {}
                }
                for s in rest {
                    content_mathml.push_str(&s.to_cmml());
                }
                content_mathml.push_str("</apply>");
            }
        }
        content_mathml
    }

    /// Translates to infix math expression to provide "string expressions" (e.g. ((α*ρ)*I)  )
    /// TA-4 uses "string expressions" to display over the transitions in their visual front end.
    pub fn to_infix_expression(&self) -> String {
        let mut expression = String::new();
        match self {
            MathExpressionTree::Atom(i) => match i {
                MathExpression::Ci(x) => {
                    expression.push_str(&format!("{}", x.content));
                }
                MathExpression::Mi(Mi(id)) => {
                    expression.push_str(&id.to_string());
                }
                MathExpression::Mn(number) => {
                    expression.push_str(&number.to_string());
                }
                MathExpression::Mtext(text) => {
                    expression.push_str(&text.to_string());
                }
                MathExpression::Mrow(_) => {
                    panic!("All Mrows should have been removed by now!");
                }
                t => panic!("Unhandled MathExpression: {:?}", t),
            },

            MathExpressionTree::Cons(head, rest) => {
                let mut operation = String::new();
                match head {
                    Operator::Add => operation.push('+'),
                    Operator::Subtract => operation.push('-'),
                    Operator::Multiply => operation.push('*'),
                    Operator::Equals => operation.push('='),
                    Operator::Divide => operation.push('/'),
                    Operator::Exp => operation.push_str("exp"),
                    _ => {}
                }
                let mut component = Vec::new();
                for s in rest {
                    component.push(s.to_infix_expression());
                }
                let math_exp = format!("({})", component.join(&operation.to_string()));
                expression.push_str(&math_exp);
            }
        }
        expression
    }

    /// Translates a MathExpressionTree struct to a LaTeX expression.
    pub fn to_latex(&self) -> String {
        let mut expression = String::new();
        match self {
            MathExpressionTree::Atom(i) => {
                process_math_expression(i, &mut expression);
            }
            MathExpressionTree::Cons(head, rest) => {
                match head {
                    Operator::Add => {
                        for (index, r) in rest.iter().enumerate() {
                            expression.push_str(&r.to_latex().to_string());
                            // Add "+" if it's not the last element
                            if index < rest.len() - 1 {
                                expression.push('+');
                            }
                        }
                    }
                    Operator::Subtract => {
                        // Handle unary minus
                        if rest.len() == 1 {
                            expression.push('-');
                            process_expression_parentheses(&mut expression, &rest[0]);
                        } else {
                            for (index, r) in rest.iter().enumerate() {
                                process_expression_parentheses(&mut expression, r);
                                // Add "-" if it's not the last element
                                if index < rest.len() - 1 {
                                    expression.push('-');
                                }
                            }
                        }
                    }
                    Operator::Multiply => {
                        for (index, r) in rest.iter().enumerate() {
                            if let MathExpressionTree::Cons(op, _) = r {
                                if is_unary_operator(op) {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Multiply = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Divide = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Dot = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else {
                                    expression.push_str(&format!("({})", r.to_latex()));
                                }
                            } else {
                                expression.push_str(&r.to_latex().to_string());
                            }
                            // Add "*" if it's not the last element
                            if index < rest.len() - 1 {
                                expression.push('*');
                            }
                        }
                    }
                    Operator::Equals => {
                        expression.push_str(&rest[0].to_latex().to_string());
                        expression.push('=');
                        expression.push_str(&rest[1].to_latex().to_string());
                    }
                    Operator::Divide => {
                        expression.push_str(&format!("\\frac{{{}}}", &rest[0].to_latex()));
                        expression.push_str(&format!("{{{}}}", &rest[1].to_latex()));
                    }
                    Operator::Exp => {
                        expression.push_str("\\mathrm{exp}");
                        expression.push_str(&format!("{{{}}}", rest[0].to_latex()));
                    }
                    Operator::Sqrt => {
                        expression.push_str("\\sqrt");
                        expression.push_str(&format!("{{{}}}", rest[0].to_latex()));
                    }
                    Operator::Lparen => {
                        expression.push('(');
                        expression.push_str(&rest[0].to_latex().to_string());
                    }
                    Operator::Rparen => {
                        expression.push(')');
                        expression.push_str(&rest[0].to_latex().to_string());
                    }
                    Operator::Compose => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push('_');
                        process_atoms_cons_parentheses(&mut expression, &rest[1]);
                    }
                    Operator::Factorial => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push('!');
                    }
                    Operator::Power => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push('^');
                        expression.push_str(&format!("{{{}}}", rest[1].to_latex()));
                    }
                    Operator::Gradient(x) => match &x.subscript {
                        Some(sub) => {
                            expression.push_str("\\nabla_{");
                            process_math_expression(sub, &mut expression);
                            expression.push('}');
                            expression.push('{');
                            process_expression_parentheses(&mut expression, &rest[0]);
                            expression.push('}');
                        }
                        None => {
                            expression.push_str("\\nabla{");
                            process_expression_parentheses(&mut expression, &rest[0]);
                            expression.push('}');
                        }
                    },
                    Operator::Dot => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push_str(" \\cdot ");
                        process_atoms_cons_parentheses(&mut expression, &rest[1]);
                    }
                    Operator::Cross => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push_str(" \\cross ");
                        process_atoms_cons_parentheses(&mut expression, &rest[1]);
                    }
                    Operator::Div => {
                        expression.push_str("\\nabla \\cdot {");
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push('}');
                    }
                    Operator::Abs => {
                        expression.push_str(&format!("\\left|{}\\right|", rest[0].to_latex()));
                    }
                    Operator::Derivative(d) => match d.notation {
                        DerivativeNotation::LeibnizTotal => {
                            expression.push_str("\\frac{d ");
                            process_expression_parentheses(&mut expression, &rest[0]);
                            expression.push_str("}{d");
                            process_math_expression(&d.bound_var.content, &mut expression);
                            expression.push('}');
                        }
                        DerivativeNotation::LeibnizPartialStandard => {
                            if d.order == 1_u8 {
                                expression.push_str("\\frac{\\partial ");
                                process_expression_parentheses(&mut expression, &rest[0]);
                                expression.push_str("}{\\partial ");
                                process_math_expression(&d.bound_var.content, &mut expression);
                                expression.push('}');
                            } else if d.order == 2_u8 {
                                expression.push_str("\\frac{\\partial^2 ");
                                process_expression_parentheses(&mut expression, &rest[0]);
                                expression.push_str("}{\\partial ");
                                process_math_expression(&d.bound_var.content, &mut expression);
                                expression.push_str("^2}");
                            }
                        }
                        DerivativeNotation::LeibnizPartialCompact => {
                            expression.push_str("\\partial_{");
                            process_math_expression(&d.bound_var.content, &mut expression);
                            expression.push('}');
                        }
                        DerivativeNotation::Newton => {
                            if d.order == 1_u8 {
                                expression.push_str("\\dot_{");
                                process_math_expression(&d.bound_var.content, &mut expression);
                                expression.push('}');
                            }
                        }
                        DerivativeNotation::DNotation => {
                            expression.push_str("\\frac{D ");
                            process_expression_parentheses(&mut expression, &rest[0]);
                            expression.push_str("}{D");
                            process_math_expression(&d.bound_var.content, &mut expression);
                            expression.push('}');
                        }
                        DerivativeNotation::Lagrange => {
                            if d.order == 1_u8 {
                                process_math_expression(&d.bound_var.content, &mut expression);
                                expression.push_str("^\\prime");
                            }
                        }
                    },
                    Operator::Sin => {
                        expression.push_str(&format!("\\sin({})", rest[0].to_latex()));
                    }
                    Operator::Cos => {
                        expression.push_str(&format!("\\cos({})", rest[0].to_latex()));
                    }
                    Operator::Tan => {
                        expression.push_str(&format!("\\tan({})", rest[0].to_latex()));
                    }
                    Operator::Sec => {
                        expression.push_str(&format!("\\sec({})", rest[0].to_latex()));
                    }
                    Operator::Csc => {
                        expression.push_str(&format!("\\csc({})", rest[0].to_latex()));
                    }
                    Operator::Cot => {
                        expression.push_str(&format!("\\cot({})", rest[0].to_latex()));
                    }
                    Operator::Arcsin => {
                        expression.push_str(&format!("\\arcsin({})", rest[0].to_latex()));
                    }
                    Operator::Arccos => {
                        expression.push_str(&format!("\\arccos({})", rest[0].to_latex()));
                    }
                    Operator::Arctan => {
                        expression.push_str(&format!("\\arctan({})", rest[0].to_latex()));
                    }
                    Operator::Arcsec => {
                        expression.push_str(&format!("\\arcsec({})", rest[0].to_latex()));
                    }
                    Operator::Arccsc => {
                        expression.push_str(&format!("\\arccsc({})", rest[0].to_latex()));
                    }
                    Operator::Arccot => {
                        expression.push_str(&format!("\\arccot({})", rest[0].to_latex()));
                    }
                    Operator::Mean => {
                        expression.push_str(&format!("\\langle {} \\rangle", rest[0].to_latex()));
                    }
                    Operator::Hat(x) => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push_str("\\hat{");
                        process_math_expression(&x.comp, &mut expression);
                        expression.push('}');
                    }
                    Operator::Summation(x) => match (&x.lower_bound, &x.upper_bound) {
                        (Some(low), Some(up)) => {
                            expression.push_str("\\sum_{");
                            expression.push_str(&format!("{}", low));
                            expression.push_str("}^{");
                            expression.push_str(&format!("{}", up));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                        }
                        (Some(low), None) => {
                            expression.push_str("\\sum_{");
                            expression.push_str(&format!("{}", low));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                        }
                        (None, Some(up)) => {
                            expression.push_str("\\sum^{");
                            expression.push_str(&format!("{}", up));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                        }
                        (None, None) => {
                            expression.push_str("\\sum ");
                            process_expression_parentheses(&mut expression, &rest[0]);
                        }
                    },
                    Operator::Int(x) => match (&x.lower_limit, &x.upper_limit) {
                        (Some(low), Some(up)) => {
                            expression.push_str("\\int_{");
                            process_math_expression(low, &mut expression);
                            expression.push_str("}^{");
                            process_math_expression(up, &mut expression);
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                            expression.push_str(&format!(" d{}", &*x.integration_variable));
                        }
                        (Some(low), None) => {
                            expression.push_str("\\int_{");
                            expression.push_str(&format!("{}", low));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                        }
                        (None, Some(up)) => {
                            expression.push_str("\\int^{");
                            expression.push_str(&format!("{}", up));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex());
                        }
                        (None, None) => {
                            expression.push_str("\\int ");
                            process_expression_parentheses(&mut expression, &rest[0]);
                        }
                    },
                    Operator::Laplacian => {
                        expression.push_str(&format!("\\nabla^2 {}", rest[0].to_latex()));
                    }
                    Operator::SurfaceInt => {
                        expression.push_str(&format!("\\oiint_S {}", rest[0].to_latex()));
                    }
                    Operator::Min => {
                        expression.push_str("\\min \\{ ");
                        for (index, r) in rest.iter().enumerate() {
                            expression.push_str(&r.to_latex().to_string());
                            expression.push(',');
                        }
                        // remove last redundant comma
                        let Some(_) = expression.pop() else { todo!() };
                        expression.push_str("\\}");
                    }
                    Operator::Multiply => {
                        for (index, r) in rest.iter().enumerate() {
                            if let MathExpressionTree::Cons(op, _) = r {
                                if is_unary_operator(op) {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Multiply = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Divide = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else if let Operator::Dot = op {
                                    expression.push_str(&r.to_latex().to_string());
                                } else {
                                    expression.push_str(&format!("({})", r.to_latex()));
                                }
                            } else {
                                expression.push_str(&r.to_latex().to_string());
                            }
                            // Add "*" if it's not the last element
                            if index < rest.len() - 1 {
                                expression.push('*');
                            }
                        }
                    }
                    Operator::Logarithm(x) => match &x.notation {
                        LogarithmNotation::Ln => {
                            expression.push_str(&format!("\\ln {}", rest[0].to_latex()));
                        }
                        LogarithmNotation::Log => {
                            expression.push_str(&format!("\\log {}", rest[0].to_latex()));
                        }
                        LogarithmNotation::LogBase(base) => {
                            expression.push_str("\\log_{");
                            expression.push_str(&format!("{}", base));
                            expression.push('}');
                            expression.push_str(&rest[0].to_latex().to_string());
                        }
                    },
                    Operator::Comma => {
                        process_atoms_cons_parentheses(&mut expression, &rest[0]);
                        expression.push(',');
                        process_atoms_cons_parentheses(&mut expression, &rest[1]);
                    }
                    _ => {
                        expression = "".to_string();
                        return "Contain unsupported operators.".to_string();
                    }
                }
            }
        }
        expression
    }
}

/// Represents a token for the Pratt parsing algorithm.
#[derive(Debug, Clone, PartialEq, Eq, new)]
pub enum Token {
    Atom(MathExpression),
    Op(Operator),
    Eof,
}

/// Lexer for the Pratt parsing algorithm.
#[derive(Debug)]
struct Lexer {
    /// Vector of input tokens.
    tokens: Vec<Token>,
}

impl MathExpression {
    /// Flatten Mfrac and Mrow elements.
    fn flatten(&self, tokens: &mut Vec<MathExpression>) {
        match self {
            // Flatten/unwrap Mrows
            MathExpression::Mrow(Mrow(elements)) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                for element in elements {
                    if let MathExpression::Ci(x) = element {
                        // Handles cos, sin, tan as operators
                        if x.content == Box::new(MathExpression::Mi(Mi("cos".to_string()))) {
                            tokens.push(MathExpression::Mo(Operator::Cos));
                            if let Some(vec) = x.func_of.clone() {
                                for v in vec {
                                    tokens.push(MathExpression::Ci(v));
                                }
                            }
                        } else if x.content == Box::new(MathExpression::Mi(Mi("sin".to_string()))) {
                            tokens.push(MathExpression::Mo(Operator::Sin));
                            if let Some(vec) = x.func_of.clone() {
                                for v in vec {
                                    tokens.push(MathExpression::Ci(v));
                                }
                            }
                        } else if x.content == Box::new(MathExpression::Mi(Mi("tan".to_string()))) {
                            tokens.push(MathExpression::Mo(Operator::Tan));
                            if let Some(vec) = x.func_of.clone() {
                                for v in vec {
                                    tokens.push(MathExpression::Ci(v));
                                }
                            }
                        } else {
                            element.flatten(tokens);
                        }
                    } else {
                        element.flatten(tokens);
                    }
                }
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            // Handles `Summation` operator with MathExpression
            MathExpression::SummationMath(x) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.op.flatten(tokens);
                x.func.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::Differential(x) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                match *x.diff {
                    MathExpression::Mo(Operator::Gradient(ref y)) => match y.subscript.clone() {
                        Some(sub) => tokens.push(MathExpression::Mo(Operator::Gradient(
                            Gradient::new(Some(sub)),
                        ))),
                        None => {
                            tokens.push(MathExpression::Mo(Operator::Gradient(Gradient::new(None))))
                        }
                    },
                    _ => {
                        x.diff.flatten(tokens);
                    }
                }
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.func.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::SurfaceIntegral(row) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                tokens.push(MathExpression::Mo(Operator::SurfaceInt));
                row.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            // Handles `Laplacian` operator with MathExpression
            MathExpression::LaplacianComp(x) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.op.flatten(tokens);
                x.comp.content.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            // Insert implicit division operators, and wrap numerators and denominators in
            // parentheses for the Pratt parsing algorithm.
            MathExpression::Mfrac(numerator, denominator) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                numerator.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Divide));
                tokens.push(MathExpression::Mo(Operator::Lparen));
                denominator.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            // Insert implicit `exponential` and `power` operators
            MathExpression::Msup(base, superscript) => {
                if let MathExpression::Ci(x) = &**base {
                    if x.content == Box::new(MathExpression::Mi(Mi("e".to_string()))) {
                        tokens.push(MathExpression::Mo(Operator::Exp));
                        tokens.push(MathExpression::Mo(Operator::Lparen));
                        superscript.flatten(tokens);
                        tokens.push(MathExpression::Mo(Operator::Rparen));
                    } else {
                        tokens.push(MathExpression::Mo(Operator::Lparen));
                        base.flatten(tokens);
                        tokens.push(MathExpression::Mo(Operator::Rparen));
                        tokens.push(MathExpression::Mo(Operator::Power));
                        tokens.push(MathExpression::Mo(Operator::Lparen));
                        superscript.flatten(tokens);
                        tokens.push(MathExpression::Mo(Operator::Rparen));
                    }
                } else {
                    base.flatten(tokens);
                    tokens.push(MathExpression::Mo(Operator::Power));
                    tokens.push(MathExpression::Mo(Operator::Lparen));
                    superscript.flatten(tokens);
                    tokens.push(MathExpression::Mo(Operator::Rparen));
                }
            }
            MathExpression::AbsoluteSup(base, superscript) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                tokens.push(MathExpression::Mo(Operator::Abs));
                tokens.push(MathExpression::Mo(Operator::Lparen));
                base.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Power));
                tokens.push(MathExpression::Mo(Operator::Lparen));
                superscript.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::Minimize(_, vec) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                for v in vec {
                    tokens.push(v.clone());
                }
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::Absolute(_operator, components) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                tokens.push(MathExpression::Mo(Operator::Abs));
                tokens.push(MathExpression::Mo(Operator::Lparen));
                components.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::Msqrt(components) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                tokens.push(MathExpression::Mo(Operator::Sqrt));
                tokens.push(MathExpression::Mo(Operator::Lparen));
                components.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            // Handles `Hat` operator with MathExpression
            MathExpression::HatComp(x) => {
                x.op.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.comp.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            MathExpression::Mover(base, over) => {
                if *over == Box::new(MathExpression::Mo(Operator::Other("^".to_string()))) {
                    base.flatten(tokens);
                    tokens.push(MathExpression::Mo(Operator::Other("^".to_string())));
                } else if MathExpression::Mo(Operator::Mean) == **over {
                    tokens.push(MathExpression::Mo(Operator::Mean));
                    tokens.push(MathExpression::Mo(Operator::Lparen));
                    base.flatten(tokens);
                    tokens.push(MathExpression::Mo(Operator::Rparen));
                }
            }

            // Handles `exp` operator with MathExpression
            MathExpression::ExpMath(x) => {
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.op.flatten(tokens);
                x.func.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }

            // Handles `Integral` operator with MathExpression
            MathExpression::Integral(x) => {
                x.op.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Lparen));
                x.integrand.flatten(tokens);
                tokens.push(MathExpression::Mo(Operator::Rparen));
            }
            t => tokens.push(t.clone()),
        }
    }
}

impl Lexer {
    fn new(input: Vec<MathExpression>) -> Lexer {
        // Flatten mrows and mfracs
        let tokens = input.iter().fold(vec![], |mut acc, x| {
            x.flatten(&mut acc);
            acc
        });

        // Insert implicit multiplication operators.
        let tokens = tokens.iter().fold(vec![], |mut acc, x| {
            if acc.is_empty() {
                acc.push(x);
            } else {
                match x {
                    // Handle left parenthesis operator '('
                    MathExpression::Mo(Operator::Lparen) => {
                        // Check last element of the accumulator.
                        if let Some(MathExpression::Mo(_)) = acc.last() {
                            // If the last element is an Mo, noop.
                        } else {
                            acc.push(&MathExpression::Mo(Operator::Multiply));
                        }
                        acc.push(x);
                    }
                    // Handle other types of operators.
                    MathExpression::Mo(Operator::Min) => {
                        acc.push(x);
                    }
                    // Handle other types of operators.
                    MathExpression::Mo(_) => {
                        acc.push(x);
                    }
                    // Handle other types of MathExpression objects.
                    t => {
                        let last_token = acc.last().unwrap();
                        match last_token {
                            MathExpression::Mo(Operator::Rparen) => {
                                // If the last item in the accumulator is a right parenthesis ')',
                                // insert a multiplication operator
                                acc.push(&MathExpression::Mo(Operator::Multiply));
                            }
                            MathExpression::Mo(_) => {
                                // If the last item in the accumulator is an Mo (but not a right
                                // parenthesis), noop
                            }
                            _ => {
                                // Otherwise, insert a multiplication operator
                                acc.push(&MathExpression::Mo(Operator::Multiply));
                            }
                        }
                        // Insert the token
                        acc.push(t);
                    }
                }
            }
            acc
        });

        // Convert MathExpression structs into Token structs.
        let mut tokens = tokens
            .into_iter()
            .map(|c| match c {
                MathExpression::Mo(op) => Token::Op(op.clone()),
                _ => Token::Atom(c.clone()),
            })
            .collect::<Vec<_>>();

        // Reverse the tokens for the Pratt parsing algorithm.
        tokens.reverse();
        Lexer { tokens }
    }

    /// Get the next Token and advance the iterator.
    fn next(&mut self) -> Token {
        self.tokens.pop().unwrap_or(Token::Eof)
    }

    /// Get the next token without advancing the iterator.
    fn peek(&self) -> Token {
        self.tokens.last().cloned().unwrap_or(Token::Eof)
    }
}

/// Following Justin's `flatten_mults` for flattening Operator::Min
fn flatten_min_op(mut expr: MathExpressionTree) -> MathExpressionTree {
    match expr {
        MathExpressionTree::Cons(ref x, ref mut y) => match x {
            Operator::Min => {
                match y[1].clone() {
                    MathExpressionTree::Cons(x1, y1) => match x1 {
                        Operator::Min => {
                            let temp1 = flatten_min_op(y1[0].clone());
                            let temp2 = flatten_min_op(y1[1].clone());
                            y.remove(1);
                            y.append(&mut [temp1, temp2].to_vec())
                        }
                        _ => {
                            let temp1 = y[1].clone();
                            y.remove(1);
                            y.append(&mut [temp1].to_vec())
                        }
                    },
                    MathExpressionTree::Atom(_x1) => {}
                }
                match y[0].clone() {
                    MathExpressionTree::Cons(x0, y0) => match x0 {
                        Operator::Min => {
                            let temp1 = flatten_min_op(y0[0].clone());
                            //let temp2 = flatten_min_op(y0[1].clone());
                            y.remove(0);
                            y.append(&mut [temp1].to_vec());
                        }
                        _ => {
                            let temp1 = y[0].clone();
                            y.remove(0);
                            y.append(&mut [temp1].to_vec())
                        }
                    },
                    MathExpressionTree::Atom(_x0) => {}
                }
            }
            _ => {
                if y.len() > 1 {
                    let temp1 = flatten_min_op(y[1].clone());
                    let temp0 = flatten_min_op(y[0].clone());
                    y.remove(1);
                    y.remove(0);
                    y.append(&mut [temp0, temp1].to_vec())
                } else {
                    let temp0 = flatten_min_op(y[0].clone());
                    y.remove(0);
                    y.append(&mut [temp0].to_vec())
                }
            }
        },
        MathExpressionTree::Atom(ref _x) => {}
    }
    expr
}

/// Construct a MathExpressionTree from a vector of MathExpression structs.
fn expr(input: Vec<MathExpression>) -> MathExpressionTree {
    let mut lexer = Lexer::new(input);
    insert_multiple_between_paren(&mut lexer);
    let mut result: MathExpressionTree = expr_bp(&mut lexer, 0);
    let mut result = flatten_min_op(result.clone());
    let mut math_vec: Vec<MathExpressionTree> = vec![];
    while lexer.next() != Token::Eof {
        let math_result = expr_bp(&mut lexer, 0);
        math_vec.push(math_result.clone());
    }

    if !math_vec.is_empty() {
        result = MathExpressionTree::Cons(Operator::Multiply, math_vec);
    }

    result
}

impl From<Vec<MathExpression>> for MathExpressionTree {
    fn from(input: Vec<MathExpression>) -> Self {
        expr(input)
    }
}

impl From<Math> for MathExpressionTree {
    fn from(input: Math) -> Self {
        expr(input.content)
    }
}

impl FromStr for MathExpressionTree {
    type Err = Error<String>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let modified_input1 = replace_unicode_with_symbols(input);
        let modified_input2 = preprocess_mathml_for_to_latex(&modified_input1).to_string();
        let modified_input3: &str = &modified_input2;
        let (_, math) = interpreted_math(modified_input3.into()).unwrap();
        Ok(MathExpressionTree::from(math))
    }
}

/// Inserts an `Operator::Multiply` token between adjacent `Operator::Lparen` and `Operator::Rparen` tokens in the given Lexer.
fn insert_multiple_between_paren(lexer: &mut Lexer) {
    let mut new_tokens = Vec::new();
    let mut iter = lexer.tokens.iter().peekable();

    while let Some(token) = iter.next() {
        new_tokens.push(token.clone());
        if let Some(next_token) = iter.peek() {
            if let Token::Op(Operator::Lparen) = token {
                if let Token::Op(Operator::Rparen) = **next_token {
                    new_tokens.push(Token::Op(Operator::Multiply).clone());
                }
            }
        }
    }
    lexer.tokens = new_tokens;
}

/// The Pratt parsing algorithm for constructing an S-expression representing an equation.
fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> MathExpressionTree {
    let mut lhs = match lexer.next() {
        Token::Atom(it) => MathExpressionTree::Atom(it),
        Token::Op(Operator::Lparen) => {
            let lhs = expr_bp(lexer, 0);
            assert_eq!(lexer.next(), Token::Op(Operator::Rparen));
            lhs
        }
        Token::Op(op) => {
            let ((), r_bp) = prefix_binding_power(&op);
            let rhs = expr_bp(lexer, r_bp);
            MathExpressionTree::Cons(op, vec![rhs])
        }
        t => panic!("bad token: {:?}", t),
    };
    loop {
        let op = match lexer.peek() {
            Token::Eof => break,
            Token::Op(op) => op,
            t => panic!("bad token: {:?}", t),
        };
        if let Some((l_bp, ())) = postfix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            lhs = MathExpressionTree::Cons(op, vec![lhs]);
            continue;
        }
        if let Some((l_bp, r_bp)) = infix_binding_power(&op) {
            if l_bp < min_bp {
                break;
            }
            lexer.next();
            lhs = {
                let rhs = expr_bp(lexer, r_bp);
                MathExpressionTree::Cons(op, vec![lhs, rhs])
            };
            continue;
        }
        break;
    }
    lhs
}

/// Table of binding powers for prefix operators.
fn prefix_binding_power(op: &Operator) -> ((), u8) {
    match op {
        Operator::Add | Operator::Subtract => ((), 9),
        Operator::Exp => ((), 21),
        Operator::Cos => ((), 21),
        Operator::Sin => ((), 21),
        Operator::Tan => ((), 21),
        Operator::Mean => ((), 25),
        Operator::SurfaceInt => ((), 25),
        Operator::Gradient(Gradient { .. }) => ((), 25),
        Operator::Derivative(Derivative { .. }) => ((), 25),
        Operator::Div => ((), 25),
        Operator::Laplacian => ((), 25),
        Operator::Abs => ((), 25),
        Operator::Sqrt => ((), 25),
        Operator::Min => ((), 25),
        Operator::Summation(Summation { .. }) => ((), 25),
        Operator::Hat(Hat { .. }) => ((), 25),
        Operator::Int(Int { .. }) => ((), 25),
        Operator::Logarithm(Logarithm { .. }) => ((), 25),
        _ => panic!("Bad operator: {:?}", op),
    }
}

/// Table of binding powers for postfix operators.
fn postfix_binding_power(op: &Operator) -> Option<(u8, ())> {
    let res = match op {
        Operator::Factorial => (11, ()),
        //Operator::DownArrow => (11, ()),
        _ => return None,
    };
    Some(res)
}

/// Table of binding powers for infix operators.
fn infix_binding_power(op: &Operator) -> Option<(u8, u8)> {
    let res = match op {
        Operator::Equals => (1, 2),
        Operator::Add => (3, 4),
        Operator::Subtract => (5, 6),
        Operator::Multiply => (7, 8),
        Operator::Divide => (9, 10),
        Operator::Compose => (14, 13),
        Operator::Power => (16, 15),
        Operator::Dot => (18, 17),
        Operator::Cross => (18, 17),
        Operator::Comma => (14, 13),
        Operator::Min => (14, 13),
        Operator::Other(op) => panic!("Unhandled operator: {}!", op),
        _ => return None,
    };
    Some(res)
}

/// Replaces Unicode representations in the input string with their corresponding symbols.
pub fn replace_unicode_with_symbols(input: &str) -> String {
    // Define a regex pattern to match Unicode representations
    let re = Regex::new(r#"&#x([0-9A-Fa-f]+);"#).unwrap();

    // Use replace_all to replace Unicode representations with corresponding symbols
    let replaced_str = re.replace_all(input, |captures: &regex::Captures| {
        // captures[0] contains the entire match, captures[1] contains the hexadecimal code
        let hex_code = &captures[1];
        // Convert hexadecimal code to u32 and then to char
        let unicode_char = u32::from_str_radix(hex_code, 16)
            .ok()
            .and_then(std::char::from_u32);

        // Replace with the Unicode character if conversion is successful, otherwise keep the original
        unicode_char.map_or_else(|| captures[0].to_string(), |c| c.to_string())
    });

    replaced_str.to_string()
}

/// Preprocesses a MathML string for conversion to LaTeX format.
///
/// This function takes a MathML string as input and performs preprocessing steps to ensure a
/// cleaner conversion to LaTeX. It removes newline characters, eliminates spaces between MathML
/// elements, and replaces occurrences of "<mi>∇</mi>" with "<mo>∇</mo>" to enhance compatibility
/// with LaTeX rendering. The resulting processed MathML string is then ready for conversion to LaTeX.
pub fn preprocess_mathml_for_to_latex(input: &str) -> String {
    // Remove all newline characters
    let no_newlines = input.replace('\n', "");

    // Remove spaces between MathML elements
    let no_spaces = Regex::new(r">\s*<")
        .unwrap()
        .replace_all(&no_newlines, "><")
        .to_string();

    no_spaces.to_string()
}

#[test]
fn test_conversion() {
    let input = "<math><mi>x</mi><mo>+</mo><mi>y</mi></math>";
    println!("Input: {input}");
    let s = input.parse::<MathExpressionTree>().unwrap();
    assert_eq!(s.to_string(), "(+ x y)");
    println!("Output: {s}\n");

    let input = "<math><mi>a</mi><mo>=</mo><mi>x</mi><mo>+</mo><mi>y</mi><mi>z</mi></math>";
    println!("Input: {input}");
    let s = input.parse::<MathExpressionTree>().unwrap();
    assert_eq!(s.to_string(), "(= a (+ x (* y z)))");
    println!("Output: {s}\n");

    let input = "<math><mi>a</mi><mo>+</mo><mo>(</mo><mo>-</mo><mi>b</mi><mo>)</mo></math>";
    println!("Input: {input}");
    let s = input.parse::<MathExpressionTree>().unwrap();
    assert_eq!(s.to_string(), "(+ a (- b))");
    println!("Output: {s}\n");

    let input =
        "<math><mn>2</mn><mi>a</mi><mo>(</mo><mi>c</mi><mo>+</mo><mi>d</mi><mo>)</mo></math>";
    println!("Input: {input}");
    let s = input.parse::<MathExpressionTree>().unwrap();
    assert_eq!(s.to_string(), "(* (* 2 a) (+ c d))");
    println!("Output: {s}\n");

    let input = "
    <math>
        <mfrac><mrow><mi>d</mi><mi>S</mi></mrow><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>
        <mo>=</mo><mo>−</mo><mi>β</mi><mi>S</mi><mi>I</mi>
        </math>
        ";
    println!("Input: {input}");
    let FirstOrderODE {
        lhs_var,
        func_of,
        with_respect_to,
        rhs,
    } = first_order_ode(input.into()).unwrap().1;
    assert_eq!(lhs_var.to_string(), "S");
    assert_eq!(func_of[0].to_string(), "");
    assert_eq!(with_respect_to.to_string(), "t");
    assert_eq!(rhs.to_string(), "(* (* (- β) S) I)");
    println!("Output: {s}\n");

    let input = "
    <math>
        <mfrac><mrow><mi>d</mi><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>
        <mo>=</mo><mo>−</mo><mi>β</mi><mi>S</mi><mi>I</mi>
        </math>
        ";
    println!("Input: {input}");
    let FirstOrderODE {
        lhs_var,
        func_of,
        with_respect_to,
        rhs,
    } = first_order_ode(input.into()).unwrap().1;
    assert_eq!(lhs_var.to_string(), "S");
    assert_eq!(func_of[0].to_string(), "t");
    assert_eq!(with_respect_to.to_string(), "t");
    assert_eq!(rhs.to_string(), "(* (* (- β) S) I)");
    println!("Output: {s}\n");
}

#[test]
fn test_to_content_mathml_example1() {
    let input = "<math><mi>x</mi><mo>+</mo><mi>y</mi></math>";
    let s = input.parse::<MathExpressionTree>().unwrap();
    let content = s.to_cmml();
    assert_eq!(content, "<apply><plus/><ci>x</ci><ci>y</ci></apply>");
}

#[test]
fn test_to_content_mathml_example2() {
    let input = "<math>
        <mfrac><mrow><mi>d</mi><mi>S</mi></mrow><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>
        <mo>=</mo><mo>−</mo><mi>β</mi><mi>S</mi><mi>I</mi>
        </math>
        ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>S</ci></apply><apply><times/><apply><times/><apply><minus/><ci>β</ci></apply><ci>S</ci></apply><ci>I</ci></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq1() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mo>-</mo>
        <mi>β</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mfrac><mrow><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow><mi>N</mi></mfrac>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    let FirstOrderODE {
        lhs_var: _,
        func_of: _,
        with_respect_to: _,
        rhs: _,
    } = first_order_ode(input.into()).unwrap().1;
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>S</ci></apply><apply><times/><apply><times/><apply><minus/><ci>β</ci></apply><ci>I</ci></apply><apply><divide/><ci>S</ci><ci>N</ci></apply></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq2() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>E</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mi>β</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mfrac><mrow><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow><mi>N</mi></mfrac>
        <mo>−</mo>
        <mi>δ</mi><mi>E</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (D(1, t) E) (- (* (* β I) (/ S N)) (* δ E)))");
    let cmml = ode.to_cmml();
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>E</ci></apply><apply><minus/><apply><times/><apply><times/><ci>β</ci><ci>I</ci></apply><apply><divide/><ci>S</ci><ci>N</ci></apply></apply><apply><times/><ci>δ</ci><ci>E</ci></apply></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq3() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mi>δ</mi><mi>E</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>−</mo>
        <mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>γ</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>−</mo>
        <mi>α</mi><mi>ρ</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    println!("cmml={:?}", cmml);
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>I</ci></apply><apply><minus/><apply><minus/><apply><times/><ci>δ</ci><ci>E</ci></apply><apply><times/><apply><times/><apply><minus/><cn>1</cn><ci>α</ci></apply><ci>γ</ci></apply><ci>I</ci></apply></apply><apply><times/><apply><times/><ci>α</ci><ci>ρ</ci></apply><ci>I</ci></apply></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq4() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>γ</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>R</ci></apply><apply><times/><apply><times/><apply><minus/><cn>1</cn><ci>α</ci></apply><ci>γ</ci></apply><ci>I</ci></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq5() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>D</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mi>α</mi>
        <mi>ρ</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>D</ci></apply><apply><times/><apply><times/><ci>α</ci><ci>ρ</ci></apply><ci>I</ci></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq6() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mo>-</mo>
        <mi>β</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mfrac><mrow><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow><mi>N</mi></mfrac>
        <mo>+</mo>
        <mi>ϵ</mi>
        <mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    assert_eq!(cmml,"<apply><eq/><apply><diff/><bvar>t</bar><ci>S</ci></apply><apply><plus/><apply><times/><apply><times/><apply><minus/><ci>β</ci></apply><ci>I</ci></apply><apply><divide/><ci>S</ci><ci>N</ci></apply></apply><apply><times/><ci>ϵ</ci><ci>R</ci></apply></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq7() {
    let input = "
    <math>
        <mfrac>
        <mrow><mi>d</mi><mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>γ</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>-</mo>
        <mi>ϵ</mi>
        <mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let ode = input.parse::<FirstOrderODE>().unwrap();
    let cmml = ode.to_cmml();
    assert_eq!(cmml, "<apply><eq/><apply><diff/><bvar>t</bar><ci>R</ci></apply><apply><minus/><apply><times/><apply><times/><apply><minus/><cn>1</cn><ci>α</ci></apply><ci>γ</ci></apply><ci>I</ci></apply><apply><times/><ci>ϵ</ci><ci>R</ci></apply></apply></apply>");
}

#[test]
fn test_content_hackathon2_scenario1_eq8() {
    let input = "
    <math>
        <mi>β</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>=</mo>
        <mi>κ</mi>
        <mi>m</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let cmml = exp.to_cmml();
    assert_eq!(
        cmml,
        "<apply><eq/><ci>β</ci><apply><times/><ci>κ</ci><ci>m</ci></apply></apply>"
    );
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= β (* κ m))");
}

#[test]
fn test_expression1() {
    let input = "<math><mi>γ</mi><mi>I</mi></math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let math = exp.to_infix_expression();
    assert_eq!(math, "(γ*I)");
}

#[test]
fn test_expression2() {
    let input = "
    <math>
        <mi>α</mi>
        <mi>ρ</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let math = exp.to_infix_expression();
    println!("math = {:?}", math);
    assert_eq!(math, "((α*ρ)*I)");
}

#[test]
fn test_expression3() {
    let input = "
    <math>
        <mi>β</mi>
        <mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mfrac><mrow><mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo></mrow><mi>N</mi></mfrac>
        <mo>−</mo>
        <mi>δ</mi><mi>E</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(- (* (* β I) (/ S N)) (* δ E))");
    let math = exp.to_infix_expression();
    assert_eq!(math, "(((β*I)*(S/N))-(δ*E))")
}

#[test]
fn test_expression4() {
    let input = "
    <math>
        <mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>γ</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>-</mo>
        <mi>ϵ</mi>
        <mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let math = exp.to_infix_expression();
    assert_eq!(math, "((((1-α)*γ)*I)-(ϵ*R))")
}

#[test]
fn test_mfrac() {
    let input = "
    <math>
        <mfrac><mi>S</mi><mi>N</mi></mfrac>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let math = exp.to_infix_expression();
    let s_exp = exp.to_string();
    assert_eq!(math, "(S/N)");
    assert_eq!(s_exp, "(/ S N)");
}

#[test]
fn test_superscript() {
    let input = "
    <math>
        <msup>
        <mi>x</mi>
        <mn>3</mn>
        </msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(^ x 3)");
}

#[test]
fn test_msup_exp() {
    let input = "
    <math>
        <msup>
        <mi>e</mi>
        <mrow><mo>-</mo><mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>γ</mi><mi>I</mi></mrow>
        </msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(exp (* (* (- (- 1 α)) γ) I))");
}

#[test]
fn test_trig_cos() {
    let input = "
    <math>
        <mrow>
        <mi>cos</mi>
        <mi>x</mi>
        </mrow>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let cmml = exp.to_cmml();
    assert_eq!(cmml, "<apply><cos/><ci>x</ci></apply>");
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Cos x)");
}

#[test]
fn test_trig_another_cos() {
    let input = "
    <math>
        <mrow>
        <mi>cos</mi>
        <mo>(</mo>
        <mi>x</mi>
        <mo>)</mo>
        </mrow>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Cos x)");
}

#[test]
fn test_mover_mean() {
    let input = "
    <math>
        <mover>
        <mi>T</mi>
        <mo>¯</mo>
        </mover>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Mean T)");
}

#[test]
fn test_one_dimensional_ebm() {
    let input = "
    <math>
        <mi>C</mi>
        <mfrac>
        <mrow><mi>∂</mi><mi>T</mi><mo>(</mo><mi>ϕ</mi><mo>,</mo><mi>t</mi><mo>)</mo></mrow>
        <mrow><mi>∂</mi><mi>t</mi></mrow>
        </mfrac>
        <mo>=</mo>
        <mo>(</mo><mn>1</mn><mo>−</mo><mi>α</mi><mo>)</mo><mi>S</mi><mo>(</mo><mi>ϕ</mi><mo>,</mo><mi>t</mi><mo>)</mo>
        <mo>-</mo>
        <mo>(</mo><mi>A</mi><mo>+</mo><mi>B</mi><mi>T</mi><mo>(</mo><mi>ϕ</mi><mo>,</mo><mi>t</mi><mo>)</mo><mo>)</mo>
        <mo>+</mo>
        <mfrac>
        <mn>D</mn>
        <mrow><mi>cos</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mfrac>
        <mi>∂</mi>
        <mrow><mi>∂</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mo>(</mo>
        <mrow><mi>cos</mi><mi>ϕ</mi></mrow>
        <mfrac>
        <mrow><mi>∂</mi><mi>T</mi></mrow>
        <mrow><mi>∂</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (* C (PD(1, t) T)) (+ (- (* (- 1 α) S) (+ A (* B T))) (* (/ D (Cos ϕ)) (PD(1, ϕ) (* (Cos ϕ) (PD(1, ϕ) T))))))");
}

#[test]
fn test_derivative_with_func_comp_in_parenthesis() {
    let input = "
    <math>
        <mfrac>
        <mi>∂</mi>
        <mrow><mi>∂</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mo>(</mo>
        <mrow><mi>cos</mi><mi>ϕ</mi></mrow>
        <mfrac>
        <mrow><mi>∂</mi><mi>T</mi></mrow>
        <mrow><mi>∂</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(PD(1, ϕ) (* (Cos ϕ) (PD(1, ϕ) T)))");
}

#[test]
fn test_derivative_with_func_inside_parenthesis() {
    let input = "
    <math>
        <mfrac>
        <mi>∂</mi>
        <mrow><mi>∂</mi><mi>ϕ</mi></mrow>
        </mfrac>
        <mo>(</mo>
        <mi>T</mi>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(PD(1, ϕ) T)");
}

#[test]
fn test_absolute_value() {
    let input = "
    <math>
        <mo>|</mo><mo>&#x2207;</mo><mi>H</mi><mo>|</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    let latex_exp = exp.to_latex();
    assert_eq!(s_exp, "(Abs (Grad H))");
    assert_eq!(latex_exp, "\\left|\\nabla{H}\\right|");
}
#[test]
fn test_another_absolute() {
    let input = "
    <math>
        <mo>|</mo><mi>H</mi><mo>|</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Abs H)");
}

#[test]
fn test_grad() {
    let input = "
    <math>
        <mi>&#x2207;</mi><mi>H</mi>
        </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let cmml = exp.to_cmml();
    assert_eq!(cmml, "<apply><grad/><ci>H</ci></apply>");
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Grad H)");
}

#[test]
fn test_absolute_as_msup() {
    let input = "
    <math>
        <mo>|</mo><mrow><mo>&#x2207;</mo><mi>H</mi></mrow>
        <msup><mo>|</mo>
        <mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow></msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(^ (Abs (Grad H)) (- n 1))");
}

#[test]
fn test_equation_halfar_dome() {
    let input = "
    <math>
        <mfrac><mrow><mi>∂</mi><mi>H</mi></mrow><mrow><mi>∂</mi><mi>t</mi></mrow></mfrac>
        <mo>=</mo>
        <mo>&#x2207;</mo>
        <mo>&#x22c5;</mo>
        <mo>(</mo>
        <mi>Γ</mi>
        <msup><mi>H</mi><mrow><mi>n</mi><mo>+</mo><mn>2</mn></mrow></msup>
        <mo>|</mo><mrow><mo>&#x2207;</mo><mi>H</mi></mrow>
        <msup><mo>|</mo>
        <mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow></msup>
        <mo>&#x2207;</mo><mi>H</mi>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= (PD(1, t) H) (Div (* (* (* Γ (^ H (+ n 2))) (^ (Abs (Grad H)) (- n 1))) (Grad H))))"
    );
    assert_eq!(exp.to_latex(), "\\frac{\\partial H}{\\partial t}=\\nabla \\cdot {(\\Gamma*H^{n+2}*\\left|\\nabla{H}\\right|^{n-1}*\\nabla{H})}");
}

#[test]
fn test_halfar_dome_rhs() {
    let input = "
    <math>
        <mo>&#x2207;</mo>
        <mo>&#x22c5;</mo>
        <mo>(</mo>
        <mi>Γ</mi>
        <msup><mi>H</mi><mrow><mi>n</mi><mo>+</mo><mn>2</mn></mrow></msup>
        <mo>|</mo><mrow><mo>&#x2207;</mo><mi>H</mi></mrow>
        <msup><mo>|</mo>
        <mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow></msup>
        <mo>&#x2207;</mo><mi>H</mi>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(Div (* (* (* Γ (^ H (+ n 2))) (^ (Abs (Grad H)) (- n 1))) (Grad H)))"
    );
}

#[test]
fn test_func_paren() {
    let input = "
    <math>
        <mo>(</mo><mi>a</mi><mo>+</mo><mo>(</mo><mi>b</mi><mo>+</mo><mi>c</mi><mo>)</mo>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(+ a (+ b c))");
}

#[test]
fn test_func_paren_and_times() {
    let input = "
    <math>
        <mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(* S (+ a b))");
}

#[test]
fn test_func_a_plus_b() {
    let input = "
    <math>
        <mo>(</mo><mi>a</mi><mo>+</mo><mi>b</mi><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(+ a b)");
}

#[test]
fn test_func_paren_and_times_another() {
    let input = "
    <math>
        <mi>S</mi><mo>(</mo><mi>t</mi><mo>)</mo>
        <mo>(</mo><mi>a</mi><mi>I</mi><mo>(</mo><mi>t</mi><mo>)</mo><mo>+</mo><mi>b</mi><mi>R</mi><mo>(</mo><mi>t</mi><mo>)</mo><mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(* S (+ (* a I) (* b R)))");
}

#[test]
fn test_divergence() {
    let input = "
    <math>
        <mo>&#x2207;</mo><mo>&#x22c5;</mo>
        <mi>H</mi>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let cmml = exp.to_cmml();
    assert_eq!(cmml, "<apply><divergence/><ci>H</ci></apply>");
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Div H)");
}

#[test]
fn test_combination() {
    let input = "
    <math>
        <mi>S</mi>
        <mrow><mi>n</mi><mo>+</mo><mn>4</mn></mrow>
        <mrow><mi>i</mi><mo>-</mo><mn>3</mn></mrow>
        <msup><mi>H</mi><mrow><mi>m</mi><mo>-</mo><mn>2</mn></mrow></msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(* (* (* S (+ n 4)) (- i 3)) (^ H (- m 2)))");
}

#[test]
fn test_mi_multiply() {
    let input = "
    <math>
        <mi>A</mi>
        <msup><mi>ρ</mi><mi>n</mi></msup>
        <msup><mi>g</mi><mi>n</mi></msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    println!("s_exp={:?}", s_exp);
}
#[allow(dead_code)] // used in tests I believe
fn test_unicode_conversion() {
    let input1 = "&#x039B; is a Greek letter.";
    let input2 = "&#x03bb; is another Greek letter.";
    let input3 = "Λ and λ are Greek letters.";
    let input4 = "Lambda and lambda are English representations of Greek letters.";

    assert_eq!(unicode_to_latex(input1), "\\Lambda is a Greek letter.");
    assert_eq!(
        unicode_to_latex(input2),
        "\\lambda is another Greek letter."
    );
    assert_eq!(
        unicode_to_latex(input3),
        "\\Lambda and \\lambda are Greek letters."
    );
    assert_eq!(
        unicode_to_latex(input4),
        "\\Lambda and \\lambda are English representations of Greek letters."
    );
}
#[test]
fn test_sexp2latex() {
    let input = "
    <math>
        <mi>&#x03bb;</mi>
        <mrow><mi>n</mi><mo>+</mo><mn>4</mn></mrow>
        <mrow><mi>i</mi><mo>-</mo><mn>3</mn></mrow>
        <msup><mi>H</mi><mrow><mi>m</mi><mo>-</mo><mn>2</mn></mrow></msup>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(latex_exp, "\\lambda*(n+4)*(i-3)*H^{m-2}");
}

#[test]
fn test_sexp2latex_derivative() {
    let input = "
    <math>
    <mfrac>
        <mrow><mi>d</mi><mi>S</mi></mrow>
        <mrow><mi>d</mi><mi>t</mi></mrow>
        </mfrac>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let latex_exp = exp.to_latex();
    assert_eq!(latex_exp, "\\frac{d S}{dt}");
}

#[test]
fn test_equation_halfar_dome_to_latex() {
    let input = "
    <math>
        <mfrac><mrow><mi>∂</mi><mi>H</mi></mrow><mrow><mi>∂</mi><mi>t</mi></mrow></mfrac>
        <mo>=</mo>
        <mo>&#x2207;</mo>
        <mo>&#x22c5;</mo>
        <mo>(</mo>
        <mi>Γ</mi>
        <msup><mi>H</mi><mrow><mi>n</mi><mo>+</mo><mn>2</mn></mrow></msup>
        <mo>|</mo><mrow><mo>&#x2207;</mo><mi>H</mi></mrow>
        <msup><mo>|</mo>
        <mrow><mi>n</mi><mo>−</mo><mn>1</mn></mrow></msup>
        <mo>&#x2207;</mo><mi>H</mi>
        <mo>)</mo>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(
        latex_exp,
        "\\frac{\\partial H}{\\partial t}=\\nabla \\cdot {(\\Gamma*H^{n+2}*\\left|\\nabla{H}\\right|^{n-1}*\\nabla{H})}"
    );
}

#[test]
fn test_equation_halfar_dome_8_1_to_latex() {
    let input = "
    <math>
      <mfrac>
        <mrow>
          <mi>&#x2202;</mi>
          <mi>H</mi>
        </mrow>
        <mrow>
          <mi>&#x2202;</mi>
          <mi>t</mi>
        </mrow>
      </mfrac>
      <mo>=</mo>
      <mi>&#x2207;</mi>
      <mo>&#x22C5;</mo>
      <mo>(</mo>
      <mi>&#x0393;</mi>
      <msup>
        <mi>H</mi>
        <mrow>
          <mi>n</mi>
          <mo>+</mo>
          <mn>2</mn>
        </mrow>
      </msup>
      <mo>|</mo>
      <mi>&#x2207;</mi>
      <mi>H</mi>
      <msup>
        <mo>|</mo>
        <mrow>
          <mi>n</mi>
          <mo>&#x2212;</mo>
          <mn>1</mn>
        </mrow>
      </msup>
      <mi>&#x2207;</mi>
      <mi>H</mi>
      <mo>)</mo>
    </math>
    ";
    let modified_input1 = &replace_unicode_with_symbols(input).to_string();
    let modified_input2 = &preprocess_mathml_for_to_latex(modified_input1).to_string();
    let exp = modified_input2.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(
        latex_exp,
        "\\frac{\\partial H}{\\partial t}=\\nabla \\cdot {(\\Gamma*H^{n+2}*\\left|\\nabla{H}\\right|^{n-1}*\\nabla{H})}"
    );
}

#[test]
fn test_equation_halfar_dome_8_2_to_latex() {
    let input = "
    <math>
      <mi>&#x0393;</mi>
      <mo>=</mo>
      <mfrac>
        <mn>2</mn>
        <mrow>
          <mi>n</mi>
          <mo>+</mo>
          <mn>2</mn>
        </mrow>
      </mfrac>
      <mi>A</mi>
      <mo>(</mo>
      <mi>&#x03C1;</mi>
      <mi>g</mi>
      <msup>
        <mo>)</mo>
        <mi>n</mi>
      </msup>
    </math>
    ";
    let modified_input1 = &replace_unicode_with_symbols(input).to_string();
    let modified_input2 = &preprocess_mathml_for_to_latex(modified_input1).to_string();
    let exp = modified_input2.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(latex_exp, "\\Gamma=\\frac{2}{n+2}*A*(\\rho*g)^{n}");
}

#[test]
fn test_equation_halfar_dome_8_3_to_latex() {
    let input = "
    <math>
      <mi>H</mi>
      <mo>(</mo>
      <mi>t</mi>
      <mo>,</mo>
      <mi>r</mi>
      <mo>)</mo>
      <mo>=</mo>
      <msub>
        <mi>H</mi>
        <mn>0</mn>
      </msub>
      <msup>
        <mrow>
          <mo>(</mo>
          <mfrac>
            <msub>
              <mi>t</mi>
              <mn>0</mn>
            </msub>
            <mi>t</mi>
          </mfrac>
          <mo>)</mo>
        </mrow>
        <mrow>
          <mfrac>
            <mn>1</mn>
            <mn>9</mn>
          </mfrac>
        </mrow>
      </msup>
      <msup>
        <mrow>
          <mo>[</mo>
          <mn>1</mn>
          <mo>&#x2212;</mo>
          <msup>
            <mrow>
              <mo>(</mo>
              <msup>
                <mrow>
                  <mo>(</mo>
                  <mfrac>
                    <msub>
                      <mi>t</mi>
                      <mn>0</mn>
                    </msub>
                    <mi>t</mi>
                  </mfrac>
                  <mo>)</mo>
                </mrow>
                <mrow>
                  <mfrac>
                    <mn>1</mn>
                    <mrow>
                      <mn>18</mn>
                    </mrow>
                  </mfrac>
                </mrow>
              </msup>
              <mfrac>
                <mi>r</mi>
                <msub>
                  <mi>R</mi>
                  <mn>0</mn>
                </msub>
              </mfrac>
              <mo>)</mo>
            </mrow>
            <mrow>
              <mfrac>
                <mn>4</mn>
                <mn>3</mn>
              </mfrac>
            </mrow>
          </msup>
          <mo>]</mo>
        </mrow>
        <mrow>
          <mfrac>
            <mn>3</mn>
            <mn>7</mn>
          </mfrac>
        </mrow>
      </msup>
    </math>
    ";
    let modified_input1 = &replace_unicode_with_symbols(input).to_string();
    let modified_input2 = &preprocess_mathml_for_to_latex(modified_input1).to_string();
    let exp = modified_input2.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(
        latex_exp,
        "H(t,r)=H_{0}*(\\frac{t_{0}}{t})^{\\frac{1}{9}}*(1-((\\frac{t_{0}}{t})^{\\frac{1}{18}}*\\frac{r}{R_{0}})^{\\frac{4}{3}})^{\\frac{3}{7}}"
    );
}

#[test]
fn test_equation_halfar_dome_8_4_to_latex() {
    let input = "
    <math>
      <msub>
        <mi>t</mi>
        <mn>0</mn>
      </msub>
      <mo>=</mo>
      <mfrac>
        <mn>1</mn>
        <mrow>
          <mn>18</mn>
          <mi>&#x0393;</mi>
        </mrow>
      </mfrac>
      <msup>
        <mrow>
          <mo>(</mo>
          <mfrac>
            <mn>7</mn>
            <mn>4</mn>
          </mfrac>
          <mo>)</mo>
        </mrow>
        <mn>3</mn>
      </msup>
      <mfrac>
        <msubsup>
          <mi>R</mi>
          <mn>0</mn>
          <mn>4</mn>
        </msubsup>
        <msubsup>
          <mi>H</mi>
          <mn>0</mn>
          <mn>7</mn>
        </msubsup>
      </mfrac>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    println!("s_exp={:?}", s_exp);
    let modified_input1 = &replace_unicode_with_symbols(input).to_string();
    let modified_input2 = &preprocess_mathml_for_to_latex(modified_input1).to_string();
    let exp = modified_input2.parse::<MathExpressionTree>().unwrap();
    let latex_exp = exp.to_latex();
    assert_eq!(
        latex_exp,
        "t_{0}=\\frac{1}{18*\\Gamma}*(\\frac{7}{4})^{3}*\\frac{R_{0}^{4}}{H_{0}^{7}}"
    );
}

#[test]
fn new_test_halfar_whitespace() {
    let input = "
    <math>
      <msub>
        <mi>t</mi>
        <mn>0</mn>
      </msub>
      <mo>=</mo>
      <mfrac>
        <mn>1</mn>
        <mrow>
          <mn>18</mn>
          <mi>&#x0393;</mi>
        </mrow>
      </mfrac>
      <msup>
        <mrow>
          <mo>(</mo>
          <mfrac>
            <mn>7</mn>
            <mn>4</mn>
          </mfrac>
          <mo>)</mo>
        </mrow>
        <mn>3</mn>
      </msup>
      <mfrac>
        <msubsup>
          <mi>R</mi>
          <mn>0</mn>
          <mn>4</mn>
        </msubsup>
        <msubsup>
          <mi>H</mi>
          <mn>0</mn>
          <mn>7</mn>
        </msubsup>
      </mfrac>
    </math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= t_{0} (* (* (/ 1 (* 18 Γ)) (^ (/ 7 4) 3)) (/ R_{0}^{4} H_{0}^{7})))"
    );
}

#[test]
fn test_equation_with_mtext() {
    let input = "<math><msub><mrow><mi mathvariant=\"script\">L</mi></mrow><mrow><mtext>reg</mtext></mrow></msub><mo>=</mo><msub><mrow><mi mathvariant=\"script\">L</mi></mrow><mrow><mi>d</mi><mn>1</mn></mrow></msub><mo>+</mo><msub><mrow><mi mathvariant=\"script\">L</mi></mrow><mrow><mi>d</mi><mn>2</mn></mrow></msub></math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= L_{reg} (+ L_{d1} L_{d2}))");
}

#[test]
fn new_msqrt_test_function() {
    let input = "<math>
    <msqrt>
    <mn>4</mn>
    <mi>a</mi>
    <mi>c</mi>
    </msqrt>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(√ (* (* 4 a) c))");
    assert_eq!(exp.to_latex(), "\\sqrt{4*a*c}");
}
#[test]
fn new_quadratic_equation() {
    let input = "<math>
  <mi>x</mi>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <mo>&#x2212;</mo>
      <mi>b</mi>
      <mo>&#x2212;</mo>
      <msqrt>
        <msup>
          <mi>b</mi>
          <mn>2</mn>
        </msup>
        <mo>&#x2212;</mo>
        <mn>4</mn>
        <mi>a</mi>
        <mi>c</mi>
      </msqrt>
    </mrow>
    <mrow>
      <mn>2</mn>
      <mi>a</mi>
    </mrow>
  </mfrac>
</math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    println!("s_exp={:?}", s_exp);
    assert_eq!(
        s_exp,
        "(= x (/ (- (- b) (√ (- (^ b 2) (* (* 4 a) c)))) (* 2 a)))"
    );
    assert_eq!(exp.to_latex(), "x=\\frac{(-b)-\\sqrt{b^{2}-4*a*c}}{2*a}");
}

#[test]
fn test_dot_in_derivative() {
    let input = "<math>
    <mrow>
    <mover>
    <mi>S</mi>
    <mo>&#x02D9;</mo>
    </mover>
    </mrow>
    <mo>(</mo>
    <mi>t</mi>
    <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(D(1, t) S)");
}

#[test]
fn test_sidarthe_equation() {
    let input = "<math>
  <mrow>
    <mover>
      <mi>S</mi>
      <mo>&#x02D9;</mo>
    </mover>
  </mrow>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>=</mo>
  <mo>&#x2212;</mo>
  <mi>S</mi>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>(</mo>
  <mi>&#x03B1;</mi>
  <mi>I</mi>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>+</mo>
  <mi>&#x03B2;</mi>
  <mi>D</mi>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>+</mo>
  <mi>&#x03B3;</mi>
  <mi>A</mi>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>+</mo>
  <mi>&#x03B4;</mi>
  <mi>R</mi>
  <mo>(</mo>
  <mi>t</mi>
  <mo>)</mo>
  <mo>)</mo>
</math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= (D(1, t) S) (* (- S) (+ (+ (+ (* α I) (* β D)) (* γ A)) (* δ R))))"
    );
}

#[test]
fn test_change_in_variable() {
    let input = "<math>
    <mi>&#x0394;</mi>
    <mi>t</mi>
</math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    println!("latex_exp = {:?}", exp.to_latex());
    assert_eq!(s_exp, "Δt");
}
#[test]
fn test_heating_rate() {
    let input = "<math>
    <msub>
    <mi>Q</mi>
    <mi>i</mi>
    </msub>
    <mo>=</mo>
    <mrow>
    <mo>(</mo>
    <msub>
    <mi>T</mi>
    <mi>i</mi>
    </msub>
    <mo>−</mo>
    <msub>
      <mi>T</mi>
      <mrow>
        <mi>i</mi>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msub>
    <mo>)</mo>
  </mrow>
    <mo>∕</mo>
  <mrow>
    <mo>(</mo>
    <msub>
      <mi>C</mi>
      <mi>p</mi>
    </msub>
    <mi>&#x0394;</mi>
    <mi>t</mi>
    <mo>)</mo>
  </mrow>
</math>
    ";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= Q_{i} (/ (- T_{i} T_{i-1}) (* C_{p} Δt)))");
}

#[test]
fn test_sum_munderover() {
    let input = "<math>
    <munderover>
    <mo>&#x2211;</mo>
    <mrow>
    <mi>l</mi>
    <mo>=</mo>
    <mi>k</mi>
    </mrow>
    <mi>K</mi>
    </munderover>
    <mi>S</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Sum_{l=k}^{K} S)");
    println!("exp.to_latex()={:?}", exp.to_latex());
    assert_eq!(exp.to_latex(), "\\sum_{l=k}^{K}S");
}

#[test]
fn test_sum_munder() {
    let input = "<math>
    <munder>
    <mo>&#x2211;</mo>
    <mrow>
    <mi>l</mi>
    <mo>=</mo>
    <mi>k</mi>
    </mrow>
    </munder>
    <mi>S</mi>
    <mo>(</mo>
    <mi>l</mi>
    <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Sum_{l=k} S)");
    println!("exp.to_latex()={:?}", exp.to_latex());
    assert_eq!(exp.to_latex(), "\\sum_{l=k}S(l)");
}

#[test]
fn test_hydrostatic() {
    let input = "<math>
    <msub>
    <mi>Φ</mi>
    <mi>k</mi>
    </msub>
    <mo>=</mo>
    <msub>
    <mi>Φ</mi>
    <mi>s</mi>
    </msub>
    <mo>+</mo>
    <mi>R</mi>
    <munderover>
    <mo>∑</mo>
    <mrow>
    <mi>l</mi>
    <mo>=</mo>
    <mi>k</mi>
    </mrow>
    <mi>K</mi>
    </munderover>
    <msub>
    <mi>H</mi>
    <mrow>
    <mi>k</mi>
    <mi>l</mi>
    </mrow>
    </msub>
    <msub>
    <mi>T</mi>
    <mrow>
    <mi>v</mi>
    <mi>l</mi>
    </mrow>
    </msub>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    //println!("s_exp={:?}", s_exp);
    assert_eq!(
        s_exp,
        "(= Φ_{k} (+ Φ_{s} (* R (Sum_{l=k}^{K} (* H_{kl} T_{vl})))))"
    );
    println!("exp.to_latex()={:?}", exp.to_latex());
    assert_eq!(
        exp.to_latex(),
        "\\Phi_{k}=\\Phi_{s}+R*\\sum_{l=k}^{K}H_{kl}*T_{vl}"
    )
}

#[test]
fn test_temperature_evolution() {
    let input = "<math>
    <mfrac>
    <mrow>
    <mi>Δ</mi>
    <msub>
    <mi>s</mi>
    <mi>i</mi>
    </msub>
    </mrow>
    <mrow>
    <mi>Δ</mi>
    <mi>t</mi>
    </mrow>
    </mfrac>
    <mo>∕</mo>
    <msub>
    <mi>C</mi>
    <mi>p</mi>
    </msub>
    <mo>=</mo>
    <mfrac>
    <mrow>
    <mo>(</mo>
    <msub>
    <mi>s</mi>
    <mi>i</mi>
    </msub>
    <mo>−</mo>
    <msub>
    <mi>s</mi>
    <mrow>
    <mi>i</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>)</mo>
    </mrow>
    <mrow>
    <mi>Δ</mi>
    <mi>t</mi>
    </mrow>
    </mfrac>
    <mo>∕</mo>
    <msub>
    <mi>C</mi>
    <mi>p</mi>
    </msub>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= (/ (/ Δs_{i} Δt) C_{p}) (/ (/ (- s_{i} s_{i-1}) Δt) C_{p}))"
    );
}

#[test]
fn test_cross_product() {
    let input = "<math>
    <mi>f</mi>
    <mo>&#x00D7;</mo>
    <mi>u</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(× f u)");
    assert_eq!(exp.to_latex(), "f \\cross u")
}
#[test]
fn test_dot_product() {
    let input = "<math>
    <mi>f</mi>
    <mo>&#x22c5;</mo>
    <mi>u</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(⋅ f u)");
    assert_eq!(exp.to_latex(), "f \\cdot u")
}

#[test]
fn test_partial_with_msub_t() {
    let input = "<math>
    <msub>
    <mi>∂</mi>
    <mi>t</mi>
    </msub>
    <mi>S</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(∂_{t}) S)");
}

#[test]
fn test_dry_static_energy() {
    let input = "<math>
    <msub>
    <mi>s</mi>
    <mi>i</mi>
    </msub>
    <mo>=</mo>
    <msub>
    <mi>s</mi>
    <mrow>
    <mi>i</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>+</mo>
    <mo>(</mo>
    <mi>Δ</mi>
    <mi>t</mi>
    <mo>)</mo>
    <msub>
    <mi>Q</mi>
    <mi>i</mi>
    </msub>
    <mrow>
    <mo>(</mo>
    <msub>
    <mi>s</mi>
    <mrow>
    <mi>i</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>,</mo>
    <msub>
    <mi>T</mi>
    <mrow>
    <mi>i</mi>
    <mo>−</mo>
    <mn>1</mn>
    </mrow>
    </msub>
    <mo>,</mo>
    <msub>
    <mi>Φ</mi>
    <mrow>
        <mi>i</mi>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msub>
    <mo>,</mo>
    <msub>
      <mi>q</mi>
      <mrow>
        <mi>i</mi>
        <mo>−</mo>
        <mn>1</mn>
      </mrow>
    </msub>
    <mo>,</mo>
    <mo>…</mo>
    <mo>)</mo>
  </mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= s_{i} (+ s_{i-1} (* Δt Q_{i})))");
}

#[test]
fn test_hat_operator() {
    let input = "<math>
    <mi>ζ</mi>
    <mover>
    <mi>z</mi>
    <mo>^</mo>
    </mover>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    println!("{:?}", exp);
    println!("{:?}", exp.to_latex());
    println!("{:?}", s_exp);
    //assert_eq!(s_exp, "(Hat(z) ζ)");
    assert_eq!(s_exp, "(* ζ Hat(z))");
    assert_eq!(exp.to_latex(), "\\zeta*\\hat{z}");
}

#[test]
fn test_vector_invariant_form() {
    let input = "<math>
    <msub>
    <mi>∂</mi>
    <mi>t</mi>
    </msub>
    <mi>u</mi>
    <mo>+</mo>
    <mo>(</mo>
    <mi>ζ</mi>
    <mrow>
    <mover>
    <mi>z</mi>
    <mo>^</mo>
    </mover>
    </mrow>
    <mo>+</mo>
    <mi>f</mi>
    <mo>)</mo>
    <mo>×</mo>
    <mi>u</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mi>&#x2207;</mi>
    <mrow>
    <mo>[</mo>
    <mi>g</mi>
    <mo>(</mo>
    <mi>h</mi>
    <mo>+</mo>
    <mi>b</mi>
    <mo>)</mo>
    <mo>+</mo>
    <mfrac>
    <mn>1</mn>
    <mn>2</mn>
    </mfrac>
    <mi>u</mi>
    <mo>⋅</mo>
    <mi>u</mi>
    <mo>]</mo>
    </mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    //assert_eq!(s_exp, "(= (+ (PD(1, t) u) (× (+ (Hat(z) ζ) f) u)) (- (Grad (+ (* g (+ h b)) (* (/ 1 2) (⋅ u u))))))");
    assert_eq!(s_exp, "(= (+ (∂_{t}) u) (× (+ (* ζ Hat(z)) f) u)) (- (Grad (+ (* g (+ h b)) (* (/ 1 2) (⋅ u u))))))");
}

#[test]
fn test_mi_dot_gradient() {
    let input = "<math>
    <mo>(</mo>
    <mi>v</mi>
    <mo>&#x22c5;</mo>
    <mi>&#x2207;</mi>
    <mo>)</mo>
    <mi>u</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(* (⋅ v Grad) u)");
}

#[test]
fn test_gradient_sub() {
    let input = "<math>
    <msub>
    <mi>∇</mi>
    <mi>h</mi>
    </msub>
    <mo>(</mo>
    <mi>p</mi>
    <mo>+</mo>
    <mi>g</mi>
    <mi>η</mi>
    <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Grad_h (+ p (* g η)))");
    assert_eq!(exp.to_latex(), "\\nabla_{h}{(p+g*\\eta)}");
}

#[test]
fn test_momentum_conservation() {
    let input = "<math>
    <msub>
    <mi>∂</mi>
    <mi>t</mi>
    </msub>
    <mi>u</mi>
    <mo>=</mo>
    <mo>−</mo>
    <mo>(</mo>
    <mi>v</mi>
    <mo>⋅</mo>
    <mi>∇</mi>
    <mo>)</mo>
    <mi>u</mi>
    <mo>−</mo>
    <mi>f</mi>
    <mo>&#x00D7;</mo>
    <mi>u</mi>
    <mo>−</mo>
    <msub>
    <mi>∇</mi>
    <mi>h</mi>
    </msub>
    <mo>(</mo>
    <mi>p</mi>
    <mo>+</mo>
    <mi>g</mi>
    <mi>η</mi>
    <mo>)</mo>
    <mo>−</mo>
    <mi>∇</mi>
    <mo>⋅</mo>
    <mi>τ</mi>
    <mo>+</mo>
    <msub>
    <mi>F</mi>
    <mrow>
    <mi>u</mi>
    </mrow>
    </msub>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (∂_{t}) u) (+ (- (- (- (* (- (⋅ v Grad)) u) (× f u)) (Grad_h (+ p (* g η)))) (Div τ)) F_{u}))");
}

#[test]
fn test_down_arrow() {
    let input = "<math>
    <msup>
        <mi>I</mi>
        <mo>&#x2193;</mo>
      </msup>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "I^\\downarrow");
    assert_eq!(exp.to_latex(), "I^\\downarrow");
}

#[test]
fn test_down_arrow2() {
    let input = "<math>
    <msup>
        <mi>I</mi>
        <mo>&#x2193;</mo>
      </msup>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "I^\\downarrow");
    assert_eq!(exp.to_latex(), "I^\\downarrow(\\lambda)");
}

#[test]
fn test_integral1() {
    let input = "<math>
    <msubsup>
        <mo>&#x222b;</mo>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>1</mn>
          </msub>
        </mrow>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>2</mn>
          </msub>
        </mrow>
      </msubsup>
    <msup>
    <mi>x</mi>
    <mn>2</mn>
    </msup>
    <mi>d</mi>
    <mi>x</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    println!("s_exp={:?}", s_exp);
    assert_eq!(s_exp, "(Int_{λ_{1}}^{λ_{2}}(x) (^ x 2))");
    println!("exp.to_latex()={:?}", exp.to_latex());
    assert_eq!(
        exp.to_latex(),
        "\\int_{\\lambda_{1}}^{\\lambda_{2}}x^{2} dx"
    );
}

#[test]
fn test_integral2() {
    let input = "<math>
    <mrow>
      <msubsup>
        <mo>&#x222b;</mo>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>1</mn>
          </msub>
        </mrow>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>2</mn>
          </msub>
        </mrow>
      </msubsup>
      <mi>ω</mi>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
        <mi>I</mi>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
      <msub>
        <mi>α</mi>
        <mrow>
          <mtext>sno </mtext>
        </mrow>
      </msub>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
        <mi>d</mi>
      <mi>λ</mi>
    </mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    println!("s_exp={:?}", s_exp);
    assert_eq!(s_exp, "(Int_{λ_{1}}^{λ_{2}}(λ) (* (* ω I) α_{sno}))");
    println!("exp.to_latex()={:?}", exp.to_latex());
    assert_eq!(exp.to_latex(), "\\int_{\\lambda_{1}}^{\\lambda_{2}}\\omega(\\lambda)*I(\\lambda)*\\alpha_{sno}(\\lambda) dλ");
}

#[test]
fn test_snowpack_optics() {
    let input = "<math>
    <mrow>
    <mover>
      <mi>ω</mi>
      <mo>¯</mo>
    </mover>
  </mrow>
  <mo>=</mo>
  <mfrac>
    <mrow>
      <msubsup>
        <mo>&#x222b;</mo>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>1</mn>
          </msub>
        </mrow>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>2</mn>
          </msub>
        </mrow>
      </msubsup>
      <mi>ω</mi>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
      <msup>
        <mi>I</mi>
        <mo>&#x2193;</mo>
      </msup>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
      <msub>
        <mi>α</mi>
        <mrow>
          <mtext>sno </mtext>
        </mrow>
      </msub>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
        <mi>d</mi>
      <mi>λ</mi>
    </mrow>
    <mrow>
      <msubsup>
        <mo>&#x222b;</mo>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>1</mn>
          </msub>
        </mrow>
        <mrow>
          <msub>
            <mi>λ</mi>
            <mn>2</mn>
          </msub>
        </mrow>
      </msubsup>
      <msup>
        <mi>I</mi>
        <mo>&#x2193;</mo>
      </msup>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
      <msub>
        <mi>α</mi>
        <mrow>
          <mtext>sno </mtext>
        </mrow>
      </msub>
      <mo>(</mo>
      <mi>λ</mi>
      <mo>)</mo>
        <mi>d</mi>
      <mi>λ</mi>
    </mrow>
  </mfrac>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (Mean ω) (/ (Int_{λ_{1}}^{λ_{2}}(λ) (* (* ω I^\\downarrow) α_{sno})) (Int_{λ_{1}}^{λ_{2}}(λ) (* I^\\downarrow α_{sno}))))");
    assert_eq!(exp.to_latex(), "\\langle \\omega \\rangle=\\frac{\\int_{\\lambda_{1}}^{\\lambda_{2}}\\omega(\\lambda)*I^\\downarrow(\\lambda)*\\alpha_{sno}(\\lambda) dλ}{\\int_{\\lambda_{1}}^{\\lambda_{2}}I^\\downarrow(\\lambda)*\\alpha_{sno}(\\lambda) dλ}");
}

#[test]
fn test_laplacian() {
    let input = "<math>
    <msup><mi>&#x2207;</mi><mn>2</mn></msup>
    <mi>T</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Laplacian T)");
    assert_eq!(exp.to_latex(), "\\nabla^2 T");
}

#[test]
fn test_fourier_law_heat_equation_1_1() {
    let input = "<math>
    <mi>Q</mi>
    <mo>=</mo>
    <mfrac><msub><mi>k</mi><mi>T</mi></msub><mi>&#x03C1;</mi></mfrac>
    <msup><mi>&#x2207;</mi><mn>2</mn></msup>
    <mi>T</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= Q (* (/ k_{T} ρ) (Laplacian T)))");
    assert_eq!(exp.to_latex(), "Q=\\frac{k_{T}}{\\rho}*\\nabla^2 T");
}

#[test]
fn test_closed_surface_integral() {
    let input = "<math>
    <msubsup><mtext>∯</mtext><mi>S</mi></msubsup>
    <mrow><mi>&#x2207;</mi><mi>T</mi></mrow>
    <mo>&#x22C5;</mo><mi>T</mi>
    <mi>d</mi><mi>S</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(SurfaceInt (* (⋅ (Grad T) T) dS))");
    assert_eq!(exp.to_latex(), "\\oiint_S \\nabla{T} \\cdot T*dS");
}

#[test]
fn test_fourier_law_heat_equation_2() {
    let input = "<math>
    <mfrac><mrow><mi>&#x2202;</mi><mi>Q</mi></mrow><mrow><mi>&#x2202;</mi><mi>t</mi></mrow></mfrac>
    <mo>=</mo>
    <mo>&#x2212;</mo><mi>k</mi>
    <msubsup><mtext>∯</mtext><mi>S</mi></msubsup>
    <mrow><mi>&#x2207;</mi><mi>T</mi></mrow>
    <mo>&#x22C5;</mo><mi>d</mi><mi>S</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= (PD(1, t) Q) (* (- k) (SurfaceInt (⋅ (Grad T) dS))))"
    );
    assert_eq!(
        exp.to_latex(),
        "\\frac{\\partial Q}{\\partial t}=(-k)*\\oiint_S \\nabla{T} \\cdot dS"
    );
}

#[test]
fn test_fourier_law_heat_equation_1_2() {
    let input = "<math>
    <mi>Q</mi>
    <mo>=</mo>
    <mfrac><msub><mi>k</mi><mi>T</mi></msub><mi>&#x03C1;</mi></mfrac>
    <mrow>
    <mo>(</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>x</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>+</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>y</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>+</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>z</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>)</mo>
    </mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= Q (* (/ k_{T} ρ) (+ (+ (PD(2, x) T) (PD(2, y) T)) (PD(2, z) T))))"
    );
    assert_eq!(exp.to_latex(), "Q=\\frac{k_{T}}{\\rho}*(\\frac{\\partial^2 T}{\\partial x^2}+\\frac{\\partial^2 T}{\\partial y^2}+\\frac{\\partial^2 T}{\\partial z^2})");
}

#[test]
fn test_second_order_derivative() {
    let input = "<math>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>x</mi><mn>2</mn></msup></mrow>
    </mfrac>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(PD(2, x) T)");
    assert_eq!(exp.to_latex(), "\\frac{\\partial^2 T}{\\partial x^2}");
}

#[test]
fn test_fourier_law_heat_equation_3() {
    let input = "<math>
    <mfrac><mrow><mi>&#x2202;</mi><mi>T</mi></mrow><mrow><mi>&#x2202;</mi><mi>t</mi></mrow></mfrac>
    <mo>=</mo>
    <mi>&#x03B1;</mi>
    <mrow>
    <mo>(</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>x</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>+</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>y</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>+</mo>
    <mfrac>
    <mrow><msup><mi>&#x2202;</mi><mn>2</mn></msup><mi>T</mi></mrow>
    <mrow><mi>&#x2202;</mi><msup><mi>z</mi><mn>2</mn></msup></mrow>
    </mfrac>
    <mo>)</mo>
    </mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(
        s_exp,
        "(= (PD(1, t) T) (* α (+ (+ (PD(2, x) T) (PD(2, y) T)) (PD(2, z) T))))"
    );
    assert_eq!(exp.to_latex(), "\\frac{\\partial T}{\\partial t}=\\alpha*(\\frac{\\partial^2 T}{\\partial x^2}+\\frac{\\partial^2 T}{\\partial y^2}+\\frac{\\partial^2 T}{\\partial z^2})");
}

#[test]
fn test_conservation_of_momentum_fluid_equation_1() {
    let input = "<math>
    <mfrac>
    <mrow><mi>D</mi><mi>u</mi></mrow>
    <mrow><mi>D</mi><mi>t</mi></mrow>
    </mfrac>
    <mo>&#x2212;</mo>
    <mi>f</mi>
    <mi>v</mi>
    <mo>=</mo>
    <mo>&#x2212;</mo>
    <mi>g</mi>
    <mfrac>
    <mrow><mi>&#x2202;</mi><mi>&#x03B7;</mi></mrow>
    <mrow><mi>&#x2202;</mi><mi>x</mi></mrow>
    </mfrac>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (- (D(1, t) u) (* f v)) (* (- g) (PD(1, x) η)))");
    assert_eq!(
        exp.to_latex(),
        "\\frac{D u}{Dt}-f*v=(-g)*\\frac{\\partial \\eta}{\\partial x}"
    );
}

#[test]
fn test_vector_notation() {
    let input = "
    <math>
    <mover><mi>v</mi><mo>&#x2192;</mo></mover>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "v");
    assert_eq!(exp.to_latex(), "\\vec{v}");
}

#[test]
fn test_conservation_of_mass_equation_2() {
    let input = "<math>
    <mfrac>
    <mrow><mi>&#x2202;</mi><mi>&#x03C1;</mi></mrow>
    <mrow><mi>&#x2202;</mi><mi>t</mi></mrow>
    </mfrac>
    <mo>=</mo>
    <mo>&#x2212;</mo>
    <mi>&#x2207;</mi>
    <mo>&#x22C5;</mo>
    <mo>(</mo>
    <mi>&#x03C1;</mi>
    <mrow><mover><mi>&#x03BC;</mi><mo>&#x2192;</mo></mover></mrow>
    <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (PD(1, t) ρ) (- (Div (* ρ μ))))");
    assert_eq!(
        exp.to_latex(),
        "\\frac{\\partial \\rho}{\\partial t}=-\\nabla \\cdot {(\\rho*\\vec{\\mu})}"
    );
}

#[test]
fn test_new_equation1() {
    let input = "<math>
    <msub><mi>p</mi><mrow><mi>i</mi><mi>j</mi></mrow></msub><mo>=</mo><mrow><mi>R</mi></mrow><mo>(</mo><msub><mi>t</mi><mi>j</mi></msub><mo>)</mo><mi>w</mi><mo>(</mo><msub><mi>t</mi><mi>i</mi></msub><mo>&#x2212;</mo><msub><mi>t</mi><mi>j</mi></msub><mo>)</mo><mo>/</mo><mi>&#x03BB;</mi><mo>(</mo><msub><mi>t</mi><mi>i</mi></msub><mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= p_{ij} (* (* R w) (/ (- t_{i} t_{j}) λ)))");
    assert_eq!(
        exp.to_latex(),
        "p_{ij}=R(t_{j})*w*\\frac{t_{i}-t_{j}}{\\lambda(t_{i})}"
    );
}

#[test]
fn test_minimize() {
    let input = "<math>
  <mi>min</mi>
  <mo>&#x2061;</mo>
  <mo>(</mo>
  <msub>
    <mi>A</mi>
    <mi>c</mi>
  </msub>
  <mo>,</mo>
  <msub>
    <mi>A</mi>
    <mi>j</mi>
  </msub>
  <mo>,</mo>
  <msub>
    <mi>A</mi>
    <mi>p</mi>
  </msub>
  <mo>)</mo>
  <mo>.</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Min A_{j} A_{p} A_{c})");
    //assert_eq!(s_exp, "(= A_{n} (- (Min A_{c} A_{j} A_{p}) R_{d}))");
    assert_eq!(exp.to_latex(), "\\min \\{ A_{j},A_{p},A_{c}\\}");
}

#[test]
fn test_minimize_mi() {
    let input = "<math>
  <mi>min</mi>
  <mo>&#x2061;</mo>
  <mo>(</mo>
    <mi>A</mi>
  <mo>,</mo>
    <mi>B</mi>
  <mo>,</mo>
    <mi>C</mi>
  <mo>)</mo>
  <mo>.</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Min B C A)");
    //assert_eq!(s_exp, "(= A_{n} (- (Min A_{c} A_{j} A_{p}) R_{d}))");
    assert_eq!(exp.to_latex(), "\\min \\{ B,C,A\\}");
}

#[test]
fn test_clm4_5_8_2() {
    let input = "<math>
  <msub>
    <mi>A</mi>
    <mi>n</mi>
  </msub>
  <mo>=</mo>
  <mi>min</mi>
  <mo>&#x2061;</mo>
  <mo>(</mo>
  <msub>
    <mi>A</mi>
    <mi>c</mi>
  </msub>
  <mo>,</mo>
  <msub>
    <mi>A</mi>
    <mi>j</mi>
  </msub>
  <mo>,</mo>
  <msub>
    <mi>A</mi>
    <mi>p</mi>
  </msub>
  <mo>)</mo>
  <mo>&#x2212;</mo>
  <msub>
    <mi>R</mi>
    <mi>d</mi>
  </msub>
  <mo>.</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= A_{n} (- (Min A_{j} A_{p} A_{c}) R_{d}))");
    assert_eq!(
        exp.to_latex(),
        "A_{n}=(\\min \\{ A_{j},A_{p},A_{c}\\})-R_{d}"
    );
}

#[test]
fn test_clm4_5_8_12_3() {
    let input = "<math>
  <msub>
    <mi>f</mi>
    <mi>L</mi>
  </msub>
  <mo>(</mo>
  <msub>
    <mi>T</mi>
    <mi>v</mi>
  </msub>
  <mo>)</mo>
  <mo>=</mo>
  <mn>1</mn>
  <mo>+</mo>
  <mi>exp</mi>
  <mo>[</mo>
  <msub>
    <mi>s</mi>
    <mn>3</mn>
  </msub>
  <mo>(</mo>
  <msub>
    <mi>s</mi>
    <mn>4</mn>
  </msub>
  <mo>&#x2212;</mo>
  <msub>
    <mi>T</mi>
    <mi>v</mi>
  </msub>
  <mo>)</mo>
  <mo>]</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= f_{L} (+ 1 (exp (* s_{3} (- s_{4} T_{v})))))");
    assert_eq!(
        exp.to_latex(),
        "f_{L}(T_{v})=1+\\mathrm{exp}{s_{3}*(s_{4}-T_{v})}"
    );
}

#[test]
fn test_log() {
    let input = "<math>
  <mrow><mi>log</mi><mi>x</mi></mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Log x)");
    assert_eq!(exp.to_latex(), "\\log x");
}

#[test]
fn test_ln() {
    let input = "<math>
  <mrow><mi>ln</mi><mi>x</mi></mrow>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Ln x)");
    assert_eq!(exp.to_latex(), "\\ln x");
}

#[test]
fn test_log_10base() {
    let input = "<math>
    <msub><mi>log</mi><mn>10</mn></msub><mi>x</mi>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(Log_{10} x)");
    assert_eq!(exp.to_latex(), "\\log_{10}x");
}

#[test]
fn test_conservation_of_mass_5() {
    let input = "<math>
    <mfrac><mi>d</mi><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>
    <mi>ln</mi><mo>&#x2061;</mo>
    <mo>(</mo>
    <mrow><mi>d</mi><mi>M</mi></mrow>
    <mo>)</mo>
    <mo>=</mo>
    <mn>0</mn>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(= (D(1, t) (Ln dM)) 0)");
    assert_eq!(exp.to_latex(), "\\frac{d \\ln dM}{dt}=0");
}

#[test]
fn test_msup_with_perp() {
    let input = "<math>
    <msup><mi>u</mi><mrow><mi>&#x22A5;</mi></mrow></msup>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    let s_exp = exp.to_string();
    assert_eq!(s_exp, "(^ u ⊥)");
    assert_eq!(exp.to_latex(), "u^{⊥}");
}

#[test]
fn test_msubsup_msub_content() {
    let input = "<math>
    <msubsup>
    <mi>β</mi>
    <mrow>
      <mi>c</mi>
      <mi>W</mi>
    </mrow>
    <mrow>
      <mi>A</mi>
      <mi>e</mi>
      <mi>r</mi>
      <mi>o</mi>
    </mrow>
    </msubsup>
    <mo>+</mo>
    <msub>
    <mi>A</mi>
    <mi>n</mi>
    </msub>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    let content = exp.to_cmml();
    println!("content={:?}", content);
    assert_eq!(s_exp, "(+ β_{cW}^{Aero} A_{n})");
    assert_eq!(exp.to_latex(), "\\beta_{cW}^{Aero}+A_{n}");
}

#[test]
fn test_new_eval_test() {
    let input = "<math>
    <mfrac><mrow><mi>d</mi><msub><mi>s</mi><mrow><mi>c</mi></mrow></msub></mrow><mrow><mi>d</mi><mi>t</mi></mrow></mfrac>
    <mo>=</mo>
    <mi>α</mi>
    <mo>∗</mo>
    <msub><mi>r</mi><mrow><mi>c</mi></mrow></msub>
    <mo>−</mo>
    <msub><mi>s</mi><mrow><mi>c</mi></mrow></msub><mo>∗</mo>
    <mo>(</mo>
    <msubsup><mi>β</mi><mrow><mi>c</mi><mi>c</mi></mrow><mrow><mi>D</mi><mi>c</mi></mrow></msubsup>
    <mo>∗</mo>
    <msub><mi>i</mi><mrow><mi>c</mi></mrow></msub>
    <mo>+</mo>
    <msubsup>
    <mi>β</mi>
    <mrow><mi>c</mi><mi>c</mi></mrow>
    <mrow><mi>A</mi><mi>e</mi><mi>r</mi><mi>o</mi></mrow>
    </msubsup>
    <mo>∗</mo>
    <msub><mi>i</mi><mrow><mi>c</mi></mrow></msub>
    <mo>+</mo>
    <msubsup>
    <mi>β</mi>
    <mrow><mi>c</mi><mi>W</mi></mrow>
    <mrow><mi>A</mi><mi>e</mi><mi>r</mi><mi>o</mi></mrow>
    </msubsup>
    <mo>∗</mo>
    <msub><mi>i</mi><mrow><mi>W</mi></mrow></msub>
    <mo>+</mo>
    <msubsup><mi>β</mi><mrow><mi>c</mi><mi>W</mi></mrow><mrow><mi>D</mi><mi>c</mi></mrow></msubsup>
    <mo>∗</mo>
    <msub><mi>i</mi><mrow><mi>W</mi></mrow></msub>
    <mo>+</mo>
    <msubsup><mi>β</mi><mrow><mi>H</mi><mi>c</mi></mrow><mrow><mi>A</mi><mi>e</mi><mi>r</mi><mi>o</mi></mrow></msubsup>
    <mo>∗</mo>
    <msub><mi>i</mi><mrow><mi>H</mi></mrow></msub>
    <mo>)</mo>
    </math>";
    let exp = input.parse::<MathExpressionTree>().unwrap();
    println!("exp={:?}", exp);
    let s_exp = exp.to_string();
    let content = exp.to_cmml();
    println!("content={:?}", content);
    assert_eq!(s_exp, "(= (D(1, t) s_{c}) (- (* α r_{c}) (* s_{c} (+ (+ (+ (+ (* β_{cc}^{Dc} i_{c}) (* β_{cc}^{Aero} i_{c})) (* β_{cW}^{Aero} i_{W})) (* β_{cW}^{Dc} i_{W})) (* β_{Hc}^{Aero} i_{H})))))");
    assert_eq!(exp.to_latex(), "\\frac{d s_{c}}{dt}=\\alpha*r_{c}-s_{c}*(\\beta_{cc}^{Dc}*i_{c}+\\beta_{cc}^{Aero}*i_{c}+\\beta_{cW}^{Aero}*i_{W}+\\beta_{cW}^{Dc}*i_{W}+\\beta_{Hc}^{Aero}*i_{H})");
}
