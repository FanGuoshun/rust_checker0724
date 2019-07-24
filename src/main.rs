pub mod rustChecker;
pub mod transactionForChecker;
use crate::rustChecker::pythonChecker::PythonChecker;


fn main() {
    let checker: PythonChecker = unsafe { PythonChecker::new(&String::from("addition")) };
    checker.startChecker();
    println!("Hello, world!");
}
