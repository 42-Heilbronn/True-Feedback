/// Contains the static feedback from

use serde::{Serialize, Deserialize};
use validator::{Validate};

#[derive(Deserialize, Serialize, Validate)]
pub struct Feedback {
    #[validate(range(min = 0, max = 10))]
    pub understanding: i32,
    #[validate(range(min = 0, max = 10))]
    pub uniqueness: i32,
    #[validate(range(min = 0, max = 10))]
    pub friendliness: i32,
    #[validate(length(max = 1024))]
    pub comment: Option<String>
}

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

pub static FEEDBACK_FIELDS_EVALUATION: [FeedbackStructureField; 4] = [
    FeedbackStructureField {
        key: "understanding",
        name: "The code was thoroughly understood",
        description: "Any Questions regarding the overall structure, design choices and individual functions could be answered flawlessly.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "uniqueness",
        name: "The solution was unique",
        description: "The solution provided a fresh perspective or approach that set it apart from conventional methods or existing alternatives?",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "friendliness",
        name: "The evaluation was very pleasant",
        description: "The atmosphere throughout the entire process was very friendly. There was no discomfort and no uneasiness.",
        data_type: FeedbackStructureFieldType::Range(0, 10),
    },
    FeedbackStructureField {
        key: "comment",
        name: "Comment",
        description: "Optional comment you would like to share with bocal",
        data_type: FeedbackStructureFieldType::String(1024),
    }
];
