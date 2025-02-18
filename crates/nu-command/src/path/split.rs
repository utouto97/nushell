use std::path::{Component, Path};

use nu_protocol::ast::Call;
use nu_protocol::engine::{EngineState, Stack, StateWorkingSet};
use nu_protocol::{
    engine::Command, Category, Example, PipelineData, ShellError, Signature, Span, Type, Value,
};

use super::PathSubcommandArguments;

struct Arguments;

impl PathSubcommandArguments for Arguments {}

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "path split"
    }

    fn signature(&self) -> Signature {
        Signature::build("path split")
            .input_output_types(vec![
                (Type::String, Type::List(Box::new(Type::String))),
                (
                    Type::List(Box::new(Type::String)),
                    Type::List(Box::new(Type::List(Box::new(Type::String)))),
                ),
            ])
            .category(Category::Path)
    }

    fn usage(&self) -> &str {
        "Split a path into a list based on the system's path separator."
    }

    fn is_const(&self) -> bool {
        true
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let args = Arguments;

        // This doesn't match explicit nulls
        if matches!(input, PipelineData::Empty) {
            return Err(ShellError::PipelineEmpty { dst_span: head });
        }
        input.map(
            move |value| super::operate(&split, &args, value, head),
            engine_state.ctrlc.clone(),
        )
    }

    fn run_const(
        &self,
        working_set: &StateWorkingSet,
        call: &Call,
        input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;
        let args = Arguments;

        // This doesn't match explicit nulls
        if matches!(input, PipelineData::Empty) {
            return Err(ShellError::PipelineEmpty { dst_span: head });
        }
        input.map(
            move |value| super::operate(&split, &args, value, head),
            working_set.permanent().ctrlc.clone(),
        )
    }

    #[cfg(windows)]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Split a path into parts",
                example: r"'C:\Users\viking\spam.txt' | path split",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string(r"C:\"),
                        Value::test_string("Users"),
                        Value::test_string("viking"),
                        Value::test_string("spam.txt"),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Split paths in list into parts",
                example: r"[ C:\Users\viking\spam.txt C:\Users\viking\eggs.txt ] | path split",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_list(vec![
                            Value::test_string(r"C:\"),
                            Value::test_string("Users"),
                            Value::test_string("viking"),
                            Value::test_string("spam.txt"),
                        ]),
                        Value::test_list(vec![
                            Value::test_string(r"C:\"),
                            Value::test_string("Users"),
                            Value::test_string("viking"),
                            Value::test_string("eggs.txt"),
                        ]),
                    ],
                    span: Span::test_data(),
                }),
            },
        ]
    }

    #[cfg(not(windows))]
    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Split a path into parts",
                example: r"'/home/viking/spam.txt' | path split",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_string("/"),
                        Value::test_string("home"),
                        Value::test_string("viking"),
                        Value::test_string("spam.txt"),
                    ],
                    span: Span::test_data(),
                }),
            },
            Example {
                description: "Split paths in list into parts",
                example: r"[ /home/viking/spam.txt /home/viking/eggs.txt ] | path split",
                result: Some(Value::List {
                    vals: vec![
                        Value::test_list(vec![
                            Value::test_string("/"),
                            Value::test_string("home"),
                            Value::test_string("viking"),
                            Value::test_string("spam.txt"),
                        ]),
                        Value::test_list(vec![
                            Value::test_string("/"),
                            Value::test_string("home"),
                            Value::test_string("viking"),
                            Value::test_string("eggs.txt"),
                        ]),
                    ],
                    span: Span::test_data(),
                }),
            },
        ]
    }
}

fn split(path: &Path, span: Span, _: &Arguments) -> Value {
    Value::List {
        vals: path
            .components()
            .filter_map(|comp| {
                let comp = process_component(comp);
                comp.map(|s| Value::string(s, span))
            })
            .collect(),
        span,
    }
}

#[cfg(windows)]
fn process_component(comp: Component) -> Option<String> {
    match comp {
        Component::RootDir => None,
        Component::Prefix(_) => {
            let mut s = comp.as_os_str().to_string_lossy().to_string();
            s.push('\\');
            Some(s)
        }
        comp => Some(comp.as_os_str().to_string_lossy().to_string()),
    }
}

#[cfg(not(windows))]
fn process_component(comp: Component) -> Option<String> {
    Some(comp.as_os_str().to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
