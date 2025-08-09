use std::path::PathBuf;

use todo_searcher::todo_list::{self};

//Todo (comment) {check this comment
#[test]
fn parse_comment(){
  let file = todo_list::create_list(PathBuf::from("tests/todo_test.rs"));
    
    let mut list = match file {
        Ok(file) => file,
        Err(e) =>panic!("{}",e)
    };
//}
    dbg!(&list);
    let todo = list.list.pop().unwrap();
    assert_eq!("end of file :)",todo.desc);

    let todo = list.list.pop().unwrap();
    assert_eq!(" pls modify this
      the river of something is fishing ttt",todo.desc);

    let todo = list.list.pop().unwrap();
    assert_eq!("add some asserts",todo.desc);
    
    let todo = list.list.pop().unwrap();
    assert_eq!("check this comment
#[test]
fn parse_comment(){
  let file = todo_list::create_list(PathBuf::from(\"tests/todo_test.rs\"));
    
    let mut list = match file {
        Ok(file) => file,
        Err(e) =>panic!(\"{}\",e)
    };
//",todo.desc); 

    //todo(test_2) {add some asserts}
    /*todo ( todo (test_3) { pls modify this
      the river of something is fishing ttt}*/
}
#[test]
fn parse_comment_filtered(){
   let file = todo_list::create_filtered_list_lazy (
       PathBuf::from("tests/todo_test.rs"),
       |list| list.traits.contains_key("test_2"));
    
    let mut list = match file {
        Ok(file) => file,
        Err(e) =>panic!("{}",e)
    };
    let todo = list.list.pop().unwrap();

    assert_eq!("add some asserts", todo.desc);
}

// todo (end_of_file) {end of file :)}


