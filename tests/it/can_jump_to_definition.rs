// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::BTreeSet;

use pretty_assertions::assert_eq;
use stack_graphs::graph::StackGraph;
use stack_graphs::paths::Paths;

use crate::test_graphs;

fn check_jump_to_definition(graph: &StackGraph, expected_paths: &[&str]) {
    let mut paths = Paths::new();
    let mut results = BTreeSet::new();
    let references = graph
        .iter_nodes()
        .filter(|handle| graph[*handle].is_reference());
    paths.find_all_paths(graph, references, |graph, paths, path| {
        if path.is_complete(graph) {
            results.insert(path.display(graph, paths).to_string());
        }
    });
    let expected_paths = expected_paths
        .iter()
        .map(|s| s.to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(expected_paths, results);
}

#[test]
fn class_field_through_function_parameter() {
    let graph = test_graphs::class_field_through_function_parameter::new();
    check_jump_to_definition(
        &graph,
        &[
            // reference to `a` in import statement
            "[main.py(17) reference a] -> [a.py(0) definition a]",
            // reference to `b` in import statement
            "[main.py(15) reference b] -> [b.py(0) definition b]",
            // reference to `foo` in function call resolves to function definition
            "[main.py(13) reference foo] -> [a.py(5) definition foo]",
            // reference to `A` as function parameter resolves to class definition
            "[main.py(9) reference A] -> [b.py(5) definition A]",
            // reference to `bar` on result flows through body of `foo` to find `A.bar`
            "[main.py(10) reference bar] -> [b.py(8) definition bar]",
            // reference to `x` in function body resolves to formal parameter
            "[a.py(8) reference x] -> [a.py(14) definition x]",
        ],
    );
}

#[test]
fn chained_methods_python() {
    let graph = test_graphs::chained_methods_python::new();
    check_jump_to_definition(
        &graph,
        &[
            "[main.py(123) reference self] -> [main.py(125) definition self]",
            "[main.py(133) reference self] -> [main.py(135) definition self]",
            "[main.py(143) reference self] -> [main.py(145) definition self]",
            "[main.py(153) reference self] -> [main.py(155) definition self]",
            "[main.py(163) reference self] -> [main.py(165) definition self]",
            "[main.py(261) reference Builder] -> [main.py(100) definition Builder]",
            "[main.py(251) reference set_a] -> [main.py(120) definition set_a]",
            "[main.py(241) reference set_b] -> [main.py(130) definition set_b]",
            "[main.py(231) reference set_c] -> [main.py(140) definition set_c]",
            "[main.py(221) reference set_d] -> [main.py(150) definition set_d]",
            "[main.py(211) reference set_e] -> [main.py(160) definition set_e]",
        ],
    );
}

#[test]
fn cyclic_imports_python() {
    let graph = test_graphs::cyclic_imports_python::new();
    check_jump_to_definition(
        &graph,
        &[
            // reference to `a` in import statement
            "[main.py(8) reference a] -> [a.py(0) definition a]",
            // reference to `foo` resolves through intermediate file to find `b.foo`
            "[main.py(6) reference foo] -> [b.py(6) definition foo]",
            // reference to `b` in import statement
            "[a.py(6) reference b] -> [b.py(0) definition b]",
            // reference to `a` in import statement
            "[b.py(8) reference a] -> [a.py(0) definition a]",
        ],
    );
}

#[test]
fn cyclic_imports_rust() {
    let graph = test_graphs::cyclic_imports_rust::new();
    check_jump_to_definition(
        &graph,
        &[
            // reference to `a` in `a::FOO` resolves to module definition
            "[test.rs(103) reference a] -> [test.rs(201) definition a]",
            // reference to `a::FOO` in `main` can resolve either to `a::BAR` or `b::FOO`
            "[test.rs(101) reference FOO] -> [test.rs(304) definition FOO]",
            "[test.rs(101) reference FOO] -> [test.rs(204) definition BAR]",
            // reference to `b` in use statement resolves to module definition
            "[test.rs(206) reference b] -> [test.rs(301) definition b]",
            // reference to `a` in use statement resolves to module definition
            "[test.rs(307) reference a] -> [test.rs(201) definition a]",
            // reference to `BAR` in module `b` can _only_ resolve to `a::BAR`
            "[test.rs(305) reference BAR] -> [test.rs(204) definition BAR]",
        ],
    );
}

#[test]
fn sequenced_import_star() {
    let graph = test_graphs::sequenced_import_star::new();
    check_jump_to_definition(
        &graph,
        &[
            // reference to `a` in import statement
            "[main.py(8) reference a] -> [a.py(0) definition a]",
            // reference to `foo` resolves through intermediate file to find `b.foo`
            "[main.py(6) reference foo] -> [b.py(5) definition foo]",
            // reference to `b` in import statement
            "[a.py(6) reference b] -> [b.py(0) definition b]",
        ],
    );
}
