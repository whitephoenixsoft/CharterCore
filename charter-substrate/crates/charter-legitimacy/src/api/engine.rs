use crate::compiler::CompiledState;
use crate::domain::{AreaGraph, SessionId};
use crate::error::{EngineError, EvaluationReport};
use crate::spec::SpecificationManifest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMode {
    NormalRuntime,
    DegradedReadOnly,
}

#[derive(Debug, Clone)]
pub struct RehydrateInput {
    pub graph: AreaGraph,
}

#[derive(Debug, Clone)]
pub struct RehydrateResult {
    pub runtime_mode: RuntimeMode,
    pub report: EvaluationReport,
    pub engine: Option<Engine>,
}

#[derive(Debug, Clone)]
pub struct Engine {
    state: CompiledState,
    runtime_mode: RuntimeMode,
}

impl Engine {
    pub fn rehydrate(input: RehydrateInput) -> Result<RehydrateResult, EngineError> {
        let compiled = CompiledState::from_graph(input.graph);

        let report = EvaluationReport::success(
            "rehydrate_engine",
            "area_graph",
            None,
        );

        let engine = Self {
            state: compiled,
            runtime_mode: RuntimeMode::NormalRuntime,
        };

        Ok(RehydrateResult {
            runtime_mode: engine.runtime_mode,
            report,
            engine: Some(engine),
        })
    }

    pub fn runtime_mode(&self) -> RuntimeMode {
        self.runtime_mode
    }

    pub fn compiled_state(&self) -> &CompiledState {
        &self.state
    }

    pub fn evaluate_session(
        &self,
        session_id: SessionId,
    ) -> Result<EvaluationReport, EngineError> {
        if self.runtime_mode == RuntimeMode::DegradedReadOnly {
            return Ok(EvaluationReport::blocked(
                "evaluate_session",
                "session",
                Some(session_id.as_str()),
                "DEGRADED_MODE_ACTIVE",
            ));
        }

        if !self.state.sessions.contains_key(&session_id) {
            return Err(EngineError::not_found("session", session_id.as_str()));
        }

        Ok(EvaluationReport::success(
            "evaluate_session",
            "session",
            Some(session_id.as_str()),
        ))
    }

    pub fn specification_manifest() -> &'static SpecificationManifest {
        crate::spec::embedded_manifest()
    }
}