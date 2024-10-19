use std::{fmt::Display, ops::RangeInclusive, path::PathBuf};

/// [debug]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#setting-a-debug-message
/// [notice]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#setting-a-notice-message
/// [warning]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#setting-a-warning-message
/// [error]: https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions#setting-an-error-message
enum GithubOutputLine {
    Debug(GithubOutputDetails),
    Notice(GithubOutputDetails),
    Warning(GithubOutputDetails),
    Error(GithubOutputDetails),
}

#[derive(Default)]
pub struct GithubOutputDetails {
    title: String,
    file: Option<PathBuf>,
    lines: Option<RangeInclusive<usize>>,
    cols: Option<RangeInclusive<usize>>,
    message: String,
}

impl GithubOutputDetails {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            ..Default::default()
        }
    }

    pub fn with_file(self, file: impl Into<PathBuf>) -> Self {
        Self {
            file: Some(file.into()),
            ..self
        }
    }

    pub fn with_lines(self, lines: RangeInclusive<usize>) -> Self {
        Self {
            lines: Some(lines),
            ..self
        }
    }
    pub fn with_cols(self, cols: RangeInclusive<usize>) -> Self {
        Self {
            cols: Some(cols),
            ..self
        }
    }
}

impl Display for GithubOutputDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            title,
            file,
            lines,
            cols,
            message,
        } = self;

        write!(f, "title={title}")?;

        if let Some(file) = file {
            write!(f, ",file={file}", file = file.display())?;
        }

        if let Some(lines) = lines {
            write!(f, ",line={line}", line = lines.start())?;
            if lines.end() > lines.start() {
                write!(f, ",endLine={line}", line = lines.end())?;
            }
        }

        if let Some(cols) = cols {
            write!(f, ",col={col}", col = cols.start())?;
            if cols.end() > cols.start() {
                write!(f, ",endColumn={col}", col = cols.end())?;
            }
        }

        write!(f, "::{message}")
    }
}

// ::warning title={title},file={name},line={line},endLine={endLine},col={col},endColumn={endColumn}::{message}
impl Display for GithubOutputLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GithubOutputLine::Debug(details) => write!(f, "::debug {details}"),
            GithubOutputLine::Notice(details) => write!(f, "::notice {details}"),
            GithubOutputLine::Warning(details) => write!(f, "::warning {details}"),
            GithubOutputLine::Error(details) => write!(f, "::error {details}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gihub_output() {
        let expected = "::warning title=test title,file=example.txt,line=5,col=10::Example message";
        let result = GithubOutputLine::Warning(
            GithubOutputDetails::new("test title", "Example message")
                .with_file("example.txt")
                .with_lines(5..=5)
                .with_cols(10..=10),
        )
        .to_string();

        assert_eq!(expected, result);
    }
}
