use clap::Parser;
use tableit::{tableformat::FORMATS, Table};

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_columns(input: &str) -> ColType {
    input.split(',').map(|s| s.to_string()).collect()
}

fn parse_rows(input: &str) -> RowType {
    let mut out = Vec::new();
    let splited = input.split('|');

    for s in splited {
        let temp = s.split(',');
        let v: Vec<String> = temp.map(|s| s.to_string()).collect();
        out.push(v);
    }

    out
}
type ColType = Vec<String>;
type RowType = Vec<Vec<String>>;

#[derive(Parser)]
#[clap(author="dammi-i", version=VERSION, about="a CLI utility to format data into a table")]
struct Opts {
    #[clap(
        short = 'c',
        long = "columns",
        required = false,
        parse(from_str = parse_columns)
    )]
    columns: Option<ColType>,
    #[clap(
        short = 'r',
        long = "rows",
        required = false,
        parse(from_str = parse_rows),
        help = "set table rows, it must match the format \"row...|row....\""
    )]
    rows: Option<RowType>,
    #[clap(
        short = 's',
        long = "style",
        required = false,
        help = "table style",
        default_value = "fancy1"
    )]
    style: String,
    #[clap(short = 'S', long = "styles", help = "display available styles")]
    styles: bool,
}

fn main() {
    let opt = Opts::parse();
    let mut table = Table::with_format(FORMATS.with(|m| m.borrow()[opt.style.as_str()].clone()));
    if opt.styles {
        let fmts = FORMATS.with(|m| {
            m.borrow()
                .keys()
                .map(|s| String::from(*s))
                .collect::<Vec<String>>()
        });
        println!("availables styles:");
        for (i, n) in fmts.into_iter().enumerate() {
            println!("\t{}: {}", i, n);
        }
    }
    if let Some(columns) = opt.columns {
        table.add_columns(columns);
    }
    if let Some(rows) = opt.rows {
        for row in rows {
            table.add_row(row);
        }
        println!("{}", table.render().unwrap());
    }
}
