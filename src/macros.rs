#[macro_export]
macro_rules! print_full_width(
    ($ident:ident, $pos:expr) => {{
        |printer| { printer.print((0, $pos), 
            &format!("{}{}", 
                $ident,
                String::from_utf8(
                    vec![b' '; 
                    printer.size.x - $ident.len()]).unwrap())); 
        }
    }}
);
