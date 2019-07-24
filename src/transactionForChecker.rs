pub mod transactionForChecker {
    pub struct TransactionForChecker {
        contractID: String,
        inputs: Vec<String>,
        referenceInputs: Vec<String>,
        parameters: Vec<String>,
        returns: Vec<String>,
        outputs: Vec<String>,
        dependencies: Vec<String>,
        methodID: String,

    }

    impl TransactionForChecker {
        pub fn new(contractID: String,
                   inputs: Vec<String>,
                   referenceInputs: Vec<String>,
                   parameters: Vec<String>,
                   returns: Vec<String>,
                   outputs: Vec<String>,
                   dependencies: Vec<String>,
                   methodID: String) -> TransactionForChecker
        {
            TransactionForChecker {
                contractID,
                inputs,
                referenceInputs,
                parameters,
                returns,
                outputs,
                dependencies,
                methodID,
            }
        }
        pub fn getContractID(&self) -> String {
            self.contractID
        }
        pub fn getMethodID(&self) -> String {
            self.methodID
        }
        pub fn getInputs(&self) -> Vec<String> {
            self.inputs
        }
        pub fn getOutputs(&self) -> Vec<String> {
            self.outputs
        }
    }
}
