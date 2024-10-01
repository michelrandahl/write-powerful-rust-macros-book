macro_rules! generate_get_value {
    ($struct_type : ident) => (
        generate_get_value!($struct_type, String);
    );
    ($struct_type : ident, $return_type : ty) => (
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    )
}
