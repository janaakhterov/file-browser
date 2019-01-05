#[macro_export]
macro_rules! print_full_width(
    ($ident:ident, $pos:expr) => {{
        |printer| {
            printer.print((0, $pos), $ident);
            printer.print_hline(($ident.len(), $pos), printer.size.x - $ident.len(), &" ");
        }
    }}
);
