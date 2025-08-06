# Goal
simple lazy parser that finds todos 
A todo has this pattern ( todo ("var") {"desc"} )


# Functionnality
patterns are only valid between // -- \n and /**/

therefore // todo (var ) \n  
pub fn add(x,y) {}

//} 
== todo (var) { pub fn add (x,y) }
