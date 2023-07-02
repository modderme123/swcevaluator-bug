use swc_core::{
    common::comments::Comments,
    ecma::{
        ast::*,
        minifier::{
            eval::{EvalResult, Evaluator},
            marks::Marks,
        },
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

pub struct TransformVisitor<C>
where
    C: Comments,
{
    pub comments: C,
    pub evaluator: Option<Evaluator>,
}

impl<C> TransformVisitor<C>
where
    C: Comments,
{
    pub fn new(comments: C) -> Self {
        Self {
            comments,
            evaluator: Default::default(),
        }
    }
}

impl<C> VisitMut for TransformVisitor<C>
where
    C: Comments,
{
    fn visit_mut_call_expr(&mut self, expr: &mut CallExpr) {
        let x = self.evaluator.as_mut().unwrap().eval(&expr.args[0].expr);
        match x {
            Some(EvalResult::Lit(lit)) => {
                expr.args[0].expr = Box::new(Expr::Lit(lit));
            }
            _ => println!("None"),
        }
        expr.visit_mut_children_with(self);
    }
    fn visit_mut_module(&mut self, module: &mut Module) {
        self.evaluator = Some(Evaluator::new(module.clone(), Marks::new()));
        module.visit_mut_children_with(self);
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor::new(&metadata.comments)))
}
