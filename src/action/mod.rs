use crate::{cli::Cmd, navigation::{Todos}};

pub(crate) mod ls;
pub(crate) mod open;



pub fn filter_todo(todos: Vec<Todos>, cmd: &Cmd)->Vec<Todos> {
    todos
        .into_iter()
        .filter_map(|mut t|{
            t.entries.retain(|entry| {
                cmd.value
                    .as_ref()
                    .map_or(true, |v| v == &entry.tag)
            });
            
            if t.entries.is_empty() {
                None
            } else {
                Some(t)
            }
        })
        .collect()
}
