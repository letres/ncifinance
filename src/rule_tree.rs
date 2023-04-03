use ta::DataItem
use ta::{
    indicators::{FastStochastic, RelativeStrengthIndex},
    Next,
};


pub trait NodeCalc {
    fn calc(&self, DataItem) -> f64;
}

enum Indicator {
    MovingAverage,
    RSI,
    MACD,
    // add more indicators as needed
}



enum Comparison {
    GreaterThan,
    LessThan,
    EqualTo,
    // add more comparisons as needed
}

enum BooleanOp {
    And,
    Or,
    // add more boolean operators as needed
}

struct Lag {
    pub value: f64,
}

struct Constant {
    pub value: f64,
}

struct Multiply {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

struct Average {
    pub nodes: Vec<Box<Node>>,
}

struct ComparisonNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub comparison: Comparison,
}

struct BooleanOpNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: BooleanOp,
}

enum Node {
    IndicatorNode { indicator: Indicator, lag: Lag },
    ConstantNode(Constant),
    MultiplyNode(Multiply),
    AverageNode(Average),
    ComparisonNode(ComparisonNode),
    BooleanOpNode(BooleanOpNode),
}
