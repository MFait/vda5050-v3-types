#[test]
fn readme_snippets_match_tested_examples() {
    let readme = include_str!("../README.md");
    let snippets: Vec<_> = readme
        .split("```rust,ignore\n")
        .skip(1)
        .map(|block| block.split_once("\n```").unwrap().0)
        .collect();
    let examples = [
        include_str!("../examples/serde.rs"),
        include_str!("../examples/json.rs"),
        include_str!("../examples/topic.rs"),
        include_str!("../examples/topic_serde.rs"),
        include_str!("../examples/topic_json.rs"),
    ];
    assert_eq!(snippets.len(), examples.len());
    for (snippet, example) in snippets.iter().zip(examples) {
        assert_eq!(*snippet, example.trim_end());
    }
    for block in readme.split("```rust").skip(1) {
        let code = block.split_once("\n```").unwrap().0;
        assert!(!code.lines().any(|line| line.starts_with("# ")));
    }
}

#[cfg(feature = "serde")]
mod serde {
    include!("../examples/serde.rs");

    #[test]
    fn runs() {
        main().unwrap();
    }
}

#[cfg(feature = "json")]
mod json {
    include!("../examples/json.rs");

    #[test]
    fn runs() {
        main().unwrap();
    }
}

#[cfg(feature = "topic")]
mod topic {
    include!("../examples/topic.rs");

    #[test]
    fn runs() {
        main().unwrap();
    }
}

#[cfg(all(feature = "topic", feature = "serde"))]
mod topic_serde {
    include!("../examples/topic_serde.rs");

    #[test]
    fn runs() {
        main().unwrap();
    }
}

#[cfg(all(feature = "topic", feature = "json"))]
mod topic_json {
    include!("../examples/topic_json.rs");

    #[test]
    fn runs() {
        main().unwrap();
    }
}
