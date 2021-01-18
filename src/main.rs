use encoding_rs;
use regex::Regex;
use std::fs;
use std::io::{Write};
use std::path::Path;

enum Terminator {
    Tab,
    Return,
    // Nothing,
}


#[derive(Debug)]
struct SymfoTable {
    table_name: String,
    // columns: Vec<SymfoColumn>,
}

impl SymfoTable {
    fn new(table: &str) -> Self {
        SymfoTable {
            table_name: String::from(table),
            // columns: Vec::new(),
        }
    }
    // }
    /// .txt
    fn dot_txt(&self) -> String {
        format!("{}.txt", self.table_name)
    }
}

// #[derive(Debug)]
// struct SymfoColumn {
//     column_name: String,
//     not_null_constraint: bool,
//     data_length: i32,
//     data_type: String,
// }

// impl SymfoColumn {
//     fn new(column: &str) -> Self {
//         SymfoColumn {
//             column_name: String::from(column),
//             not_null_constraint: false,
//             data_length: 0,
//             data_type: String::new(),
//         }
//     }

//     fn print(&self) {
//         println!("Column Name: {}", self.column_name);
//         println!("Data Type: {}", self.data_type);
//     }
// }

// struct SymfoState {
//     table_name: String,
//     active: bool,
//     state: StateBox,
// }

// enum StateBox {
//     Never,
//     ColumnName,
//     NotNullConstraint,
//     DataType,
//     Finish,
// }

fn main() {
    let text = read_file("ncc_egcdb.txt");

    // 改行で区切り分割して Vecに入れる。
    let lines = text.split("\n");
    let mut tables = Vec::new();

    // print!("{}", text);
    let mut cnt = 0;

    // ファイルから切り出すパターン
    let re_table_name =
        Regex::new(r".*Table name \.\.\.\.\.\. (?P<table_name>[_0-9A-Z]*)").unwrap();
    let re_column_name =
        Regex::new(r"Column name         \.\.\.\.\.\. (?P<column_name>[_0-9A-Z]*)").unwrap();
    let re_data_type = Regex::new(r"Data type           \.\.\.\.\.\. (?P<data_type>.*)").unwrap();

    // let mut state = SymfoState {
    //     table_name: String::new(),
    //     active: false,
    //     state: StateBox::Never,
    // };

    // ファイルの書き出しテスト
    // lei pathname = format!("out/{}", "out_text.txt");
    // let out_file_path = Path::new(&pathname);
    // // let mut out_file = fs::File::create(out_file_path).unwrap();
    // let mut out_file = fs::OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .append(true)
    //     .open(out_file_path)
    //     .unwrap();
    // writeln!(out_file, "hoge");

    // let mut out_file_name = String::new();

    for line in lines {
        cnt += 1;

        let trimed = line.trim();

        // テーブル名が検出された場合
        if re_table_name.is_match(trimed) {
            let table_name = re_table_name.replace(trimed, "$table_name").into_owned();
            tables.push(SymfoTable::new(&table_name));

            // out_file_name = table_name;
            // 表示
            // println!("{}", table_name);
            // write_out_file(out_file_name, content)
        }

        // カラム名が検出された場合
        if re_column_name.is_match(trimed) {
            let column_name = re_column_name.replace(trimed, "$column_name").into_owned();
            let out_file_name = tables.last().unwrap().dot_txt();
            // 表示
            // print!("\t{}", column_name);
            write_out_file(out_file_name, column_name, Terminator::Tab);
        }

        if re_data_type.is_match(trimed) {
            let data_type = re_data_type.replace(trimed, "$data_type").into_owned();
            let out_data_type = data_type
                .replace("CHARACTER VARYING", "VARCHAR")
                .replace("CHARACTER", "CHAR")
                .replace("(", "\t")
                .replace(")", "");
            let out_file_name = tables.last().unwrap().dot_txt();
            // 表示
            // print!("\t{}", column_name);
            write_out_file(out_file_name, out_data_type, Terminator::Return);
            // 表示
            // println!("\t{}", out_data_type);
        }
        if cnt > 1000 {
            // break;
        }

        // let caps = re_table_name.captures(line).unwrap();
        // if let s = caps.get(1).unwrap_or("default");
        // }
    }
    // Ok(())
    // for t in tables {
    //     t.print();
    // }
}

fn write_out_file(file_base_name: String, content: String, terminator: Terminator) {
    let pathname = format!("out/{}", file_base_name);
    let out_file_path = Path::new(&pathname);
    // let mut out_file = fs::File::create(out_file_path).unwrap();
    let mut out_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(out_file_path)
        .unwrap();

    let t = match terminator {
        Terminator::Tab => format!("{}\t", content),
        Terminator::Return => format!("{}\n", content),
        // Terminator::Nothing => content,
    };

    write!(out_file, "{}", t).unwrap();
}

///  ファイルを読んで文字列を返すよ。
/// &str -> String
fn read_file(file_path: &str) -> String {
    // 一気に全部読み込む。
    let s = fs::read(file_path).unwrap();

    // SHIFT_JISです。
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
    let text = res.into_owned();
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_001() {
        assert_eq!(read_file("test_text.txt"), String::from("aaa"));
    }
    #[test]
    fn test_read_file_002() {
        assert_ne!(
            read_file("test_text.txt"),
            String::from("aab"),
            "読み込みはできるが文字列が違う。"
        );
    }
}
