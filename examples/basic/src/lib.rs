rofi_mode::export_mode!(Mode<'_>);

struct Mode<'rofi> {
    api: rofi_mode::Api<'rofi>,
    entries: Vec<String>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "plugin-example-basic\0";
    const DISPLAY_NAME: &'static str = "A basic Rofi plugin\0";

    fn init(api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        Ok(Self {
            api,
            entries: Vec::new(),
        })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_style(&mut self, _line: usize) -> rofi_mode::Style {
        rofi_mode::Style::NORMAL
    }

    fn entry(&mut self, line: usize) -> rofi_mode::Entry {
        rofi_mode::Entry::from(&self.entries[line]).with_style(self.entry_style(line))
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        input: &mut rofi_mode::String,
        selected_line: usize,
    ) -> rofi_mode::Action {
        match event {
            rofi_mode::Event::Cancel => return rofi_mode::Action::Exit,
            rofi_mode::Event::Ok { alt: _ } => {
                println!("Selected option {:?}", self.entries[selected_line]);
                return rofi_mode::Action::Exit;
            }
            rofi_mode::Event::CustomInput { alt: _ } => {
                self.entries.push(input.into());
                input.clear();
            }
            rofi_mode::Event::DeleteEntry => {
                self.entries.remove(selected_line);
            }
            rofi_mode::Event::Complete => {
                input.clear();
                input.push_str(&self.entries[selected_line]);
            }
            rofi_mode::Event::CustomCommand(_) => {}
        }
        rofi_mode::Action::Reload
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&*self.entries[line])
    }

    fn entry_icon(&mut self, _line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        self.api.query_icon("computer", height).wait(&mut self.api)
    }

    fn message(&mut self) -> rofi_mode::String {
        let entries = self.entries.len();
        if entries == 1 {
            "1 entry registered".into()
        } else {
            rofi_mode::format!("{entries} entries registered")
        }
    }
}
