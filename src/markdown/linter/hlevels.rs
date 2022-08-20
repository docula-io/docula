use super::handler::lint::{Linter, Issue};
use ordinal::Ordinal;

pub struct HeadingLevels {}

impl Linter for HeadingLevels {
    fn name(&self) -> &'static str {
        "Heading Levels"
    }

    fn description(&self) -> &'static str {
        "Checks that no heading levels are skipped in a document."
    }

    fn lint(&self, content: &str) -> Vec<Issue> {
        let mut current_depth = 0;
        let mut res = Vec::new();

        for (num, line) in content.lines().enumerate() {
           if let Some(depth) = heading_count(line) {
               let delta = depth - current_depth;
               if delta > 1 {
                   let start = line.find('#').expect("A # should exist for us to get here");
                   let end = line.len();
                   let msg = format!(
                       "Skipped {} level header", Ordinal(current_depth + 1).to_string(),
                   );

                   res.push(Issue{
                       line_start: num,
                       line_end: num,
                       col_start: start,
                       col_end: end,
                       content: line.to_owned(),
                       msg,
                   });
               }

               current_depth = depth;
           }
        }

        res
    }
}

fn heading_count(line: &str) -> Option<usize> {
    match line.trim_start().chars().take_while(|x| x == &'#').collect::<Vec<char>>().len() {
        0 => None,
        x => match x > 6 {
            true => None,
            false => Some(x),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! header_lint_test {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let linter = HeadingLevels{};
                    let (input, expected) = $value;

                    assert_eq!(expected, linter.lint(input))
                }
            )*
        }
    }

    header_lint_test! {
        happy_path: (
            "# Heading 1\n\nHello, world\n## Heading 2\n ### Heading 3",
            Vec::<Issue>::new(),
        ),

        bad_second_heading: (
            "# Heading 1\n\nFoo bar\n### Heading 2", vec![Issue{
                line_start: 3,
                line_end: 3,
                col_start: 0,
                col_end: 13,
                content: "### Heading 2".to_owned(),
                msg: "Skipped 2nd level header".to_owned(),
            }]
        ),

        bad_first_heading: ("## Foo", vec![Issue{
            line_start: 0,
            line_end: 0,
            col_start: 0,
            col_end: 6,
            content: "## Foo".to_string(),
            msg: "Skipped 1st level header".to_owned(),
        }]),

        space_before_happy: (
            " # Foo\n     ## Bar\n\nHello, world\n    ### Zoom",
            Vec::<Issue>::new()
        ),
        
        hashes_per_line: (
            "# Foo ##### Foo\n## Bar ######Bar", Vec::<Issue>::new()
        ),

        non_normal_header: (
            "# Intro\n####### Not a header\n## Foo", Vec::<Issue>::new(),
        ),
    }
}
