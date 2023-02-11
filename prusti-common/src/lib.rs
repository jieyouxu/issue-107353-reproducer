pub mod vir {
    pub mod polymorphic_vir {
        pub struct DomainFunc;
        pub struct Domain;
        #[derive(Clone)]
        pub struct LocalVar;
        pub struct Type;
    
        #[derive(Clone)]
        pub struct Expr;
        pub struct Trigger;
    
        impl Expr {
            pub fn implies(left: Expr, right: Expr) -> Self { todo!() }
            pub fn forall(_: Vec<LocalVar>, _: Vec<()>, _: Expr) -> Self { todo!() }
            pub fn eq_cmp(left: Expr, right: Expr) -> Self { todo!() }
            pub fn and(left: Expr, right: Expr) -> Self { todo!() }
        }
    
        impl Trigger {
            pub fn new(_: Vec<Expr>) { todo!() }
        }
    }
    
    pub mod macros {
        pub mod polymorphic {
            #[macro_export]
            macro_rules! vir_expr {
                ($lhs: tt == $rhs: tt) => {
                    $crate::vir::polymorphic_vir::Expr::eq_cmp(vir_expr!($lhs), vir_expr!($rhs))
                };
                
                ($head: tt && $tail: tt) => {
                    $crate::vir::polymorphic_vir::Expr::and(vir_expr!($head), vir_expr!($tail))
                };
    
                ($antecedent: tt ==> $consequent: tt) => {
                    $crate::vir::polymorphic_vir::Expr::implies(vir_expr!($antecedent), vir_expr!($consequent))
                };
    
                (forall $($name: ident : $type: tt),+ :: $({ $($triggers: tt),+ })+ :: $body: tt) => {
                    $crate::vir::polymorphic_vir::Expr::forall(
                        vec![$($crate::vir_local!($name: $type)),+],
                        vec![
                            $($crate::vir::polymorphic_vir::Trigger::new(vec![
                                $(vir_expr!($triggers)),+
                            ])),*
                        ],
                        vir_expr!($body),
                    )
                };
    
                (forall $([$vars: expr]),+ :: $({ $($triggers: tt),+ })+ :: $body: tt) => {
                    $crate::vir::polymorphic_vir::Expr::forall(
                        vec![$($vars.clone()),+],
                        vec![
                            $($crate::vir::polymorphic_vir::Trigger::new(vec![
                                $(vir_expr!($triggers)),+
                            ])),*
                        ],
                        vir_expr!($body),
                    )
                };
    
                ([ $e: expr ]) => { $e.clone() };
    
                (( $($tokens: tt)+ )) => { vir_expr!($($tokens)+) }
            }
        }
    }    
}
