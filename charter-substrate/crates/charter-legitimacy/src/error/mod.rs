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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorEntry {
    pub error_code: String,
    pub related_objects: Vec<String>,
    pub block_type: Option<BlockType>,
    pub block_scope: Option<BlockScope>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationReport {
    pub evaluation_id: Option<String>,
    pub command_type: String,
    pub target_object_type: String,
    pub target_object_id: Option<String>,
    pub outcome: EvaluationOutcome,
    pub errors: Vec<ErrorEntry>,
    pub primary_error_code: Option<String>,
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
        Self {
            evaluation_id: None,
            command_type: command_type.into(),
            target_object_type: target_object_type.into(),
            target_object_id: target_object_id.map(str::to_owned),
            outcome: EvaluationOutcome::Success,
            errors: Vec::new(),
            primary_error_code: None,
            diagnostics: None,
            occurred_at: None,
            schema_version: 1,
        }
    }

    pub fn blocked(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
        error_code: impl Into<String>,
    ) -> Self {
        let entry = ErrorEntry {
            error_code: error_code.into(),
            related_objects: Vec::new(),
            block_type: Some(BlockType::Temporary),
            block_scope: Some(BlockScope::Session),
        };

        Self {
            evaluation_id: None,
            command_type: command_type.into(),
            target_object_type: target_object_type.into(),
            target_object_id: target_object_id.map(str::to_owned),
            outcome: EvaluationOutcome::Blocked,
            errors: vec![entry.clone()],
            primary_error_code: Some(entry.error_code),
            diagnostics: None,
            occurred_at: None,
            schema_version: 1,
        }
    }

    pub fn rejected(
        command_type: impl Into<String>,
        target_object_type: impl Into<String>,
        target_object_id: Option<&str>,
        error_code: impl Into<String>,
    ) -> Self {
        let entry = ErrorEntry {
            error_code: error_code.into(),
            related_objects: Vec::new(),
            block_type: None,
            block_scope: None,
        };

        Self {
            evaluation_id: None,
            command_type: command_type.into(),
            target_object_type: target_object_type.into(),
            target_object_id: target_object_id.map(str::to_owned),
            outcome: EvaluationOutcome::Rejected,
            errors: vec![entry.clone()],
            primary_error_code: Some(entry.error_code),
            diagnostics: None,
            occurred_at: None,
            schema_version: 1,
        }
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
    pub fn code(&self) -> &'static str {
        match self {
            Self::MultiAreaGraph { .. } => "MULTI_AREA_GRAPH_DETECTED",
            Self::DuplicateId { .. } => "DUPLICATE_ID",
            Self::MissingReference { .. } => "MISSING_REFERENCE",
            Self::InvalidReceiptResolutionBinding { .. } => "INVALID_RECEIPT_RESOLUTION_BINDING",
            Self::InvalidTerminalReceiptReference { .. } => "MISSING_REFERENCE",
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
        ErrorEntry {
            error_code: self.code().to_string(),
            related_objects: self.related_objects(),
            block_type: None,
            block_scope: None,
        }
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
        }
    }
}

impl std::error::Error for EngineError {}