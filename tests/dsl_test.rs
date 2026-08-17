#[cfg(test)]
mod tests {
    use genrate_ppt::dsl::{Component, PresentationSpec, SlideKind, SlideSpec};

    #[test]
    fn validates_a_valid_presentation() {
        let spec = PresentationSpec {
            title: "Rust Architecture".into(),
            theme: "technical".into(),
            slides: vec![SlideSpec {
                id: "slide-1".into(),
                kind: SlideKind::Cover,
                title: "Rust Architecture".into(),
                subtitle: None,
                components: vec![Component::Text {
                    text: "Introduction".into(),
                    role: Default::default(),
                }],
            }],
        };

        assert!(spec.validate(1).is_ok());
    }

    #[test]
    fn rejects_empty_title() {
        let spec = PresentationSpec {
            title: "  ".into(),
            theme: String::new(),
            slides: vec![SlideSpec {
                id: "slide-1".into(),
                kind: SlideKind::Content,
                title: "Content".into(),
                subtitle: None,
                components: vec![],
            }],
        };

        assert!(spec.validate(1).is_err());
    }
}
