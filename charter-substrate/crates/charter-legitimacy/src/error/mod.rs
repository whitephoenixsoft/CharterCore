use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaluationOutcome {
    Success,
    Rejected,
    Blocked,
    NoOp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Temporary,
    Permanent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockScope {
    Session,
    Candidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReportPhase {
    StructuralValidation,
    ReceiptValidation,
    GovernanceValidation,
    SessionStateValidation,
    FreezeBoundaryValidation,
    ParticipantValidation,
    ResolutionLifecycleValidation,
    CandidateValidation,
    VoteValidation,
    DecisionConstraintValidation,
    BlockingEvaluation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorCode {
    InvalidUuid,
    DuplicateId,
    MissingReference,
    OrphanReferenceDetected,
    InvalidEnumValue,
    InvalidStateCombination,
    StructureCycleDetected,
    MultiAreaGraphDetected,
    CrossAreaStructureEdgeProhibited,
    GovernanceSlotEmpty,
    GovernanceSlotMultiplicity,
    ParticipantIdReuseDetected,

    ReceiptMissing,
    ReceiptHashMismatch,
    ReceiptOrphanDetected,
    SnapshotParticipantMismatch,

    DegradedModeActive,

    SessionTerminalImmutable,
    SessionNotActive,
    SessionBlockedTemporary,
    SessionBlockedPermanent,

    CandidateSetFrozen,
    ParticipantSetFrozen,
    ConstraintMutationForbidden,

    ParticipantNotFound,
    CannotRemoveLastParticipant,
    DuplicateParticipantDisplayName,
    ParticipantAlreadyRemoved,
    InvalidParticipantEpoch,
    NoParticipantsPresent,

    AuthorityContextMismatch,
    ScopeContextMismatch,

    AcceptanceConditionsNotMet,
    AreaBlockedByPermanentSession,
    AuthorityRuleViolation,
    ConstraintViolation,
    StructuralConflict,
    ResolutionAlreadySuperseded,
    NoEligibleCandidates,

    InvalidResolutionStateTransition,
    RetiredStateViolation,
    OnHoldStateViolation,
    SnapshotIncomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorEntry {
    pub phase: ReportPhase,
    pub error_code: ErrorCode,
    pub related_objects: Vec<String>,
    pub block_type: Option<BlockType>,
    pub block_scope: Option<BlockScope>,
}

impl ErrorEntry {
    pub fn new(
        phase: ReportPhase,
        error_code: ErrorCode,
        mut related_objects: Vec<String>,
    ) -> Self {
        related_objects.sort();

        Self {
            phase,
            error_code,
            related_objects,
            block_type: None,
            block_scope: None,
        }
    }

    pub fn blocking(
        phase: ReportPhase,
        error_code: ErrorCode,
        mut related_objects: Vec<String>,
        block_type: BlockType,
        block_scope: BlockScope,
    ) -> Self {
        related_objects.sort();

        Self {
            phase,
            error_code,
            related_objects,
            block_type: Some(block_type),
            block_scope: Some(block_scope),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationReport {
    pub evaluation_id: Option<String>,
    pub command_type: String,
    pub target_object_type: String,
    pub target_object_id: Option<String>,
    pub outcome: EvaluationOutcome,
    pub errors: Vec<ErrorEntry>,
    pub primary_error_code: Option<ErrorCode>,
    pub diagnostics: Option<String>,
    pub occurred_at: Option<String>,
    pub schema_version: u32,
}

impl EvaluationReport {
    pub fn success(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
    ) -> Self {
        EvaluationReportBuilder::new(command_type, target_object_type, target_object_id)
            .finish(true)
    }

    pub fn no_op(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
    ) -> Self {
        EvaluationReportBuilder::new(command_type, target_object_type, target_object_id)
            .finish(false)
    }

    pub fn rejected(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
        error_code: ErrorCode,
    ) -> Self {
        EvaluationReportBuilder::new(command_type, target_object_type, target_object_id)
            .error(ErrorEntry::new(
                ReportPhase::StructuralValidation,
                error_code,
                Vec::new(),
            ))
            .finish(false)
    }

    pub fn blocked(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
        error_code: ErrorCode,
    ) -> Self {
        EvaluationReportBuilder::new(command_type, target_object_type, target_object_id)
            .error(ErrorEntry::blocking(
                ReportPhase::BlockingEvaluation,
                error_code,
                Vec::new(),
                BlockType::Temporary,
                BlockScope::Session,
            ))
            .finish(false)
    }
}

#[derive(Debug, Clone)]
pub struct EvaluationReportBuilder {
    command_type: String,
    target_object_type: String,
    target_object_id: Option<String>,
    errors: Vec<ErrorEntry>,
}

impl EvaluationReportBuilder {
    pub fn new(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
    ) -> Self {
        Self {
            command_type: command_type.into(),
            target_object_type: target_object_type.into(),
            target_object_id: target_object_id.map(str::to_owned),
            errors: Vec::new(),
        }
    }

    pub fn error(mut self, error: ErrorEntry) -> Self {
        self.errors.push(error);
        self
    }

    pub fn push_error(&mut self, error: ErrorEntry) {
        self.errors.push(error);
    }

    pub fn finish(mut self, mutation_occurred: bool) -> EvaluationReport {
        self.errors.sort_by(|a, b| {
            match a.phase.cmp(&b.phase) {
                std::cmp::Ordering::Equal => match a.error_code.cmp(&b.error_code) {
                    std::cmp::Ordering::Equal => a.related_objects.cmp(&b.related_objects),
                    other => other,
                },
                other => other,
            }
        });

        self.errors.dedup_by(|a, b| {
            a.phase == b.phase
                && a.error_code == b.error_code
                && a.related_objects == b.related_objects
                && a.block_type == b.block_type
                && a.block_scope == b.block_scope
        });

        let outcome = derive_outcome(&self.errors, mutation_occurred);
        let primary_error_code = self.errors.first().map(|e| e.error_code);

        EvaluationReport {
            evaluation_id: None,
            command_type: self.command_type,
            target_object_type: self.target_object_type,
            target_object_id: self.target_object_id,
            outcome,
            errors: self.errors,
            primary_error_code,
            diagnostics: None,
            occurred_at: None,
            schema_version: 1,
        }
    }
}

fn derive_outcome(errors: &[ErrorEntry], mutation_occurred: bool) -> EvaluationOutcome {
    if errors.iter().any(|e| e.block_type.is_none()) {
        EvaluationOutcome::Rejected
    } else if errors.iter().any(|e| e.block_type.is_some()) {
        EvaluationOutcome::Blocked
    } else if mutation_occurred {
        EvaluationOutcome::Success
    } else {
        EvaluationOutcome::NoOp
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityError {
    MultiAreaGraph {
        expected_area_id: String,
        found_area_id: String,
        object_type: String,
        object_id: String,
    },
    DuplicateId {
        object_type: String,
        object_id: String,
    },
    MissingReference {
        object_type: String,
        object_id: String,
        field_name: String,
        missing_object_type: String,
        missing_object_id: String,
    },
    InvalidReceiptResolutionBinding {
        receipt_id: String,
        detail: String,
    },
    InvalidTerminalReceiptReference {
        session_id: String,
        receipt_id: String,
    },
}

impl IntegrityError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::MultiAreaGraph { .. } => ErrorCode::MultiAreaGraphDetected,
            Self::DuplicateId { .. } => ErrorCode::DuplicateId,
            Self::MissingReference { .. } => ErrorCode::MissingReference,
            Self::InvalidReceiptResolutionBinding { .. } => ErrorCode::ReceiptOrphanDetected,
            Self::InvalidTerminalReceiptReference { .. } => ErrorCode::MissingReference,
        }
    }

    pub fn related_objects(&self) -> Vec<String> {
        match self {
            Self::MultiAreaGraph {
                object_type,
                object_id,
                ..
            } => vec![format!("{object_type}:{object_id}")],

            Self::DuplicateId {
                object_type,
                object_id,
            } => vec![format!("{object_type}:{object_id}")],

            Self::MissingReference {
                object_type,
                object_id,
                missing_object_type,
                missing_object_id,
                ..
            } => vec![
                format!("{object_type}:{object_id}"),
                format!("{missing_object_type}:{missing_object_id}"),
            ],

            Self::InvalidReceiptResolutionBinding { receipt_id, .. } => {
                vec![format!("receipt:{receipt_id}")]
            }

            Self::InvalidTerminalReceiptReference {
                session_id,
                receipt_id,
            } => vec![
                format!("session:{session_id}"),
                format!("receipt:{receipt_id}"),
            ],
        }
    }

    pub fn to_error_entry(&self) -> ErrorEntry {
        ErrorEntry::new(
            ReportPhase::StructuralValidation,
            self.code(),
            self.related_objects(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    NotFound {
        object_type: String,
        object_id: String,
    },
    InvalidInput(String),
    InitializationFailed {
        errors: Vec<IntegrityError>,
    },
    InvalidState {
        message: String,
    },
}

impl EngineError {
    pub fn not_found(object_type: impl Into<String>, object_id: impl Into<String>) -> Self {
        Self::NotFound {
            object_type: object_type.into(),
            object_id: object_id.into(),
        }
    }
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound {
                object_type,
                object_id,
            } => write!(f, "{} not found: {}", object_type, object_id),

            Self::InvalidInput(msg) => write!(f, "invalid input: {}", msg),

            Self::InitializationFailed { errors } => {
                write!(f, "initialization failed with {} integrity error(s)", errors.len())
            }

            Self::InvalidState { message } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for EngineError {}