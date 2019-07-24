mod transactionForChecker;

pub struct TransactionForChecker{
    contractID: String,
    inputs: Vec<String>,
    referenceInputs: Vec<String>,
    parameters: Vec<String>,
    returns: Vec<String>,
    outputs: Vec<String>,
    dependencies: Vec<String>,
    methodID: Vec<String>,

}

impl TransactionForChecker{
    pub fn new(&mut self,
               &contractID: String,
               &inputs: Vec<String>,
               &referenceInputs: Vec<String>,
               &parameters: Vec<String>,
               &returns: Vec<String>,
               &outputs: Vec<String>,
               &dependencies: Vec<String>,
               &methodID: Vec<String>) -> TransactionForChecker
    {
        self.contractID = contractID;
        self.inputs = inputs;
        self.referenceInputs = referenceInputs;
        self.parameters = parameters;
        self.returns = returns;
        self.outputs = outputs;
        self.dependencies = dependencies;
        self.methodID = methodID;
    }
    pub fn getContractID(&self) -> String{
        self::contractID
    }
    pub fn getMethodID(&self) -> String{
        self::methodID
    }
    pub fn getInputs(&self) -> Vec<String>{
        self::inputs
    }
    pub fn getOutputs(&self) -> Vec<String>{
        self::outputs
    }
}