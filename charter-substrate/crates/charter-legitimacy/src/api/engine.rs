use crate::compiler::{validate_graph, CompiledState};
use crate::domain::AreaGraph;
use crate::error::{EngineError, EvaluationReport, EvaluationOutcome};
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
    pub runtime_mode: Option<RuntimeMode>,
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
        let errors = validate_graph(&input.graph);

        if !errors.is_empty() {
            let mut report = EvaluationReport::rejected(
                "rehydrate_engine",
                "area_graph",
                input.graph.area_id.as_ref().map(|x| x.as_str()),
                errors[0].code(),
            );

            report.errors = errors.iter().map(|e| e.to_error_entry()).collect();
            report.primary_error_code = report.errors.first().map(|e| e.error_code.clone());
            report.outcome = EvaluationOutcome::Rejected;

            return Ok(RehydrateResult {
                runtime_mode: None,
                report,
                engine: None,
            });
        }

        let state = CompiledState::from_graph(input.graph);

        let report = EvaluationReport::success("rehydrate_engine", "area_graph", None);

        let engine = Self {
            state,
            runtime_mode: RuntimeMode::NormalRuntime,
        };

        Ok(RehydrateResult {
            runtime_mode: Some(engine.runtime_mode),
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

    pub fn specification_manifest() -> &'static SpecificationManifest {
        crate::spec::embedded_manifest()
    }
}