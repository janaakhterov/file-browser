#[macro_export]
macro_rules! print_full_width(
    ($ident:ident, $pos:expr) => {{
        |printer| {
            printer.print((0, $pos), $ident);
            printer.print_hline(($ident.len(), $pos), printer.size.x - $ident.len(), &" ");
        }
    }}
);

#[macro_export]
macro_rules! print_full_width_with_selection(
    ($printer:ident, $element:ident, $focus:ident, $color:ident, $name:ident, $pos:ident) => {{
        if $element == $focus {
            $printer.with_color(
                $color.highlight,
                print_full_width!($name, $pos));
        } else {
            $printer.with_color(
                $color.regular,
                print_full_width!($name, $pos));
        }
    }}
);
