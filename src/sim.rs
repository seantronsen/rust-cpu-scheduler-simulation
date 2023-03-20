use crate::{ProgramError, Result};

#[derive(PartialEq, Debug)]
pub struct SimProcess {
    pub name: String,
    pub priority: u8,
    pub burst: u32,
    pub wait: u32,
}

impl std::fmt::Display for SimProcess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Process: {} | Priority: {} | Remaining Burst: {} | Wait Time: {}",
            self.name, self.priority, self.burst, self.wait,
        )
    }
}

impl TryFrom<String> for SimProcess {
    type Error = ProgramError;
    fn try_from(value: String) -> Result<Self> {
        let mut components = value.split(",").map(|s| String::from(s.trim()));
        let name = match components.next() {
            Some(str) => str,
            _ => {
                return Err(ProgramError::InvalidProcessSpecification(String::from(
                    value,
                )))
            }
        };

        let priority = match components.next() {
            Some(str) => str.parse::<u8>()?,
            _ => {
                return Err(ProgramError::InvalidProcessSpecification(String::from(
                    value,
                )))
            }
        };

        let burst = match components.next() {
            Some(str) => str.parse::<u32>()?,
            _ => {
                return Err(ProgramError::InvalidProcessSpecification(String::from(
                    value,
                )))
            }
        };

        Ok(SimProcess::new(name, priority, burst))
    }
}

impl SimProcess {
    pub fn new(name: String, priority: u8, burst: u32) -> Self {
        Self {
            name,
            priority,
            burst,
            wait: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_reference_process() -> SimProcess {
        SimProcess::new("T1".to_string(), 5, 25)
    }

    #[test]
    fn can_parse_process_from_string() -> Result<()> {
        let line = "T1,5,25".to_string();
        let process = SimProcess::try_from(line);

        assert_eq!(build_reference_process(), process?);

        Ok(())
    }

    #[test]
    fn shouldnt_parse_process_from_invalid_string() {
        let line = String::from("T1, 23, ");
        assert!(SimProcess::try_from(line).is_err());

        let line = String::from("T1, 5, abc");
        assert!(SimProcess::try_from(line).is_err());
    }

    #[test]
    fn valid_display() {
        let reference_display_string =
            "Process: T1 | Priority: 5 | Remaining Burst: 25 | Wait Time: 0";
        assert_eq!(
            build_reference_process().to_string(),
            reference_display_string
        );
    }
}
