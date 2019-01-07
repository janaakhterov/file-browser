#[macro_export]
macro_rules! print_full_width(
    ($name:ident, $size:ident, $pos:expr) => {{
        |printer| {
            printer.print((0, $pos), &$name);
            printer.print_hline(($name.len(), $pos), printer.size.x - $name.len() - $size.len(), &" ");
            printer.print((printer.size.x - $size.len(), $pos), &$size);
        }
    }}
);

#[macro_export]
macro_rules! print_full_width_with_selection(
    ($printer:ident, $element:ident, $focus:ident, $name:ident, $size:ident, $color:ident, $pos:ident) => {{
        if $element == $focus {
            $printer.with_color(
                $color.highlight,
                print_full_width!($name, $size, $pos));
        } else {
            $printer.with_color(
                $color.regular,
                print_full_width!($name, $size, $pos));
        }
    }}
);
