#[macro_export]
macro_rules! executable {
    ( $fn_name: ident,  $op: tt, $val: expr) => {
        fn $fn_name<'r>(registers: &'r mut [MaybeBorrowed<RegisterValue>], data: &[MaybeBorrowed<RegisterValue>]) -> &'r [MaybeBorrowed<'r, RegisterValue>] {
            assert_eq!(registers.len(), data.len());

            for index in 0..registers.len() {
                registers[index] = MaybeBorrowed::Owned(registers[index].get() $op $val)
            }

            return registers;
        }
    };

    ( $fn_name: ident, $op: tt) => {
        fn $fn_name<'r>(registers: &'r mut [MaybeBorrowed<RegisterValue>], data: &[MaybeBorrowed<RegisterValue>]) -> &'r [MaybeBorrowed<'r, RegisterValue>] {
            assert_eq!(registers.len(), data.len());

            for index in 0..registers.len() {
                registers[index] = MaybeBorrowed::Owned(registers[index].get() $op data[index].get())
            }

            return registers;
        }
    };

}

#[macro_export]
macro_rules! executables {
    ($($fn_tail: path),*) => {
         &[
            $(
                AnyExecutable(stringify!($fn_tail), $fn_tail),
            )*
        ] as &'static [AnyExecutable]
    };
}
