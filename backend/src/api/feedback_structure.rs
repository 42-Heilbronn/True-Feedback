//// Contains the static feedback form

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::enums::FeedbackKind;

#[derive(Serialize)]
pub struct FeedbackStructure {
    pub id: i32,
    pub fields: Vec<FeedbackStructureField>,
}

#[derive(Clone, Serialize)]
pub struct FeedbackStructureField {
    key: &'static str,
    name: &'static str,
    description: &'static str,
    data_type: FeedbackStructureFieldType,
}

#[derive(Clone, Serialize)]
pub enum FeedbackStructureFieldType {
    Range(i32, i32),
    String(i32),
}

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Feedback {
    FeedbackEvaluator(FeedbackEvaluator),
    FeedbackEvaluated(FeedbackEvaluated),
}

impl Feedback {
    pub fn is_kind(&self, kind: FeedbackKind) -> bool {
        match self {
            Feedback::FeedbackEvaluator(_) => kind == FeedbackKind::Evaluator,
            Feedback::FeedbackEvaluated(_) => kind == FeedbackKind::Evaluated,
        }
    }

    pub fn current_id(&self) -> i32 {
        match self {
            Feedback::FeedbackEvaluator(_) => FEEDBACK_EVALUATOR_ID,
            Feedback::FeedbackEvaluated(_) => FEEDBACK_EVALUATED_ID,
        }
    }
}

// EVALUATOR

// Increment if there are changes to the stored data
const FEEDBACK_EVALUATOR_ID: i32 = 1;

#[derive(Deserialize, Serialize, Validate)]
pub struct FeedbackEvaluator {
    #[validate(range(min = 0, max = 10))]
    pub understanding: i32,
    #[validate(range(min = 0, max = 10))]
    pub uniqueness: i32,
    #[validate(range(min = 0, max = 10))]
    pub friendliness: i32,
    #[validate(length(max = 1024))]
    pub comment: Option<String>,
}

pub static FEEDBACK_EVALUATOR_FIELDS: [FeedbackStructureField; 4] = [
    FeedbackStructureField {
        key: "understanding",
        name: "The code was thoroughly understood",
        description: "Any Questions regarding the overall structure, design choices \
                        and individual functions could be answered flawlessly.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "uniqueness",
        name: "The solution was unique",
        description: "The solution provided a fresh perspective or approach that \
                        set it apart from conventional methods or existing alternatives?",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "friendliness",
        name: "The evaluation was very pleasant",
        description: "The atmosphere throughout the entire process was very friendly. \
                        There was no discomfort and no uneasiness.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "comment",
        name: "Comment",
        description: "Optional comment you would like to share with bocal",
        data_type: FeedbackStructureFieldType::String(1024),
    },
];

// EVALUATED

// Increment if there are changes to the stored data
const FEEDBACK_EVALUATED_ID: i32 = 1;

#[derive(Deserialize, Serialize, Validate)]
pub struct FeedbackEvaluated {
    #[validate(range(min = 0, max = 10))]
    pub rigorness: i32,
    #[validate(range(min = 0, max = 10))]
    pub friendliness: i32,
    #[validate(length(max = 1024))]
    pub comment: Option<String>,
}

pub static FEEDBACK_EVALUATED_FIELDS: [FeedbackStructureField; 3] = [
    FeedbackStructureField {
        key: "rigorness",
        name: "The code was rigorously examined",
        description: "Every aspect of the code, from logic and functionality \
                        to performance and security, was subjected to thorough and meticulous \
                        scrutiny to ensure a robust and reliable solution.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "friendliness",
        name: "The evaluation was very pleasant",
        description: "The atmosphere throughout the entire process was very friendly. \
                        There was no discomfort and no uneasiness.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "comment",
        name: "Comment",
        description: "Optional comment you would like to share with bocal",
        data_type: FeedbackStructureFieldType::String(1024),
    },
];
