//! This file is joker resolver rs
//!
//! - Resolver
//!     This struct used static resolve all maybe stmt and expr. handle closure env wait question.  
//!
//!

use super::{
    ast::{
        Assign, Binary, BlockStmt, BreakStmt, Call, ContinueStmt, Expr, ExprAcceptor, ExprStmt,
        ExprVisitor, ForStmt, FunStmt, Grouping, IfStmt, Lambda, Literal, Logical, PrintStmt,
        ReturnStmt, Stmt, StmtAcceptor, StmtVisitor, Trinomial, Unary, VarStmt, Variable,
        WhileStmt,
    },
    error::{JokerError, ReportError},
    interpreter::Interpreter,
    token::{Token, TokenType},
};
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

pub trait StmtResolver<T> {
    fn resolve(&self, stmt: &Stmt) -> Result<T, JokerError>;
    fn resolve_block(&self, stmts: &[Stmt]) -> Result<T, JokerError>;
    fn resolve_function(&self, stmt: &FunStmt, fun_state: FunStatus) -> Result<T, JokerError>;
}

pub trait ExprResolver<T> {
    fn resolve(&self, expr: &Expr) -> Result<T, JokerError>;
    fn resolve_local(&self, expr: Expr, name: &Token) -> Result<T, JokerError>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunStatus {
    Outer,
    Inner,
}

pub struct Resolver {
    interpreter: Rc<Interpreter>,
    scopes_stack: RefCell<Vec<HashMap<String, bool>>>,
    current_fun: RefCell<FunStatus>,
}

impl Resolver {
    pub fn new(interpreter: Rc<Interpreter>) -> Resolver {
        Resolver {
            interpreter,
            scopes_stack: RefCell::new(Vec::new()),
            current_fun: RefCell::new(FunStatus::Outer),
        }
    }
    fn begin_scope(&self) {
        self.scopes_stack.borrow_mut().push(HashMap::new());
    }
    fn end_scope(&self) {
        self.scopes_stack.borrow_mut().pop();
    }
    fn declare(&self, name: &Token) -> Result<(), JokerError> {
        if let Some(scope) = self.scopes_stack.borrow_mut().last_mut() {
            match scope.entry(name.lexeme.clone()) {
                Entry::Occupied(_) => {
                    return Err(JokerError::Resolver(ResolverError::Var(
                        VarError::RedefineError(RedefineError::report_error(
                            name,
                            format!(
                                "Variable '{}' is already declared in this scope.",
                                name.lexeme
                            ),
                        )),
                    )));
                }
                Entry::Vacant(entry) => {
                    entry.insert(false);
                    return Ok(());
                }
            }
        }
        Ok(())
    }
    fn define(&self, name: &Token) -> Result<(), JokerError> {
        if let Some(scope) = self.scopes_stack.borrow_mut().last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
        Ok(())
    }
}

// Resolver
impl StmtResolver<()> for Resolver {
    fn resolve(&self, stmt: &Stmt) -> Result<(), JokerError> {
        stmt.accept(self)
    }
    fn resolve_block(&self, stmts: &[Stmt]) -> Result<(), JokerError> {
        for stmt in stmts {
            StmtResolver::resolve(self, stmt)?;
        }
        Ok(())
    }
    fn resolve_function(&self, stmt: &FunStmt, fun_state: FunStatus) -> Result<(), JokerError> {
        let fun_enclosing: FunStatus = self.current_fun.replace(fun_state);

        self.begin_scope();
        for param in &stmt.params {
            self.declare(param)?;
            self.define(param)?;
        }
        StmtResolver::resolve_block(self, &stmt.body)?;
        self.end_scope();

        self.current_fun.replace(fun_enclosing);
        Ok(())
    }
}
impl ExprResolver<()> for Resolver {
    fn resolve(&self, expr: &Expr) -> Result<(), JokerError> {
        expr.accept(self)?;
        Ok(())
    }
    fn resolve_local(&self, expr: Expr, name: &Token) -> Result<(), JokerError> {
        for (layer, scope) in self.scopes_stack.borrow().iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, layer); // 解决闭包：环境冻结
                return Ok(());
            }
        }
        Ok(())
    }
}

// Visitor
impl StmtVisitor<()> for Resolver {
    fn visit_if(&self, stmt: &IfStmt) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &stmt.condition)?;
        StmtResolver::resolve(self, &stmt.then_branch)?;
        if let Some(else_branch) = &stmt.else_branch {
            StmtResolver::resolve(self, else_branch)?;
        }
        Ok(())
    }
    fn visit_var(&self, stmt: &VarStmt) -> Result<(), JokerError> {
        self.declare(&stmt.name)?;
        // Expr all have value, so not condition if ..then.
        ExprResolver::resolve(self, &stmt.value)?;
        self.define(&stmt.name)?;
        Ok(())
    }
    fn visit_for(&self, stmt: &ForStmt) -> Result<(), JokerError> {
        if let Some(initializer) = &stmt.initializer {
            StmtResolver::resolve(self, initializer)?;
        }
        ExprResolver::resolve(self, &stmt.condition)?;
        if let Some(increment) = &stmt.increment {
            ExprResolver::resolve(self, increment)?;
        }
        StmtResolver::resolve(self, &stmt.body)?;
        Ok(())
    }
    fn visit_fun(&self, stmt: &FunStmt) -> Result<(), JokerError> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name)?;
        StmtResolver::resolve_function(self, stmt, FunStatus::Inner)?;
        Ok(())
    }
    fn visit_expr(&self, stmt: &ExprStmt) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &stmt.expr)?;
        Ok(())
    }
    fn visit_print(&self, stmt: &PrintStmt) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &stmt.expr)?;
        Ok(())
    }
    fn visit_block(&self, stmt: &BlockStmt) -> Result<(), JokerError> {
        self.begin_scope();
        self.resolve_block(&stmt.stmts)?;
        self.end_scope();
        Ok(())
    }
    fn visit_while(&self, stmt: &WhileStmt) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &stmt.condition)?;
        StmtResolver::resolve(self, &stmt.body)?;
        Ok(())
    }
    fn visit_break(&self, _stmt: &BreakStmt) -> Result<(), JokerError> {
        Ok(())
    }
    fn visit_return(&self, stmt: &ReturnStmt) -> Result<(), JokerError> {
        if self.current_fun.borrow().eq(&FunStatus::Outer) {
            return Err(JokerError::Resolver(ResolverError::KeyWord(
                KeyWordError::Pos(PosError::report_error(
                    &stmt.keyword,
                    String::from("Cannot use 'return' outside of a function."),
                )),
            )));
        }
        ExprResolver::resolve(self, &stmt.value)?;
        Ok(())
    }
    fn visit_continue(&self, _stmt: &ContinueStmt) -> Result<(), JokerError> {
        Ok(())
    }
}

impl ExprVisitor<()> for Resolver {
    fn visit_call(&self, expr: &Call) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.callee)?;
        for arg in &expr.arguments {
            ExprResolver::resolve(self, arg)?;
        }
        Ok(())
    }
    fn visit_unary(&self, expr: &Unary) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.r_expr)?;
        Ok(())
    }
    fn visit_binary(&self, expr: &Binary) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.l_expr)?;
        ExprResolver::resolve(self, &expr.r_expr)?;
        Ok(())
    }
    fn visit_assign(&self, expr: &Assign) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.value)?;
        ExprResolver::resolve_local(self, Expr::Assign(expr.clone()), &expr.name)?;
        Ok(())
    }
    fn visit_lambda(&self, expr: &Lambda) -> Result<(), JokerError> {
        StmtResolver::resolve(self, &expr.body)?;
        Ok(())
    }
    fn visit_literal(&self, _expr: &Literal) -> Result<(), JokerError> {
        Ok(())
    }
    fn visit_logical(&self, expr: &Logical) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.l_expr)?;
        ExprResolver::resolve(self, &expr.r_expr)?;
        Ok(())
    }
    fn visit_grouping(&self, expr: &Grouping) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.expr)?;
        Ok(())
    }
    fn visit_variable(&self, expr: &Variable) -> Result<(), JokerError> {
        if let Some(scope) = self.scopes_stack.borrow().last() {
            if let Some(init_status) = scope.get(&expr.name.lexeme) {
                if !init_status {
                    return Err(JokerError::Resolver(ResolverError::Var(
                        VarError::InitError(InitError::report_error(
                            &expr.name,
                            String::from("Can't read local variable in its own initializer."),
                        )),
                    )));
                }
            }
        }
        self.resolve_local(Expr::Variable(expr.clone()), &expr.name)?;
        Ok(())
    }
    fn visit_trinomial(&self, expr: &Trinomial) -> Result<(), JokerError> {
        ExprResolver::resolve(self, &expr.condition)?;
        ExprResolver::resolve(self, &expr.l_expr)?;
        ExprResolver::resolve(self, &expr.r_expr)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum ResolverError {
    Var(VarError),
    KeyWord(KeyWordError),
}
impl ReportError for ResolverError {
    fn report(&self) {
        match self {
            ResolverError::Var(var) => ReportError::report(var),
            ResolverError::KeyWord(keyword) => ReportError::report(keyword),
        }
    }
}

#[derive(Debug)]
pub enum VarError {
    InitError(InitError),
    RedefineError(RedefineError),
}
impl ReportError for VarError {
    fn report(&self) {
        match self {
            VarError::InitError(init) => ReportError::report(init),
            VarError::RedefineError(redefine) => ReportError::report(redefine),
        }
    }
}

#[derive(Debug)]
pub struct InitError {
    line: usize,
    where_: String,
    msg: String,
}
impl InitError {
    pub fn new(token: &Token, msg: String) -> InitError {
        let where_: String = if token.ttype == TokenType::Eof {
            String::from(" at end")
        } else {
            format!(" at '{}'", token.lexeme)
        };
        InitError {
            line: token.line,
            where_,
            msg,
        }
    }
    pub fn report_error(token: &Token, msg: String) -> InitError {
        let arg_limit = InitError::new(token, msg);
        arg_limit.report();
        arg_limit
    }
}
impl ReportError for InitError {
    fn report(&self) {
        eprintln!(
            "[line {}] where: '{}', \n\tmsg: {}\n",
            self.line, self.where_, self.msg
        );
    }
}

#[derive(Debug)]
pub struct RedefineError {
    line: usize,
    where_: String,
    msg: String,
}
impl RedefineError {
    pub fn new(token: &Token, msg: String) -> RedefineError {
        let where_: String = if token.ttype == TokenType::Eof {
            String::from(" at end")
        } else {
            format!(" at '{}'", token.lexeme)
        };
        RedefineError {
            line: token.line,
            where_,
            msg,
        }
    }
    pub fn report_error(token: &Token, msg: String) -> RedefineError {
        let arg_limit = RedefineError::new(token, msg);
        arg_limit.report();
        arg_limit
    }
}
impl ReportError for RedefineError {
    fn report(&self) {
        eprintln!(
            "[line {}] where: '{}', \n\tmsg: {}\n",
            self.line, self.where_, self.msg
        );
    }
}

#[derive(Debug)]
pub enum KeyWordError {
    Pos(PosError),
}
impl ReportError for KeyWordError {
    fn report(&self) {
        match self {
            KeyWordError::Pos(pos) => ReportError::report(pos),
        }
    }
}

#[derive(Debug)]
pub struct PosError {
    line: usize,
    where_: String,
    msg: String,
}
impl PosError {
    pub fn new(token: &Token, msg: String) -> PosError {
        let where_: String = if token.ttype == TokenType::Eof {
            String::from(" at end")
        } else {
            format!(" at '{}'", token.lexeme)
        };
        PosError {
            line: token.line,
            where_,
            msg,
        }
    }
    pub fn report_error(token: &Token, msg: String) -> PosError {
        let arg_limit = PosError::new(token, msg);
        arg_limit.report();
        arg_limit
    }
}
impl ReportError for PosError {
    fn report(&self) {
        eprintln!(
            "[line {}] where: '{}', \n\tmsg: {}\n",
            self.line, self.where_, self.msg
        );
    }
}
