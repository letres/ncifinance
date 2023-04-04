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
    fn update_calc(&mut self,T);
}


//Comparison Node
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
            Comparison::GreaterThan => (left.compute_tree(data) > right.compute_tree(data)) as f64;
            Comparison::LessThanEq => (left.compute_tree(data) <= right.compute_tree(data)) as f64;
        }
    }
    fn update_calc(&mut self, data:T){
        left.compute_tree(data);
        right.compute_tree(data);
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


//This needs to take in the value from the previous total computationThis should work.
struct Lag {
    pub value: f64,
}

struct LagNode{
    pub value: f64,
}

impl NodeCalc for LagNode{
    fn compute_tree(&self, _data:DataItem)-> f64 {
        self.value
    }
    fn update_calc(&mut self,data:Lag){
        self.value = data.value;
    }
}   


//This should just be a wrapper (or less) around ta indicators
//Should be done first, This combined with Constant and Compare should 
//Be enough to imitate the example engine5
struct IndicatorNode<T:ta::Next<f64>>(T)

impl NodeCalc for IndicatorNode{
    fn compute_tree(&self, data:DataItem)-> f64 {
        self.next(data)
    }
    fn update_calc(&mut self,_:T){}
}   
//TODO:Max and Min is equivilent to (N)Or


//TODO:Multiply is equivilant to to an AND on Binary inputs 
struct Multiply {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

//TODO: This will do some nice things
struct Average {
    pub nodes: Vec<Box<Node>>,
}

