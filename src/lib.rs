use std::fmt::Debug;
use std::fmt::Display;
use std::fmt;
use std::rc::Rc;

//TODO cloning???
//TODO cloning children???


type Children = Vec<Rc<ElementTrait>>;

trait ElementChildren {
    fn get_children(&self) -> &Children;
}

trait ElementTag {
    fn get_tag(&self) -> &String;
}

trait ElementTrait : ElementChildren + ElementTag + Debug + Display {
    fn render(&self) -> Option<Rc<ElementTrait>> {
        None
    }

    fn should_render(&self) -> bool {
        true
    }

    fn render_to_string(&self) -> String {
        if self.should_render() {
            match self.render() {
                Some(element) => element.render_to_string(),
                None => String::new(),
            }
        } else {
            format!("<{tag}>{}</{tag}>", self.render_children_to_string(), tag=self.get_tag())
        }
    }

    fn render_children_to_string(&self) -> String {
        self.get_children().iter()
            .map(|child| child.render_to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
}


#[derive(Debug)]
struct ElementStruct<P: Debug + Clone> {
    element_type: String,
    props: P,
    children: Children,
}

impl<P: Debug + Clone> ElementChildren for ElementStruct<P> {
    fn get_children(&self) -> &Children {
        &self.children
    }
}

impl<P: Debug + Clone> ElementTag for ElementStruct<P> {
    fn get_tag(&self) -> &String {
        &self.element_type
    }
}

impl<P: Debug + Clone> Display for ElementStruct<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "- {},  {:?}\n", self.element_type, self.props).unwrap();
        for c in &self.children {
            write!(f, "  {}\n", c).unwrap();
        }

        write!(f, "\n")
    }
}

fn create_element<P>(props: P, children: Children) -> Rc<ElementTrait>
    where ElementStruct<P>: ElementTrait,
          P: Debug + Clone + 'static
{

    // Infer the name of the element (tag name)
    let raw = format!("{:?}", props);
    let parts: Vec<&str> = raw.split("{").collect();
    let struct_name = parts[0].to_string();
    let element_type = if struct_name.contains("Props") {
        let parts: Vec<&str> = struct_name.split("Props").collect();
        let name = parts[0].to_string();
        name
    } else {
        struct_name
    };


    Rc::new(
        ElementStruct {
            element_type: element_type.to_lowercase(),
            children: children,
            props: props,
        }
    )
}


#[derive(Debug, Clone)]
struct TextProps {
    text: String,
}

impl ElementTrait for ElementStruct<TextProps> {
    fn render_to_string(&self) -> String {
        self.props.text.clone()
    }

    fn should_render(&self) -> bool {
        false
    }
}


#[derive(Debug, Clone)]
struct DivProps {
    class: String,
}

impl ElementTrait for ElementStruct<DivProps> {
    fn should_render(&self) -> bool {
        false
    }
}


#[derive(Debug, Clone)]
struct PProps {
    class: String,
}

impl ElementTrait for ElementStruct<PProps> {
    fn should_render(&self) -> bool {
        false
    }
}



#[derive(Debug, Clone)]
struct PersonProps {
    name: String,
}

impl ElementTrait for ElementStruct<PersonProps> {
    fn render(&self) -> Option<Rc<ElementTrait>> {
        Some(create_element(NameProps{name: self.props.name.clone(), show: true}, vec![]))
    }
}

#[derive(Debug, Clone)]
struct NameProps {
    show: bool,
    name: String,
}

impl ElementTrait for ElementStruct<NameProps> {
    fn render(&self) -> Option<Rc<ElementTrait>> {
        // In here custom logic will come
        if !self.props.show {
            return Some(create_element(
                    PProps{class: String::new()},
                    vec![
                        create_element(TextProps{text: "NOT_SHOWING".to_string()}, vec![]),
                    ]
                ))
        }


        Some(
            create_element(
                DivProps { class: "form-control".to_string()},
                vec![
                    create_element(TextProps{text: "SHOWING".to_string()}, vec![]),
                    create_element(TextProps{text: self.props.name.clone()}, vec![]),
                ]
            )
        )
    }
}

#[derive(Debug, Clone)]
struct IfProps {
    cond: bool,
}

impl ElementTrait for ElementStruct<IfProps> {
    fn render(&self) -> Option<Rc<ElementTrait>> {

        if !self.props.cond {
            return None
        }


        Some(
            create_element(
                DivProps { class: "form-control".to_string()},
                self.children.clone(),
            )
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("+++++++++++++");
        let d1 = create_element(
            DivProps { class: "form-control".to_string()},
            vec![
                create_element(TextProps{text: "Im a content".to_string()}, vec![]),
                create_element(PProps{class: "paragraph".to_string()}, vec![]),
            ]
        );
        println!("{}", d1);
        println!("render\n{}", d1.render_to_string());
        println!("=============\n");


        println!("+++++++++++++");
        let c = create_element(NameProps{show: true, name: "Fran".to_string()}, vec![]);
        println!("{}", c);
        println!("render\n{}", c.render_to_string());
        println!("=============\n");

        println!("+++++++++++++");
        let c = create_element(PersonProps{name: "Fran".to_string()}, vec![]);
        println!("{}", c);
        println!("render\n{}", c.render_to_string());
        println!("=============\n");

        println!("+++++++++++++");
        let c = create_element(IfProps{cond: false}, vec![
            create_element(TextProps{text: "Im a content".to_string()}, vec![]),
        ]);

        println!("{}", c);
        println!("render\n{}", c.render_to_string());
        println!("=============\n");

        println!("+++++++++++++");
        let c = create_element(IfProps{cond: true}, vec![
            create_element(TextProps{text: "Im a content".to_string()}, vec![]),
        ]);

        println!("{}", c);
        println!("render\n{}", c.render_to_string());
        println!("=============\n");
    }
}
