use crate::frontend::{GeneralOsdiCall, GeneralOsdiInput};
use crate::subfuncitons::automatic_slicing::function_cfg_from_full_cfg;
use openvaf_data_structures::index_vec::IndexVec;
use openvaf_data_structures::BitSet;
use openvaf_hir::{Unknown, VariableId};
use openvaf_ir::convert::Convert;
use openvaf_ir::Type;
use openvaf_middle::cfg::{ControlFlowGraph, IntLocation, InternedLocations};
use openvaf_middle::const_fold::DiamondLattice;
use openvaf_middle::{
    COperand, COperandData, CallArg, CallType, CallTypeConversion, Derivative, InputKind, Local,
    Mir, OperandData, ParameterInput, RValue, StmntKind,
};
use openvaf_session::sourcemap::Span;
use openvaf_transformations::InvProgramDependenceGraph;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub type ModelTempUpdateCfg = ControlFlowGraph<ModelTempUpdateCallType>;

#[derive(PartialEq, Eq, Clone)]
pub enum ModelTempUpdateCallType {}

impl CallType for ModelTempUpdateCallType {
    type I = ModelTempUpdateInput;

    fn const_fold(&self, _call: &[DiamondLattice]) -> DiamondLattice {
        match *self {}
    }

    fn derivative<C: CallType>(
        &self,
        _original: Local,
        _mir: &Mir<C>,
        _arg_derivative: impl FnMut(CallArg) -> Derivative<Self::I>,
    ) -> Derivative<Self::I> {
        match *self {}
    }
}

impl Display for ModelTempUpdateCallType {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl Debug for ModelTempUpdateCallType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Convert<ModelTempUpdateCallType> for GeneralOsdiCall {
    fn convert(self) -> ModelTempUpdateCallType {
        match self {
            GeneralOsdiCall::Noise => unreachable!(),
            GeneralOsdiCall::TimeDerivative => unreachable!(),
            GeneralOsdiCall::StopTask(_, _) => unreachable!(),
            GeneralOsdiCall::NodeCollapse(_, _) => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModelTempUpdateInput {
    Parameter(ParameterInput),
    Temperature,
}

impl Display for ModelTempUpdateInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parameter(param_input) => Display::fmt(param_input, f),
            Self::Temperature => write!(f, "$temperature"),
        }
    }
}

impl InputKind for ModelTempUpdateInput {
    fn derivative<C: CallType>(&self, _unknown: Unknown, _mir: &Mir<C>) -> Derivative<Self> {
        unreachable!() // No derivatives allows in the init function since that would require values that depend uponm voltages
    }

    fn ty<C: CallType>(&self, mir: &Mir<C>) -> Type {
        match self {
            Self::Parameter(ParameterInput::Value(param)) => mir[*param].ty,
            Self::Parameter(ParameterInput::Given(_)) => Type::BOOL,
            Self::Temperature => Type::REAL,
        }
    }
}

struct GeneralToModelTempUpdate;

impl CallTypeConversion<GeneralOsdiCall, ModelTempUpdateCallType> for GeneralToModelTempUpdate {
    fn map_input(
        &mut self,
        src: <GeneralOsdiCall as CallType>::I,
    ) -> COperandData<ModelTempUpdateCallType> {
        let input = match src {
            GeneralOsdiInput::Parameter(x) => ModelTempUpdateInput::Parameter(x),
            GeneralOsdiInput::Temperature => ModelTempUpdateInput::Temperature,
            _ => unreachable!(),
        };

        OperandData::Read(input)
    }

    fn map_call_val(
        &mut self,
        _call: GeneralOsdiCall,
        _args: IndexVec<CallArg, COperand<GeneralOsdiCall>>,
        _span: Span,
    ) -> RValue<ModelTempUpdateCallType> {
        unreachable!()
    }

    fn map_call_stmnt(
        &mut self,
        _call: GeneralOsdiCall,
        _args: IndexVec<CallArg, COperand<GeneralOsdiCall>>,
        _span: Span,
    ) -> StmntKind<ModelTempUpdateCallType> {
        unreachable!()
    }
}

pub struct ModelTempUpdateFunction {
    pub cfg: ControlFlowGraph<ModelTempUpdateCallType>,
    pub written_vars: BitSet<VariableId>,
}

impl ModelTempUpdateFunction {
    pub fn new(
        mir: &Mir<GeneralOsdiCall>,
        cfg: &ControlFlowGraph<GeneralOsdiCall>,
        tainted_locations: &BitSet<IntLocation>,
        assumed_locations: &BitSet<IntLocation>,
        locations: &InternedLocations,
        pdg: &InvProgramDependenceGraph,
        all_output_stmnts: &BitSet<IntLocation>,
    ) -> (Self, BitSet<IntLocation>) {
        let (cfg, function_output_locations, written_vars) = function_cfg_from_full_cfg(
            mir,
            cfg,
            tainted_locations,
            Some(assumed_locations),
            all_output_stmnts,
            locations,
            pdg,
        );

        let cfg = cfg.map(&mut GeneralToModelTempUpdate);

        (Self { cfg, written_vars }, function_output_locations)
    }
}
