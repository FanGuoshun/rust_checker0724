
pub mod pythonChecker {

    extern crate serde_derive;
    extern crate serde_json;
    extern crate rustc_serialize;
    extern crate reqwest;
    use rustc_serialize::json;
    use std::process::{Command, Child};
    use crate::transactionForChecker::transactionForChecker::TransactionForChecker;
    use std::env;
    use reqwest::Client;
    use reqwest::Error;
    use self::reqwest::Response;


    static mut LATESTPORT: i32 = 13000;
    static mut CHECKER_HOST: String = String::from("127.0.0.1");

    pub struct PythonChecker {
        pythonScriptPath: String,
        contractID: String,
        methodID: String,
        port: i32,
        pythonExecutable: String,
    }

    impl PythonChecker {
        pub unsafe fn new(contractID: &String) -> PythonChecker {
            if LATESTPORT == 65535 {
                LATESTPORT = 13000;
            }
            LATESTPORT += 1;
            PythonChecker {
                pythonScriptPath: String::from("contracts/") + &contractID.to_string()+ &".py".to_string(),
                contractID: contractID.to_string(),
                methodID: String::from(""),
                port: LATESTPORT,
                pythonExecutable: String::from("/home/aaron/project/python2_env/env/bin/python"),
            }
        }
        pub fn startChecker(&self) -> Child {
            println!("Starting checker@ {}", self.getURL(&"".to_string()));
            let checkerProcess = Command::new(self.pythonExecutable)
                .args(&[self.pythonScriptPath,
                    "checker".to_string(), "--port".to_string()])
                .arg(self.port.to_string())
                .spawn()
                .expect("Checker failed to start!");
            checkerProcess
        }
        pub fn getContractID(&self) -> String {
            self.contractID
        }
        pub fn getMethodID(&self) -> String {
            self.methodID
        }
        pub unsafe fn getURL(&self, methodID: &String) -> String {
            String::from("http://") + &CHECKER_HOST + &":".to_string() + &self.port.to_string()
                + &"/".to_string() + &self.contractID.to_string() + &"/".to_string() + &methodID.to_string()
        }
        pub unsafe fn check(&self, transaction: &TransactionForChecker) -> Result<Response, Error> {

            let postData = json::encode(&transaction).unwrap();
            let url = self.getURL(&transaction.getMethodID());
            let result = Client::new().post(&url).json(&postData).send();
            result
        }
    }
}
