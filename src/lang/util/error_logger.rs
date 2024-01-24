use crate::lang::compiler::CompilerError;

#[derive(Clone)]
pub(crate) struct ErrorLogger<'a> {
    filename: &'a str,
    input: &'a str,
}

impl<'a> ErrorLogger<'a> {
    pub(crate) fn new(filename:&'a str , input: &'a str) -> Self {
        ErrorLogger { filename, input }
    }

    pub(crate) fn report<'b>(&'a self, error_detail: &'b CompilerError) -> () {
        use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
        let mut colors = ColorGenerator::new();
        let color_1 = colors.next();

        let mut report = Report::build(ReportKind::Error, self.filename, 0)
            .with_code(&error_detail.error_code)
            .with_message(&error_detail.error_message)
            .with_label(
                Label::new((self.filename, error_detail.token.span.clone()))
                    .with_message(&error_detail.span_message)
                    .with_color(color_1),
            );

        if let Some(help) = &error_detail.help {
            report = report.with_help(help);
        }

        if let Some(info) = &error_detail.info {
            report = report.with_note(info);
        }

        report
            .finish()
            .eprint((self.filename, Source::from(self.input)))
            .unwrap();
    }

    pub(crate) fn report_many(&self, errors: &[CompilerError]) -> () {
        for error in errors {
            self.report(error);
        }
    }
}
