use comfy_table::{Table, presets::UTF8_FULL}

mod table {
    struct Collumn {
        pub title: String,
        pub lenght: usize,
        pub cell: Vec<String>,
    }

    pub struct Table {
        collums:Vec<Collumn>
    }
    impl Table {
        pub fn init(title:String, col: Vec<String>){

        }
        pub fn iter(&self){

        }
    }

    pub struct TableIter<'a> {
        rows: Vec<&'a Collumn> 
    }

    impl<'a> Iterator for Table<'a> {
        type Item = Vec<(&'a str, usize)>;

        fn next(&mut self)->Option<Self::Item> {
            
        }
    }

}
