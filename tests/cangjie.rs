mod support;

// ============================================================================
// Runnable Tests
// ============================================================================

#[test]
fn runnables_main_example() {
    support::assert_query_snapshot(
        "runnables_main",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/runnables.scm",
    );
}

#[test]
fn runnables_test_example() {
    support::assert_query_snapshot(
        "runnables_test",
        "tests/languages/cangjie/test_example.cj",
        "languages/cangjie/runnables.scm",
    );
}

// ============================================================================
// Outline Tests
// ============================================================================

#[test]
fn outline_example() {
    support::assert_query_snapshot(
        "outline",
        "tests/languages/cangjie/outline_example.cj",
        "languages/cangjie/outline.scm",
    );
}

// ============================================================================
// Textobjects Tests
// ============================================================================

#[test]
fn textobjects() {
    support::assert_query_snapshot(
        "textobjects",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/textobjects.scm",
    );
}

// ============================================================================
// Highlights Tests
// ============================================================================

#[test]
fn highlights() {
    support::assert_query_snapshot(
        "highlights",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/highlights.scm",
    );
}

// ============================================================================
// Folds Tests
// ============================================================================

#[test]
fn folds() {
    support::assert_query_snapshot(
        "folds",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/folds.scm",
    );
}

// ============================================================================
// Indents Tests
// ============================================================================

#[test]
fn indents() {
    support::assert_query_snapshot(
        "indents",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/indents.scm",
    );
}

// ============================================================================
// Locals Tests
// ============================================================================

#[test]
fn locals() {
    support::assert_query_snapshot(
        "locals",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/locals.scm",
    );
}

// ============================================================================
// Brackets Tests
// ============================================================================

#[test]
fn brackets() {
    support::assert_query_snapshot(
        "brackets",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/brackets.scm",
    );
}

// ============================================================================
// Overrides Tests
// ============================================================================

#[test]
fn overrides() {
    support::assert_query_snapshot(
        "overrides",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/overrides.scm",
    );
}

// ============================================================================
// Injections Tests
// ============================================================================

#[test]
fn injections() {
    support::assert_query_snapshot(
        "injections",
        "tests/languages/cangjie/main_example.cj",
        "languages/cangjie/injections.scm",
    );
}
