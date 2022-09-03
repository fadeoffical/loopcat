#[macro_export]
macro_rules! lööp {
    ( $($statement:stmt;)* ) => { loop { $( $statement )* } };
}

#[cfg(test)]
mod tests {
    use crate::lööp;

    #[test]
    fn test_lööp() {
        lööp! {
            break;
        };
    }

    #[test]
    fn test_lööp_print() {
        lööp! {
            print!("lollmao");
            break;
        };
    }
}
