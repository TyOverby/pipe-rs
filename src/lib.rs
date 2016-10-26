extern crate itertools;

#[derive(Eq, PartialEq, Debug)]
pub enum Node {
    Section(Section),
    Text(String),
    EmptyLine,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Section {
    name: String,
    label: Option<String>,
    children: Vec<Node>
}

fn parse_nodes<I>(parent_level: i32, mut lines: &mut itertools::PutBack<I>) -> Vec<Node>
where I: Iterator<Item=String> {
    #[derive(PartialEq, Eq)]
    enum LineType {
        Blank,
        Text,
        Section
    }

    let mut out = vec![];

    while let Some(mut line) = lines.next() {
        let mut typ = LineType::Blank;
        let mut indentation = 0;

        for c in line.chars() {
            match c {
                ' ' | '\t' => indentation += 1,
                '|' => {
                    typ = LineType::Section;
                    break;
                }
                other => {
                    typ = LineType::Text;
                    break;
                }
            }
        }

        if indentation <= parent_level && typ != LineType::Blank {
            lines.put_back(line);
            break;
        }

        match typ {
            LineType::Blank => out.push(Node::EmptyLine),
            LineType::Text => {
                line.drain(.. indentation as usize);
                out.push(Node::Text(line));
            }
            LineType::Section => {
                unimplemented!();
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools;

    fn parse_string(s: &str) -> Vec<Node> {
        ::parse_nodes(-1, &mut itertools::put_back(s.lines().map(String::from)))
    }

    #[test]
    fn parse_empty() {
        let s = "";
        assert_eq!(parse_string(s), vec![]);
    }

    #[test]
    fn parse_single_text() {
        let s = "hello world";
        assert_eq!(parse_string(s), vec![Node::Text("hello world".into())]);
    }

    #[test]
    fn parse_multiline_text() {
        let s =
r#"hello world
this is a test
"#;
        assert_eq!(parse_string(s), vec![
                   Node::Text("hello world".into()),
                   Node::Text("this is a test".into())]);
    }

    #[test]
    fn parse_multiline_text_with_blank() {
        let s =
r#"hello world

this is a test
"#;
        assert_eq!(parse_string(s), vec![
                   Node::Text("hello world".into()),
                   Node::EmptyLine,
                   Node::Text("this is a test".into())]);
    }

    #[test]
    fn parse_multiline_text_with_jagged_start() {
        let s =
r#"hello world
   this is a test
"#;
        assert_eq!(parse_string(s), vec![
                   Node::Text("hello world".into()),
                   Node::Text("this is a test".into())]);
    }
}
