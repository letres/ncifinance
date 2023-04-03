use ta::{
    indicators::{
        //Oscilattors
        FastStochastic, RelativeStrengthIndex, 
        MovingAverageConvergenceDivergence,        
    },
    Next,
    DataItem,
};

//To add novel features, impl Update with a custom Type for the struct

pub trait NodeCalc<T> {
    fn compute_tree(&self, DataItem) -> f64;
    fn update_calc(&self,T);
}



enum Comparison {
    GreaterThan,
    LessThanEq,
}

struct ComparisonNode {
    pub left: Box<NodeCalc>,
    pub right: Box<NodeCalc>,
    pub comparison: Comparison,
}


impl NodeCalc for ComparisonNode{
    fn compute_tree(&self, data:DataItem)-> f64 {
        match self.comparison {
            Comparison::GreaterThan => (left.compute_tree(data) > right.calc(data)) as f64;
            Comparison::LessThanEq => (left.compute_tree(data) <= right.calc(data)) as f64;
        }
    }
}

struct ConstantNode {
    pub value: f64,
}

impl NodeCalc for ConstantNode{
    fn compute_tree(&self, _data:DataItem)-> f64 {
        self.value
    }
    fn update_calc(&self,_:T){}
}   


//This should just be a wrapper (or less) around ta indicators
enum Indicator {
    SMA,
    EMA,
    RSI,
    MACD,
    // add more indicators as needed
}

//This needs to take in the value from the previous total computation
struct Lag {
    pub value: f64,
}


//Max and Min is equivilent to (N)Or


//Multiply is equivilant to to an AND on Binary inputs
struct Multiply {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

struct Average {
    pub nodes: Vec<Box<Node>>,
}


//The Nodes will Eval by passing a DataItem down the Tree
enum Node {
    IndicatorNode { indicator: Indicator, lag: Lag },
    ConstantNode(ConstantNode),
    MultiplyNode(Multiply),
    AverageNode(Average),
    ComparisonNode(ComparisonNode),
}
