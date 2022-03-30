mod builder;
mod live_derivatives;
mod unkowns;

use ahash::AHashMap;
pub use builder::build_derivatives;
pub use live_derivatives::LiveDerivatives;
use mir::{ControlFlowGraph, FuncRef, Function, Value};
use typed_indexmap::TiMap;
pub use unkowns::{FirstOrderUnkown, FirstOrderUnkownInfo, Unkowns};

pub fn auto_diff(
    func: &mut Function,
    cfg: &ControlFlowGraph,
    unkowns: &TiMap<FirstOrderUnkown, FuncRef, FirstOrderUnkownInfo>,
    extra_derivatives: impl IntoIterator<Item = (Value, FuncRef)>,
) -> AHashMap<(Value, FirstOrderUnkown), Value> {
    let mut unkowns = Unkowns::new(unkowns);
    let live_derivative = LiveDerivatives::build(func, &mut unkowns, extra_derivatives);
    // for inst in live_derivative.derivatives.rows() {
    //     let unkowns_ = live_derivative.derivatives.row(inst).unwrap();
    //     if unkowns_.is_empty() {
    //         continue;
    //     }
    //     println!("{}:", func.dfg.display_inst(inst));
    //     for unkown in unkowns_.iter() {
    //         println!("\t{:?}", unkown)
    //     }
    // }
    build_derivatives(func, cfg, &unkowns, &live_derivative)
}

// pub fn find_derivatives_of()
