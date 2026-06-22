use crate::{Error, ErrorCode, Result, error::validate_finite};

#[derive(Clone, Debug, PartialEq)]
pub enum CalcLength {
    Px(f32),
    Percent(f32),
    Sum(Vec<CalcLengthTerm>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CalcLengthTerm {
    pub operator: CalcOperator,
    pub value: Box<CalcLength>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CalcOperator {
    Add,
    Sub,
}

impl CalcLength {
    #[must_use]
    pub const fn px(value: f32) -> Self {
        Self::Px(value)
    }

    pub fn try_px(value: f32) -> Result<Self> {
        validate_finite(value, "calc px term")?;
        Ok(Self::Px(value))
    }

    #[must_use]
    pub const fn percent(value: f32) -> Self {
        Self::Percent(value)
    }

    pub fn try_percent(value: f32) -> Result<Self> {
        validate_finite(value, "calc percent term")?;
        Ok(Self::Percent(value))
    }

    #[must_use]
    pub fn sum(terms: impl IntoIterator<Item = CalcLengthTerm>) -> Self {
        Self::Sum(terms.into_iter().collect())
    }

    #[must_use]
    pub fn uses_percentage(&self) -> bool {
        match self {
            Self::Px(_) => false,
            Self::Percent(_) => true,
            Self::Sum(terms) => terms.iter().any(|term| term.value.uses_percentage()),
        }
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Px(value) => validate_finite(*value, "calc px term"),
            Self::Percent(value) => validate_finite(*value, "calc percent term"),
            Self::Sum(terms) if terms.is_empty() => Err(Error::new(
                ErrorCode::InvalidValue,
                "calc sum must contain at least one term",
            )),
            Self::Sum(terms) => terms.iter().try_for_each(|term| term.value.validate()),
        }
    }

    #[must_use]
    pub fn to_css_string(&self) -> String {
        match self {
            Self::Px(value) => format_number(*value, "px"),
            Self::Percent(value) => format_number(*value, "%"),
            Self::Sum(terms) => {
                let mut output = String::from("calc(");
                for (index, term) in terms.iter().enumerate() {
                    if index == 0 {
                        match term.operator {
                            CalcOperator::Add => output.push_str(&term.value.to_css_string()),
                            CalcOperator::Sub => {
                                output.push('-');
                                output.push_str(&term.value.to_css_string());
                            }
                        }
                    } else {
                        match term.operator {
                            CalcOperator::Add => output.push_str(" + "),
                            CalcOperator::Sub => output.push_str(" - "),
                        }
                        output.push_str(&term.value.to_css_string());
                    }
                }
                output.push(')');
                output
            }
        }
    }
}

impl CalcLengthTerm {
    #[must_use]
    pub fn add(value: CalcLength) -> Self {
        Self {
            operator: CalcOperator::Add,
            value: Box::new(value),
        }
    }

    #[must_use]
    pub fn sub(value: CalcLength) -> Self {
        Self {
            operator: CalcOperator::Sub,
            value: Box::new(value),
        }
    }
}

fn format_number(value: f32, suffix: &str) -> String {
    format!("{value}{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ErrorCode;

    #[test]
    fn calc_length_reports_percentage_use() {
        let calc = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::px(20.0)),
            CalcLengthTerm::add(CalcLength::percent(10.0)),
        ]);

        assert!(calc.uses_percentage());
        assert_eq!(calc.to_css_string(), "calc(20px + 10%)");
    }

    #[test]
    fn calc_length_rejects_non_finite_terms() {
        let error = CalcLength::try_px(f32::NAN).unwrap_err();
        assert_eq!(error.code(), ErrorCode::InvalidValue);
    }

    #[test]
    fn nested_calc_serializes_stably() {
        let inner = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::px(12.0)),
            CalcLengthTerm::add(CalcLength::percent(3.0)),
        ]);
        let outer = CalcLength::sum([
            CalcLengthTerm::add(CalcLength::percent(100.0)),
            CalcLengthTerm::sub(inner),
        ]);

        assert_eq!(outer.to_css_string(), "calc(100% - calc(12px + 3%))");
    }
}
