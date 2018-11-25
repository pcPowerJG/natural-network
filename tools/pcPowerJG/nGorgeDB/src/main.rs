
extern crate nGorgeDB;
use nGorgeDB::Language;

fn main() {
    let mut o = Language::on_create();

	o.get_("        
        \n
        \n add table { 0, 1, 2  }
        \n add table { 1, 2, 3  }
        \n add table { 2, 1, 10 }
        \n add table_row_chanse { 0, 1, 10, 15 }
        \n add table_row_chanse { 1, Name, 10, 15 }
        \n add table_row_chanse { 2, 1, 10, 15 }
        \n add table_row_chanse { 3, Name, 10, 3 }
        \n search _ in table param 1=1
        \n search _ in table_row_chanse param 1=Name&3=3
        \n remove _ in table param 1=1
        \n backup \n
        \n".to_string());
}
