#[macro_export]
macro_rules! print_full_width(
    ($name:ident, $size:ident, $pos:expr) => {{
        |printer| {
            if $name.len() < printer.size.x {
                printer.print((0, $pos), &$name);
                let size = $size.read();
                if $name.len() + size.len() < printer.size.x {
                    printer.print_hline(($name.len(), $pos), printer.size.x - $name.len() - size.len(), &" ");
                    printer.print((printer.size.x - size.len(), $pos), &size);
                } else {
                    printer.print_hline(($name.len(), $pos), printer.size.x - $name.len(), &" ");
                }
            } else {
                printer.print((0, $pos), &$name[0..printer.size.x - 1]);
                printer.print((printer.size.x - 1, $pos), &"~");
            }

        }
    }}
);

#[macro_export]
macro_rules! print_full_width_with_selection(
    ($printer:ident, $element:ident, $focus:ident, $enabled:ident, $name:ident, $size:ident, $color:ident, $pos:ident) => {{
        if $element == $focus  && $enabled {
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

#[macro_export]
macro_rules! print_empty(
    ($printer:ident, $color:ident) => {{
        $printer.with_color(
            $color,
            |printer| {
                printer.print((0, 0), "empty");
        });
    }}
);
