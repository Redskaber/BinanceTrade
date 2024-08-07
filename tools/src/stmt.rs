use super::ast::Expr;
use super::token::Token;
use super::error::JokerError;


pub enum Stmt {
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarStmt(VarStmt),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
}

pub struct ExprStmt {
    pub expr: Expr,
}

pub struct PrintStmt {
    pub expr: Expr,
}

pub struct VarStmt {
    pub name: Token,
    pub value: Expr,
}

pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Box<Stmt>,
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl<T> StmtVisitor<T> for Stmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        match self {
            Stmt::ExprStmt(exprstmt) => exprstmt.accept(visitor),
            Stmt::PrintStmt(printstmt) => printstmt.accept(visitor),
            Stmt::VarStmt(varstmt) => varstmt.accept(visitor),
            Stmt::BlockStmt(blockstmt) => blockstmt.accept(visitor),
            Stmt::IfStmt(ifstmt) => ifstmt.accept(visitor),
            Stmt::WhileStmt(whilestmt) => whilestmt.accept(visitor),
        }
    }
}

pub trait StmtVisitor<T> {
    fn visit_exprstmt(&self, expr: &ExprStmt) -> Result<T, JokerError>;
    fn visit_printstmt(&self, expr: &PrintStmt) -> Result<T, JokerError>;
    fn visit_varstmt(&self, expr: &VarStmt) -> Result<T, JokerError>;
    fn visit_blockstmt(&self, expr: &BlockStmt) -> Result<T, JokerError>;
    fn visit_ifstmt(&self, expr: &IfStmt) -> Result<T, JokerError>;
    fn visit_whilestmt(&self, expr: &WhileStmt) -> Result<T, JokerError>;
}

pub trait StmtAcceptor<T> {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError>;
}

impl<T> StmtAcceptor<T> for ExprStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_exprstmt(self)
    }
}

impl<T> StmtAcceptor<T> for PrintStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_printstmt(self)
    }
}

impl<T> StmtAcceptor<T> for VarStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_varstmt(self)
    }
}

impl<T> StmtAcceptor<T> for BlockStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_blockstmt(self)
    }
}

impl<T> StmtAcceptor<T> for IfStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_ifstmt(self)
    }
}

impl<T> StmtAcceptor<T> for WhileStmt {
    fn accept(&self, visitor: &dyn StmtVisitor<T>) -> Result<T, JokerError> {
        visitor.visit_whilestmt(self)
    }
}

