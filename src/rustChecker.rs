pub mod transactionForChecker;

extern crate reqwest;
extern crate serde_derive;
extern crate serde_json;
extern crate rustc_serialize;
extern crate reqwest;
// 引入rustc_serialize模块
use rustc_serialize::json;
use std::process::Command;
use rustc_serialize::json;
use transactionForChecker::TransactionForChecker;
use std::env;
use reqwest::Client;
use reqwest::Error;

static mut LATESTPORT: i32 = 13000;
static mut CHECKER_HOST: String = String::from("127.0.0.1");

pub struct PythonChecker {
    pythonScriptPath: String,
    contractID: String,
    methodID: String,
    checkerProcess: Command,
    port: i32,
    pythonExecutable: String,

}

impl PythonChecker {


    pub fn getPythonExecutable() -> String{
        String::from("/home/aaron/project/python2_env/env/bin/python")
    }

    pub unsafe fn new(&mut self, &contractID: String) -> PythonChecker {
        self.contractID = contractID;
        self.pythonScriptPath = "contracts/" + contractID + ".py";
        self.pythonExecutable = getPythonExecutable();
        if (LATESTPORT == 65535) {
            LATESTPORT = 13000;
        }
        LATESTPORT += 1;
        self.port = LATESTPORT;
    }
    pub fn startChecker(&self) {
        self::checkerProcess = Command::new(self.pythonExecutable)
            .args(&[self.pythonScriptPath,
                "checker", "--port"])
            .arg(port)
            .spawn()
            .expect("Checker failed to start!");

    }
    pub fn getContractID(&self) -> String{
        self::contractID
    }
    pub fn getMethodID(&self) -> String{
        self::methodID
    }
    pub unsafe fn getURL(&self, &methodID: String) -> String{
        "http://"+&CHECKER_HOST+":"+self::port+"/"+self::contractID+"/"+methodID
    }
    pub fn check(&self, &transaction: TransactionForChecker) -> String{
        println!("\nChecker URL:");
        println!("\t" + self.getURL(transaction.getMethodID()));

        let postData = json::encode(&transaction).unwrap();
        let url = self.getURL(transaction.getMethodID());
        Client::new().post(url).json(postData).send()?
    }

}

fn main() {

}