// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::BTreeSet;

use pretty_assertions::assert_eq;
use stack_graphs::graph::StackGraph;
use stack_graphs::partial::PartialPaths;

use crate::test_graphs;

fn check_partial_paths_in_file(graph: &StackGraph, file: &str, expected_paths: &[&str]) {
    let file = graph.get_file_unchecked(file);
    let mut partials = PartialPaths::new();
    let mut results = BTreeSet::new();
    partials.find_all_partial_paths_in_file(graph, file, |graph, partials, path| {
        if !path.is_complete_as_possible(graph) {
            return;
        }
        if !path.is_productive(partials) {
            return;
        }
        results.insert(path.display(graph, partials).to_string());
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
    check_partial_paths_in_file(
        &graph,
        "main.py",
        &[
            // definition of `__main__` module
            "<__main__,%1> ($1) [root] -> [main.py(0) definition __main__] <%1> ($1)",
            // reference to `a` in import statement
            "<%1> () [main.py(17) reference a] -> [root] <a,%1> ()",
            // `from a import *` means we can rewrite any lookup of `__main__.*` → `a.*`
            "<__main__.,%1> ($1) [root] -> [root] <a.,%1> ($1)",
            // reference to `b` in import statement
            "<%1> () [main.py(15) reference b] -> [root] <b,%1> ()",
            // `from b import *` means we can rewrite any lookup of `__main__.*` → `b.*`
            "<__main__.,%1> ($1) [root] -> [root] <b.,%1> ($1)",
            // we can look for every reference in either `a` or `b`
            "<%1> () [main.py(9) reference A] -> [root] <a.A,%1> ()",
            "<%1> () [main.py(9) reference A] -> [root] <b.A,%1> ()",
            "<%1> () [main.py(10) reference bar] -> [root] <a.foo()/([main.py(7)]).bar,%1> ()",
            "<%1> () [main.py(10) reference bar] -> [root] <b.foo()/([main.py(7)]).bar,%1> ()",
            "<%1> () [main.py(13) reference foo] -> [root] <a.foo,%1> ()",
            "<%1> () [main.py(13) reference foo] -> [root] <b.foo,%1> ()",
            // parameter 0 of function call is `A`, which we can look up in either `a` or `b`
            "<0,%1> ($1) [main.py(7) exported scope] -> [root] <a.A,%1> ($1)",
            "<0,%1> ($1) [main.py(7) exported scope] -> [root] <b.A,%1> ($1)",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "a.py",
        &[
            // definition of `a` module
            "<a,%1> ($1) [root] -> [a.py(0) definition a] <%1> ($1)",
            // definition of `foo` function
            "<a.foo,%1> ($1) [root] -> [a.py(5) definition foo] <%1> ($1)",
            // reference to `x` in function body can resolve to formal parameter
            "<%1> () [a.py(8) reference x] -> [a.py(14) definition x] <%1> ()",
            // result of function is `x`, which is passed in as a formal parameter...
            "<a.foo()/($2),%1> ($1) [root] -> [a.py(14) definition x] <%1> ()",
            // ...which we can look up either the 0th actual positional parameter...
            "<a.foo()/($2),%1> ($1) [root] -> [jump to scope] <0,%1> ($2)",
            // ...or the actual named parameter `x`
            "<a.foo()/($2),%1> ($1) [root] -> [jump to scope] <x,%1> ($2)",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "b.py",
        &[
            // definition of `b` module
            "<b,%1> ($1) [root] -> [b.py(0) definition b] <%1> ($1)",
            // definition of class `A`
            "<b.A,%1> ($1) [root] -> [b.py(5) definition A] <%1> ($1)",
            // definition of class member `A.bar`
            "<b.A.bar,%1> ($1) [root] -> [b.py(8) definition bar] <%1> ($1)",
            // `bar` can also be accessed as an instance member
            "<b.A()/($2).bar,%1> ($1) [root] -> [b.py(8) definition bar] <%1> ($2)",
        ],
    );
}

#[test]
fn chained_methods_python() {
    let graph = test_graphs::chained_methods_python::new();
    check_partial_paths_in_file(
        &graph,
        "main.py",
        // NOTE: Because everything in this example is local to one file, there aren't any partial
        // paths involving the root node.
        &[
            //f
            "<__main__,%1> ($1) [root] -> [main.py(0) definition __main__] <%1> ($1)",

            "<__main__.Builder,%1> ($1) [root] -> [main.py(100) definition Builder] <%1> ($1)",

            "<__main__.Builder.set_a,%1> ($1) [root] -> [main.py(120) definition set_a] <%1> ($1)",
            "<__main__.Builder.set_b,%1> ($1) [root] -> [main.py(130) definition set_b] <%1> ($1)",
            "<__main__.Builder.set_c,%1> ($1) [root] -> [main.py(140) definition set_c] <%1> ($1)",
            "<__main__.Builder.set_d,%1> ($1) [root] -> [main.py(150) definition set_d] <%1> ($1)",
            "<__main__.Builder.set_e,%1> ($1) [root] -> [main.py(160) definition set_e] <%1> ($1)",

            "<%1> ($1) [main.py(122) exported scope] -> [main.py(125) definition self] <%1> ($1)",
            "<%1> ($1) [main.py(132) exported scope] -> [main.py(135) definition self] <%1> ($1)",
            "<%1> ($1) [main.py(142) exported scope] -> [main.py(145) definition self] <%1> ($1)",
            "<%1> ($1) [main.py(152) exported scope] -> [main.py(155) definition self] <%1> ($1)",
            "<%1> ($1) [main.py(162) exported scope] -> [main.py(165) definition self] <%1> ($1)",

            "<%1> () [main.py(123) reference self] -> [main.py(125) definition self] <%1> ()",
            "<%1> () [main.py(133) reference self] -> [main.py(135) definition self] <%1> ()",
            "<%1> () [main.py(143) reference self] -> [main.py(145) definition self] <%1> ()",
            "<%1> () [main.py(153) reference self] -> [main.py(155) definition self] <%1> ()",
            "<%1> () [main.py(163) reference self] -> [main.py(165) definition self] <%1> ()",

            // All of the references in our call chain at the end refer to the correct definitions.
            "<%1> () [main.py(261) reference Builder] -> [main.py(100) definition Builder] <%1> ()",
            "<%1> () [main.py(211) reference set_e] -> [main.py(160) definition set_e] <%1> ()",
            "<%1> () [main.py(221) reference set_d] -> [main.py(150) definition set_d] <%1> ()",
            "<%1> () [main.py(231) reference set_c] -> [main.py(140) definition set_c] <%1> ()",
            "<%1> () [main.py(241) reference set_b] -> [main.py(130) definition set_b] <%1> ()",
            "<%1> () [main.py(251) reference set_a] -> [main.py(120) definition set_a] <%1> ()",

            "<.set_a,%1> ($1) [main.py(122) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<.set_a,%1> ($1) [main.py(132) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<.set_a,%1> ($1) [main.py(142) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<.set_a,%1> ($1) [main.py(152) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<.set_a,%1> ($1) [main.py(162) exported scope] -> [main.py(120) definition set_a] <%1> ()",

            "<.set_b,%1> ($1) [main.py(122) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<.set_b,%1> ($1) [main.py(132) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<.set_b,%1> ($1) [main.py(142) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<.set_b,%1> ($1) [main.py(152) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<.set_b,%1> ($1) [main.py(162) exported scope] -> [main.py(130) definition set_b] <%1> ()",

            //"<.set_c()/($2),%1> ($1) [main.py(122) exported scope] -> [main.py(145) definition self] <%1> ($2)",
            //"<.set_c()/($2),%1> ($1) [main.py(132) exported scope] -> [main.py(145) definition self] <%1> ($2)",
            //"<.set_c()/($2),%1> ($1) [main.py(142) exported scope] -> [main.py(145) definition self] <%1> ($2)",
            //"<.set_c()/($2),%1> ($1) [main.py(152) exported scope] -> [main.py(145) definition self] <%1> ($2)",
            //"<.set_c()/($2),%1> ($1) [main.py(162) exported scope] -> [main.py(145) definition self] <%1> ($2)",

            "<.set_c,%1> ($1) [main.py(122) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<.set_c,%1> ($1) [main.py(132) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<.set_c,%1> ($1) [main.py(142) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<.set_c,%1> ($1) [main.py(152) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<.set_c,%1> ($1) [main.py(162) exported scope] -> [main.py(140) definition set_c] <%1> ()",

            "<.set_d()/($2),%1> ($1) [main.py(122) exported scope] -> [main.py(155) definition self] <%1> ($2)",
            "<.set_d()/($2),%1> ($1) [main.py(132) exported scope] -> [main.py(155) definition self] <%1> ($2)",
            "<.set_d()/($2),%1> ($1) [main.py(142) exported scope] -> [main.py(155) definition self] <%1> ($2)",
            "<.set_d()/($2),%1> ($1) [main.py(152) exported scope] -> [main.py(155) definition self] <%1> ($2)",
            "<.set_d()/($2),%1> ($1) [main.py(162) exported scope] -> [main.py(155) definition self] <%1> ($2)",

            "<.set_d,%1> ($1) [main.py(122) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<.set_d,%1> ($1) [main.py(132) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<.set_d,%1> ($1) [main.py(142) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<.set_d,%1> ($1) [main.py(152) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<.set_d,%1> ($1) [main.py(162) exported scope] -> [main.py(150) definition set_d] <%1> ()",

            "<.set_e()/($2),%1> ($1) [main.py(122) exported scope] -> [main.py(165) definition self] <%1> ($2)",
            "<.set_e()/($2),%1> ($1) [main.py(132) exported scope] -> [main.py(165) definition self] <%1> ($2)",
            "<.set_e()/($2),%1> ($1) [main.py(142) exported scope] -> [main.py(165) definition self] <%1> ($2)",
            "<.set_e()/($2),%1> ($1) [main.py(152) exported scope] -> [main.py(165) definition self] <%1> ($2)",
            "<.set_e()/($2),%1> ($1) [main.py(162) exported scope] -> [main.py(165) definition self] <%1> ($2)",

            "<.set_e,%1> ($1) [main.py(122) exported scope] -> [main.py(160) definition set_e] <%1> ()",
            "<.set_e,%1> ($1) [main.py(132) exported scope] -> [main.py(160) definition set_e] <%1> ()",
            "<.set_e,%1> ($1) [main.py(142) exported scope] -> [main.py(160) definition set_e] <%1> ()",
            "<.set_e,%1> ($1) [main.py(152) exported scope] -> [main.py(160) definition set_e] <%1> ()",
            "<.set_e,%1> ($1) [main.py(162) exported scope] -> [main.py(160) definition set_e] <%1> ()",

            "<__main__.Builder()/($2).set_a,%1> ($1) [root] -> [main.py(120) definition set_a] <%1> ()",
            "<__main__.Builder()/($2).set_b,%1> ($1) [root] -> [main.py(130) definition set_b] <%1> ()",
            "<__main__.Builder()/($2).set_c,%1> ($1) [root] -> [main.py(140) definition set_c] <%1> ()",
            "<__main__.Builder()/($2).set_d,%1> ($1) [root] -> [main.py(150) definition set_d] <%1> ()",
            "<__main__.Builder()/($2).set_e,%1> ($1) [root] -> [main.py(160) definition set_e] <%1> ()",

            //"<__main__.Builder()/($2).set_c()/($3),%1> ($1) [root] -> [main.py(145) definition self] <%1> ($3)",
            "<__main__.Builder()/($2).set_d()/($3),%1> ($1) [root] -> [main.py(155) definition self] <%1> ($3)",
            "<__main__.Builder()/($2).set_e()/($3),%1> ($1) [root] -> [main.py(165) definition self] <%1> ($3)",

            "<__main__.Builder.set_a()/($2),%1> ($1) [root] -> [main.py(125) definition self] <%1> ($2)",
            "<__main__.Builder.set_b()/($2),%1> ($1) [root] -> [main.py(135) definition self] <%1> ($2)",
            "<__main__.Builder.set_c()/($2),%1> ($1) [root] -> [main.py(145) definition self] <%1> ($2)",
            "<__main__.Builder.set_d()/($2),%1> ($1) [root] -> [main.py(155) definition self] <%1> ($2)",
            "<__main__.Builder.set_e()/($2),%1> ($1) [root] -> [main.py(165) definition self] <%1> ($2)",

            "<set_a,%1> ($1) [main.py(112) exported scope] -> [main.py(120) definition set_a] <%1> ($1)",
            "<set_b,%1> ($1) [main.py(112) exported scope] -> [main.py(130) definition set_b] <%1> ($1)",
            "<set_c,%1> ($1) [main.py(112) exported scope] -> [main.py(140) definition set_c] <%1> ($1)",
            "<set_d,%1> ($1) [main.py(112) exported scope] -> [main.py(150) definition set_d] <%1> ($1)",
            "<set_e,%1> ($1) [main.py(112) exported scope] -> [main.py(160) definition set_e] <%1> ($1)",

            "<set_a()/($2),%1> ($1) [main.py(112) exported scope] -> [main.py(125) definition self] <%1> ($2)",
            "<set_b()/($2),%1> ($1) [main.py(112) exported scope] -> [main.py(135) definition self] <%1> ($2)",
            "<set_c()/($2),%1> ($1) [main.py(112) exported scope] -> [main.py(145) definition self] <%1> ($2)",
            "<set_d()/($2),%1> ($1) [main.py(112) exported scope] -> [main.py(155) definition self] <%1> ($2)",
            "<set_e()/($2),%1> ($1) [main.py(112) exported scope] -> [main.py(165) definition self] <%1> ($2)",

            //"<set_b()/($2).set_a,%1> ($1) [main.py(112) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            //"<set_b()/($2).set_b,%1> ($1) [main.py(112) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            //"<set_b()/($2).set_c,%1> ($1) [main.py(112) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            //"<set_b()/($2).set_d,%1> ($1) [main.py(112) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            //"<set_b()/($2).set_e,%1> ($1) [main.py(112) exported scope] -> [main.py(160) definition set_e] <%1> ()",

            "<set_c()/($2).set_a,%1> ($1) [main.py(112) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<set_c()/($2).set_b,%1> ($1) [main.py(112) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<set_c()/($2).set_c,%1> ($1) [main.py(112) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<set_c()/($2).set_d,%1> ($1) [main.py(112) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<set_c()/($2).set_e,%1> ($1) [main.py(112) exported scope] -> [main.py(160) definition set_e] <%1> ()",

            "<set_d()/($2).set_a,%1> ($1) [main.py(112) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<set_d()/($2).set_b,%1> ($1) [main.py(112) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<set_d()/($2).set_c,%1> ($1) [main.py(112) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<set_d()/($2).set_d,%1> ($1) [main.py(112) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<set_d()/($2).set_e,%1> ($1) [main.py(112) exported scope] -> [main.py(160) definition set_e] <%1> ()",

            "<set_e()/($2).set_a,%1> ($1) [main.py(112) exported scope] -> [main.py(120) definition set_a] <%1> ()",
            "<set_e()/($2).set_b,%1> ($1) [main.py(112) exported scope] -> [main.py(130) definition set_b] <%1> ()",
            "<set_e()/($2).set_c,%1> ($1) [main.py(112) exported scope] -> [main.py(140) definition set_c] <%1> ()",
            "<set_e()/($2).set_d,%1> ($1) [main.py(112) exported scope] -> [main.py(150) definition set_d] <%1> ()",
            "<set_e()/($2).set_e,%1> ($1) [main.py(112) exported scope] -> [main.py(160) definition set_e] <%1> ()",
        ],
    );
}

#[test]
fn cyclic_imports_python() {
    let graph = test_graphs::cyclic_imports_python::new();
    check_partial_paths_in_file(
        &graph,
        "main.py",
        &[
            // definition of `__main__` module
            "<__main__,%1> ($1) [root] -> [main.py(0) definition __main__] <%1> ($1)",
            // reference to `a` in import statement
            "<%1> () [main.py(8) reference a] -> [root] <a,%1> ()",
            // `from a import *` means we can rewrite any lookup of `__main__.*` → `a.*`
            "<__main__.,%1> ($1) [root] -> [root] <a.,%1> ($1)",
            // reference to `foo` becomes `a.foo` because of import statement
            "<%1> () [main.py(6) reference foo] -> [root] <a.foo,%1> ()",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "a.py",
        &[
            // definition of `a` module
            "<a,%1> ($1) [root] -> [a.py(0) definition a] <%1> ($1)",
            // reference to `b` in import statement
            "<%1> () [a.py(6) reference b] -> [root] <b,%1> ()",
            // `from b import *` means we can rewrite any lookup of `a.*` → `b.*`
            "<a.,%1> ($1) [root] -> [root] <b.,%1> ($1)",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "b.py",
        &[
            // definition of `b` module
            "<b,%1> ($1) [root] -> [b.py(0) definition b] <%1> ($1)",
            // reference to `a` in import statement
            "<%1> () [b.py(8) reference a] -> [root] <a,%1> ()",
            // `from a import *` means we can rewrite any lookup of `b.*` → `a.*`
            "<b.,%1> ($1) [root] -> [root] <a.,%1> ($1)",
            // definition of `foo`
            "<b.foo,%1> ($1) [root] -> [b.py(6) definition foo] <%1> ($1)",
        ],
    );
}

#[test]
fn cyclic_imports_rust() {
    let graph = test_graphs::cyclic_imports_rust::new();
    check_partial_paths_in_file(
        &graph,
        "test.rs",
        // NOTE: Because everything in this example is local to one file, there aren't any partial
        // paths involving the root node.
        &[
            // reference to `a` in `main` function
            "<%1> () [test.rs(103) reference a] -> [test.rs(201) definition a] <%1> ()",
            // reference to `a` in `b` function
            "<%1> () [test.rs(307) reference a] -> [test.rs(201) definition a] <%1> ()",
            // reference to `b` in `a` function
            "<%1> () [test.rs(206) reference b] -> [test.rs(301) definition b] <%1> ()",
            // reference to `FOO` in `main` can resolve either to `a::BAR` or `b::FOO`
            "<%1> () [test.rs(101) reference FOO] -> [test.rs(204) definition BAR] <%1> ()",
            "<%1> () [test.rs(101) reference FOO] -> [test.rs(304) definition FOO] <%1> ()",
            // reference to `BAR` in `b` resolves _only_ to `a::BAR`
            "<%1> () [test.rs(305) reference BAR] -> [test.rs(204) definition BAR] <%1> ()",
        ],
    );
}

#[test]
fn sequenced_import_star() {
    let graph = test_graphs::sequenced_import_star::new();
    check_partial_paths_in_file(
        &graph,
        "main.py",
        &[
            // definition of `__main__` module
            "<__main__,%1> ($1) [root] -> [main.py(0) definition __main__] <%1> ($1)",
            // reference to `a` in import statement
            "<%1> () [main.py(8) reference a] -> [root] <a,%1> ()",
            // `from a import *` means we can rewrite any lookup of `__main__.*` → `a.*`
            "<__main__.,%1> ($1) [root] -> [root] <a.,%1> ($1)",
            // reference to `foo` becomes `a.foo` because of import statement
            "<%1> () [main.py(6) reference foo] -> [root] <a.foo,%1> ()",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "a.py",
        &[
            // definition of `a` module
            "<a,%1> ($1) [root] -> [a.py(0) definition a] <%1> ($1)",
            // reference to `b` in import statement
            "<%1> () [a.py(6) reference b] -> [root] <b,%1> ()",
            // `from b import *` means we can rewrite any lookup of `a.*` → `b.*`
            "<a.,%1> ($1) [root] -> [root] <b.,%1> ($1)",
        ],
    );
    check_partial_paths_in_file(
        &graph,
        "b.py",
        &[
            // definition of `b` module
            "<b,%1> ($1) [root] -> [b.py(0) definition b] <%1> ($1)",
            // definition of `foo` inside of `b` module
            "<b.foo,%1> ($1) [root] -> [b.py(5) definition foo] <%1> ($1)",
        ],
    );
}
