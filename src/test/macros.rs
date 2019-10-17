macro_rules! parametrize_versions {
    ( $it:item ) => {
        #[rstest_parametrize(v_string,
        n_parts,
        case("1", 1),
        case("1.2", 2),
        case("1.2.3.4", 4),
        case("1.2.3.4.5.6.7.8", 8),
        case("0", 1),
        case("0.0.0", 3),
        case("1.0.0", 3),
        case("0.0.1", 3),
        case("", 0),
        case(".", 0),
        case("...", 0),
        case("1.2.dev", 3),
        case("1.2-dev", 3),
        case("1.2.alpha.4", 4),
        case("1.2-alpha-4", 4),
        case("snapshot.1.2", 3),
        case("snapshot-1.2", 3),
        // TODO: inspect and fix this case
        // case("version-compare 2.1.8.1 / build 209", 4),
        )]
        $it
    };
}

macro_rules! parametrize_versions_errors {
    ( $it:item ) => {
        #[rstest_parametrize(v_string,
        n_parts,
        case("abc", 1),
        case("alpha.dev.snapshot", 3),
        case("test. .snapshot", 3),
        // TODO: broken case, decide what to do here
        // case("$", 1),
        )]
        $it
    };
}

/// List of version sets for dynamic tests
macro_rules! parametrize_versions_set {
    ( $it:item ) => {
        #[rstest_parametrize(a,
        b,
        operator,
        case("1", "1", &CompOp::Eq),
        case("1.0.0.0", "1", &CompOp::Eq),
        case("1", "1.0.0.0", &CompOp::Eq),
        case("0", "0", &CompOp::Eq),
        case("0.0.0", "0", &CompOp::Eq),
        case("0", "0.0.0", &CompOp::Eq),
        case("", "", &CompOp::Eq),
        case("", "0.0", &CompOp::Eq),
        case("0.0", "", &CompOp::Eq),
        case("", "0.1", &CompOp::Lt),
        case("0.1", "", &CompOp::Gt),
        case("1.2.3", "1.2.3", &CompOp::Eq),
        case("1.2.3", "1.2.4", &CompOp::Lt),
        case("1.0.0.1", "1.0.0.0", &CompOp::Gt),
        case("1.0.0.0", "1.0.0.1", &CompOp::Lt),
        case("1.2.3.4", "1.2", &CompOp::Gt),
        case("1.2", "1.2.3.4", &CompOp::Lt),
        case("1.2.3.4", "2", &CompOp::Lt),
        case("2", "1.2.3.4", &CompOp::Gt),
        case("123", "123", &CompOp::Eq),
        case("123", "1.2.3", &CompOp::Gt),
        case("1.2.3", "123", &CompOp::Lt),
        case("1.1.2", "1.1.30-dev", &CompOp::Lt),
        case("1.2.3", "1.2.3.alpha", &CompOp::Gt),
        case("1.2.3", "1.2.3-dev", &CompOp::Gt),
        case("1.2.3.dev", "1.2.3.alpha", &CompOp::Eq),
        case("1.2.3-dev", "1.2.3-alpha", &CompOp::Eq),
        case("1.2.3.dev.1", "1.2.3.alpha", &CompOp::Gt),
        case("1.2.3-dev-1", "1.2.3-alpha", &CompOp::Gt),
        case("version-compare 3.2.0 / build 0932", "3.2.5", &CompOp::Lt),
        case("version-compare 3.2.0 / build 0932", "3.1.1", &CompOp::Gt),
        case(
            "version-compare 1.4.1 / build 0043",
            "version-compare 1.4.1 / build 0043",
            &CompOp::Eq,
        ),
        case(
            "version-compare 1.4.1 / build 0042",
            "version-compare 1.4.1 / build 0043",
            &CompOp::Lt,
        ),
        // TODO: inspect these cases
        case("snapshot.1.2.3", "1.2.3.alpha", &CompOp::Lt),
        case("snapshot-1.2.3", "1.2.3-alpha", &CompOp::Lt),
        )]
        $it
    };
}

/// List of invalid version sets for dynamic tests
macro_rules! parametrize_errors_set {
    ( $it:item ) => {
        #[rstest_parametrize(a,
        b,
        operator,
        case("1.2.3", "1.2.3", &CompOp::Lt),
        case("1.2", "1.2.0.0", &CompOp::Ne),
        case("1.2.3.dev", "dev", &CompOp::Eq),
        case("snapshot", "1", &CompOp::Lt),
        )]
        $it
    };
}