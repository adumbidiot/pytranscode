use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};
use rustpython_parser::ast::{
    Comparison,
    ExpressionType,
    Number,
    Operator,
    StatementType,
    StringGroup,
};
use std::{
    collections::HashSet,
    convert::TryInto,
};

pub trait CodeGen {
    fn get_tokens(&self, ctx: &mut CodeGenContext) -> TokenStream;
}

pub struct Scope {
    vars: HashSet<String>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: HashSet::new(),
        }
    }

    pub fn add_var(&mut self, var: String) {
        self.vars.insert(var);
    }

    pub fn has_var(&self, var: &str) -> bool {
        self.vars.contains(var)
    }
}

pub struct CodeGenContext {
    scopes: Vec<Scope>,
}

impl CodeGenContext {
    pub fn new() -> Self {
        CodeGenContext {
            scopes: vec![Scope::new()],
        }
    }

    pub fn add_var(&mut self, var: String) {
        self.scopes.last_mut().unwrap().add_var(var);
    }

    pub fn has_var(&self, var: &str) -> bool {
        self.scopes.last().unwrap().has_var(var)
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}

pub struct CodeGenerator {
    ctx: CodeGenContext,
    code: TokenStream,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            ctx: CodeGenContext::new(),
            code: TokenStream::new(),
        }
    }

    pub fn add_many<T: CodeGen, I: Iterator<Item = T>>(&mut self, iter: I) {
        let ctx = &mut self.ctx;
        self.code.extend(iter.map(|obj| obj.get_tokens(ctx)));
    }

    pub fn export_tokens(&self) -> TokenStream {
        self.code.clone()
    }
}

pub struct PythonStatement<'a>(&'a StatementType);

impl<'a> From<&'a StatementType> for PythonStatement<'a> {
    fn from(s: &'a StatementType) -> Self {
        PythonStatement(s)
    }
}

impl<'a> CodeGen for PythonStatement<'a> {
    fn get_tokens(&self, ctx: &mut CodeGenContext) -> TokenStream {
        let inner = match self.0 {
            StatementType::Expression { expression } => {
                PythonExpression::from(&expression.node).get_tokens(ctx)
            }
            StatementType::Assign { targets, value } => {
                if targets.len() != 1 {
                    unimplemented!("{:#?}", targets);
                }
                let name = PythonExpression::from(&targets[0].node).get_tokens(ctx);
                let value = PythonExpression::from(&value.node).get_tokens(ctx);

                let name_str = name.to_string();

                if ctx.has_var(&name_str) {
                    quote! {
                        #name = #value
                    }
                } else {
                    ctx.add_var(name_str);
                    quote! {
                        let #name = #value
                    }
                }
            }
            StatementType::If { test, body, orelse } => {
                ctx.push_scope();
                let test_expr = PythonExpression::from(&test.node).get_tokens(ctx);
                let body = body
                    .iter()
                    .map(|expr| PythonStatement::from(&expr.node).get_tokens(ctx))
                    .collect::<Vec<_>>();
                if let Some(orelse) = orelse {
                    let orelse = orelse
                        .iter()
                        .map(|expr| PythonStatement::from(&expr.node).get_tokens(ctx));
                    quote! {
                        if #test_expr {
                            #(#body);*
                        }else{
                            #(#orelse);*
                        }
                    }
                } else {
                    quote! {
                        if #test_expr {
                            #(#body);*
                        }
                    }
                }
            }
            _ => unimplemented!("{:#?}", self.0),
        };
        quote! {
            #inner;
        }
    }
}

pub struct PythonExpression<'a>(&'a ExpressionType);

impl<'a> From<&'a ExpressionType> for PythonExpression<'a> {
    fn from(e: &'a ExpressionType) -> Self {
        PythonExpression(e)
    }
}

impl<'a> CodeGen for PythonExpression<'a> {
    fn get_tokens(&self, ctx: &mut CodeGenContext) -> TokenStream {
        match self.0 {
            ExpressionType::Call {
                function,
                args,
                keywords,
            } => {
                if keywords.len() != 0 {
                    unimplemented!("{:#?}", keywords);
                }
                let func = PythonExpression::from(&function.node).get_tokens(ctx);
                let args = args
                    .iter()
                    .map(|expr| PythonExpression::from(&expr.node).get_tokens(ctx));
                quote! {
                    #func(&[#((&#args)),*])
                }
            }
            ExpressionType::Identifier { name } => {
                let name = format_ident!("{}", name);
                quote! { #name }
            }
            ExpressionType::String { value } => match value {
                StringGroup::Constant { value } => {
                    quote! { PyObject::from(#value) }
                }
                _ => unimplemented!("{:#?}", self.0),
            },
            ExpressionType::Number { value } => match value {
                Number::Integer { value } => {
                    let mut data = value.to_signed_bytes_le();
                    if data.len() > 8 {
                        unimplemented!();
                    }
                    data.resize(8, 0);
                    let val = i64::from_le_bytes(data[..8].try_into().unwrap());
                    quote! {
                        PyObject::from(#val)
                    }
                }
                Number::Float { value } => {
                    quote! {
                        PyObject::from(#value)
                    }
                }
                _ => unimplemented!("{:#?}", self.0),
            },
            ExpressionType::IfExpression { test, body, orelse } => {
                unimplemented!("{:#?}", self.0);
                if orelse.node != ExpressionType::None {
                    unimplemented!("{:#?}", orelse);
                }
                let test_expr = PythonExpression::from(&test.node).get_tokens(ctx);
                let body = PythonExpression::from(&body.node).get_tokens(ctx);

                quote! {
                    if #test_expr {
                        #body
                    }
                }
            }
            ExpressionType::Compare { vals, ops } => {
                if ops.len() != 1 {
                    unimplemented!("{:#?}", ops);
                }

                if vals.len() != 2 {
                    unimplemented!("{:#?}", vals);
                }
                let left = PythonExpression::from(&vals[0].node).get_tokens(ctx);
                let right = PythonExpression::from(&vals[1].node).get_tokens(ctx);
                let op = match ops[0] {
                    Comparison::Equal => quote! { == },
                    Comparison::Greater => quote! { > },
                    _ => unimplemented!("{:#?}", ops[0]),
                };

                quote! {
                    #left #op #right
                }
            }
            ExpressionType::Binop { a, op, b } => {
                let a = PythonExpression::from(&a.node).get_tokens(ctx);
                let b = PythonExpression::from(&b.node).get_tokens(ctx);
                match op {
                    Operator::Mult => {
                        return quote! { #a.mul(&#b) };
                    }
                    Operator::Add => {
                        return quote! { #a.add(&#b) };
                    }
                    Operator::Sub => {
                        return quote! { #a.sub(&#b) };
                    }
                    Operator::Div => {
                        return quote! { #a.div(&#b) };
                    }
                    Operator::Pow => {
                        return quote! { #a.pow(&#b) };
                    }
                    _ => unimplemented!("{:#?}", op),
                }
            }
            _ => unimplemented!("{:#?}", self.0),
        }
    }
}
