macro_rules! opcodes {
    ($($ident:ident $(($ty:ty))?),*) => {
        
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Opcode {
            $($ident$(($ty))?,)*
        }
        
        pub trait OpcodeExecutor {
            $(
            paste::paste!{
                    fn [<$ident:lower>](&mut self $(, $ty: $ty)?);
                }
            )*
        }
    };
}
opcodes!(ADD, SUB, DIV, MUL, MODU, PUSH(u128), POP);

pub trait Get<T> {
    fn get(&self) -> T;
}
