use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ColumnFormat {
    pub top_right_corner: &'static str,
    pub bot_right_corner: &'static str,
    pub top_left_corner: &'static str,
    pub bot_left_corner: &'static str,
    pub separator: &'static str,
    pub top_separator: &'static str,
    pub mid_separator: &'static str,
    pub bot_separator: &'static str,
    pub line: &'static str,
}

#[derive(Clone, Debug)]
pub struct RowFormat {
    pub right_corner: &'static str,
    pub left_corner: &'static str,
    pub bot_right_corner: &'static str,
    pub bot_left_corner: &'static str,
    pub separator: &'static str,
    pub bot_separator: &'static str,
    pub line: &'static str,
}

#[derive(Clone, Debug)]
pub struct TableFormat {
    pub column: ColumnFormat,
    pub row: RowFormat,
}

thread_local! {
pub static FORMATS: RefCell<HashMap<&'static str, TableFormat>> = RefCell::new((|| {
    let mut map = HashMap::<&'static str, TableFormat>::new();
    map.insert(
        "rst",
        TableFormat {
            column: ColumnFormat {
                top_right_corner: "+",
                bot_right_corner: "+",
                top_left_corner: "+",
                bot_left_corner: "+",
                separator: "|",
                top_separator: "+",
                mid_separator: "+",
                bot_separator: "+",
                line: "-",
            },
            row: RowFormat {
                right_corner: "+",
                left_corner: "+",
                bot_right_corner: "+",
                bot_left_corner: "+",
                separator: "|",
                bot_separator: "+",
                line: "-",
            },
        });
        /*
        static column_start: &str = "═";
        static column_top_rcorner: &str = "╔";
        static column_bot_rcorner: &str = "╟";

        static column_top_lcorner: &str = "╗";
        static column_bot_lcorner: &str = "╣";
        static column_separator: &str = "║";
        static column_top_separator: &str = "╦";
        static column_mid_separator: &str = "╬";
        static column_bot_separator: &str = "╩";
        static column_end: &str = "═";

        static row_separator: &str = "║";
        static row_end: &str = "═";

        static end_rcorner: &str = "╚";
        static end_lcorner: &str = "╝";
        */
        map.insert(
        "fancy1",
        TableFormat {
            column: ColumnFormat {
                top_right_corner: "╔",
                bot_right_corner: "╟",
                top_left_corner: "╗",
                bot_left_corner: "╣",
                separator: "║",
                top_separator: "╦",
                mid_separator: "╬",
                bot_separator: "╩",
                line: "═",
            },
            row: RowFormat {
                right_corner: "╟",
                left_corner: "╣",
                bot_right_corner: "╚",
                bot_left_corner:"╝",
                separator: "║",
                bot_separator: "╩",
                line: "═",
            }
        });
        /* ━, ┌, └, ├
        */
        map.insert(
        "light",
        TableFormat {
            column: ColumnFormat {
                top_right_corner: "┌",
                bot_right_corner: "├",
                top_left_corner: "┐",
                bot_left_corner: "┤",
                separator: "│",
                top_separator: "┬",
                mid_separator: "┼",
                bot_separator: "┴",
                line: "─",
            },
            row: RowFormat {
                right_corner: "├",
                left_corner: "┤",
                bot_right_corner: "└",
                bot_left_corner:"┘",
                separator: "│",
                bot_separator: "┴",
                line: "─",
            }
        });
        map.insert(
        "single",
        TableFormat {
            column: ColumnFormat {
                top_right_corner: " ",
                bot_right_corner: " ",
                top_left_corner: " ",
                bot_left_corner: " ",
                separator: " ",
                top_separator: " ",
                mid_separator: " ",
                bot_separator: " ",
                line: "-",
            },
            row: RowFormat {
                right_corner: " ",
                left_corner: " ",
                bot_right_corner: " ",
                bot_left_corner:" ",
                separator: " ",
                bot_separator: " ",
                line: " ",

            }
        });
    map

})()
);
}
