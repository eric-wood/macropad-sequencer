#[macro_export]
macro_rules! declare_peripherals {
    (
        struct $struct_name:ident {
            $(
                $field_name:ident => $peripheral:ident,
            )*
        }
    ) => {
        struct $struct_name {
            $(
                pub $field_name: Peri<'static, embassy_rp::peripherals::$peripheral>,
            )*
        }

        impl $struct_name {
            pub fn new(p: embassy_rp::Peripherals) -> Self {
                Self {
                    $(
                        $field_name: p.$peripheral,
                    )*
                }
            }
        }
    };
}
