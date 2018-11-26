use super::{AccessModifier, Annotation, Class, CompilationUnit, Field, FieldType, Import, Method};

use nom::multispace;

fn is_word(b: u8) -> bool {
    match b {
        b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'.' => true,
        _ => false,
    }
}

named!(
    word<String>,
    map_res!(take_while!(is_word), |b: &[u8]| String::from_utf8(
        b.to_vec()
    ))
);

named!(
    comment<()>,
    do_parse!(tag!("//") >> take_until_and_consume!("\n") >> ())
);
named!(
    block_comment<()>,
    do_parse!(tag!("/*") >> take_until_and_consume!("*/") >> ())
);
named!(
    br<()>,
    alt!(map!(multispace, |_| ()) | comment | block_comment)
);

named!(
    import<String>,
    do_parse!(tag!("import") >> many1!(br) >> import: word >> many0!(br) >> tag!(";") >> (import))
);

named!(
    field_type<FieldType>,
    alt!(tag!("String") => { |_| FieldType::String } |
    tag!("Boolean") => { |_| FieldType::Boolean } |
    tag!("boolean") => { |_| FieldType::Boolean } |
    tag!("long") => { |_| FieldType::Long } |
    tag!("Long") => { |_| FieldType::Long } |
    tag!("int") => { |_| FieldType::Int } |
    tag!("Integer") => { |_| FieldType::Int } |
    tag!("short") => { |_| FieldType::Short } |
    word => { |w| FieldType::Type(w) })
);

named!(
    access_modifier<AccessModifier>,
    alt!(tag!("public") => { |_| AccessModifier::Public } |
    tag!("protected") => { |_| AccessModifier::Protected } |
    tag!("private") => { |_| AccessModifier::Private })
);

named!(
    package<String>,
    do_parse!(
        tag!("package") >> many1!(br) >> package: word >> many0!(br) >> tag!(";") >> (package)
    )
);

named!(
    annotation<Annotation>,
    do_parse!(
        tag!("@")
            >> name: word
            >> tag!("(")
            >> options: take_until!(")")
            >> tag!(")")
            >> (Annotation {
                name: name,
                options: String::from_utf8_lossy(&options.to_vec()).to_string(),
            })
    )
);

named!(
    method<Method>,
    do_parse!(
        access_modifier: opt!(access_modifier)
            >> many1!(br)
            >> typ: field_type
            >> many1!(br)
            >> name: word
            >> tag!("(")
            >> arguments: take_until!(")")
            >> tag!(")")
            >> many0!(br)
            >> tag!("{")
            >> take_until!("}")
            >> tag!("}")
            >> (Method {
                name: name,
                return_type: typ,
                arguments: String::from_utf8_lossy(&arguments.to_vec()).to_string(),
                access_modifier: access_modifier,
            })
    )
);

named!(
    class_field<Field>,
    do_parse!(
        access_modifier: opt!(access_modifier)
            >> many1!(br)
            >> typ: field_type
            >> many1!(br)
            >> name: word
            >> many0!(br)
            >> tag!(";")
            >> (Field {
                name: name,
                field_type: typ,
                access_modifier: access_modifier,
            })
    )
);

enum ClassEvent {
    Field(Field),
    Method(Method),
    Annotation(Annotation),
    Ignore,
}

named!(
    class_event<ClassEvent>,
    alt!(class_field => { |f| ClassEvent::Field(f) } |
    method => { |m| ClassEvent::Method(m) } |
    annotation => { |a| ClassEvent::Annotation(a) } |
    br => { |_| ClassEvent::Ignore })
);

named!(
    class_events<(String, Vec<ClassEvent>, Option<AccessModifier>)>,
    do_parse!(
        access_modifier: opt!(access_modifier)
            >> many0!(br)
            >> tag!("class")
            >> many1!(br)
            >> name: word
            >> many0!(br)
            >> tag!("{")
            >> many0!(br)
            >> events: many0!(class_event)
            >> many0!(br)
            >> tag!("}")
            >> many0!(br)
            >> many0!(tag!(";"))
            >> ((name, events, access_modifier))
    )
);

named!(
    class<Class>,
    map!(
        class_events,
        |(name, events, access_modifier): (String, Vec<ClassEvent>, Option<AccessModifier>)| {
            let mut class = Class {
                name: name.to_owned(),
                fields: Vec::new(),
                methods: Vec::new(),
                access_modifier: access_modifier,
                annotations: Vec::new(),
            };

            for event in events {
                match event {
                    ClassEvent::Field(f) => class.fields.push(f),
                    ClassEvent::Method(m) => class.methods.push(m),
                    ClassEvent::Annotation(a) => class.annotations.push(a),
                    ClassEvent::Ignore => (),
                }
            }
            class
        }
    )
);

enum Event {
    Package(String),
    Import(String),
    Annotation(Annotation),
    Class(Class),
    Ignore,
}

named!(
    event<Event>,
    alt!(
        package => { |p| Event::Package(p) } |
        import => { |i| Event::Import(i) } |
        class => { |c| Event::Class(c) } |
        annotation => { |a| Event::Annotation(a) } |
        br => { |_| Event::Ignore }
    )
);

named!(pub compilation_unit<CompilationUnit>, map!(many0!(event), |events: Vec<Event>| {
    let mut cu = CompilationUnit::new();

    for event in events {
        match event {
            Event::Package(p) => cu.package = Some(p),
            Event::Import(i) => cu.imports.push(Import { path: i }),
            Event::Class(c) => cu.classes.push(c),
            Event::Annotation(a) => cu.annotations.push(a),
            Event::Ignore => (),
        }
    }
    cu
}));
