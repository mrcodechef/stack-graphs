// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

//! Detect and avoid cycles in our path-finding algorithm.
//!
//! Cycles in a stack graph can indicate many things.  Your language might allow mutually recursive
//! imports.  If you are modeling dataflow through function calls, then any recursion in your
//! function calls will lead to cycles in your stack graph.  And if you have any control-flow paths
//! that lead to infinite loops at runtime, we'll probably discover those as stack graph paths
//! during the path-finding algorithm.
//!
//! (Note that we're only considering cycles in well-formed paths.  For instance, _pop symbol_
//! nodes are "guards" that don't allow you to progress into a node if the top of the symbol stack
//! doesn't match.  We don't consider that a valid path, and so we don't have to worry about
//! whether it contains any cycles.)
//!
//! This module implements a cycle detector that lets us detect these situations and "cut off"
//! these paths, not trying to extend them any further.  Note that any cycle detection logic we
//! implement will be a heuristic.  In particular, since our path-finding algorithm will mimic any
//! runtime recursion, a "complete" cycle detection logic would be equivalent to the Halting
//! Problem.
//!
//! Right now, we implement a simple heuristic where we limit the number of distinct paths that we
//! process that have the same start and end nodes.  We do not make any guarantees that we will
//! always use this particular heuristic, however!  We reserve the right to change the heuristic at
//! any time.

use std::collections::HashMap;

use smallvec::SmallVec;

use crate::arena::Handle;
use crate::graph::Node;
use crate::graph::Symbol;
use crate::partial::PartialPath;
use crate::partial::PartialPaths;
use crate::paths::Path;
use crate::paths::Paths;

/// Helps detect cycles in the path-finding algorithm.
pub struct CycleDetector<P> {
    paths: HashMap<PathKey, SmallVec<[P; 8]>>,
}

#[doc(hidden)]
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PathKey {
    start_node: Handle<Node>,
    start_symbol_stack_head: Option<Handle<Symbol>>,
    start_scope_stack_head: Option<Handle<Node>>,
    end_node: Handle<Node>,
    end_symbol_stack_head: Option<Handle<Symbol>>,
    end_scope_stack_head: Option<Handle<Node>>,
}

#[doc(hidden)]
pub trait HasPathKey: Clone {
    fn is_shorter_than(&self, other: &Self) -> bool;
}

impl Path {
    pub(crate) fn path_key(&self, paths: &Paths) -> PathKey {
        let mut symbol_stack = self.symbol_stack;
        let end_symbol_stack_head = symbol_stack.pop_front(paths).map(|symbol| symbol.symbol);
        let mut scope_stack = self.scope_stack;
        let end_scope_stack_head = scope_stack.pop_front(paths);
        PathKey {
            start_node: self.start_node,
            start_symbol_stack_head: None,
            start_scope_stack_head: None,
            end_node: self.end_node,
            end_symbol_stack_head,
            end_scope_stack_head,
        }
    }
}

impl HasPathKey for Path {
    fn is_shorter_than(&self, other: &Self) -> bool {
        self.edges.len() < other.edges.len() && self.symbol_stack.len() <= other.symbol_stack.len()
    }
}

impl PartialPath {
    pub(crate) fn path_key(&self, partials: &mut PartialPaths) -> PathKey {
        let mut symbol_stack_precondition = self.symbol_stack_precondition;
        let start_symbol_stack_head = symbol_stack_precondition
            .pop_front(partials)
            .map(|symbol| symbol.symbol);
        let mut scope_stack_precondition = self.scope_stack_precondition;
        let start_scope_stack_head = scope_stack_precondition.pop_front(partials);
        let mut symbol_stack_postcondition = self.symbol_stack_postcondition;
        let end_symbol_stack_head = symbol_stack_postcondition
            .pop_front(partials)
            .map(|symbol| symbol.symbol);
        let mut scope_stack_postcondition = self.scope_stack_postcondition;
        let end_scope_stack_head = scope_stack_postcondition.pop_front(partials);
        PathKey {
            start_node: self.start_node,
            start_symbol_stack_head,
            start_scope_stack_head,
            end_node: self.end_node,
            end_symbol_stack_head,
            end_scope_stack_head,
        }
    }
}

impl HasPathKey for PartialPath {
    fn is_shorter_than(&self, other: &Self) -> bool {
        self.edges.len() < other.edges.len()
            && (self.symbol_stack_precondition.len() + self.symbol_stack_postcondition.len())
                <= (other.symbol_stack_precondition.len() + other.symbol_stack_postcondition.len())
    }
}

const MAX_SIMILAR_PATH_COUNT: usize = 4;

impl<P> CycleDetector<P>
where
    P: HasPathKey,
{
    /// Creates a new, empty cycle detector.
    pub fn new() -> CycleDetector<P> {
        CycleDetector {
            paths: HashMap::new(),
        }
    }

    /// Determines whether we should process this path during the path-finding algorithm.  If our
    /// heuristics decide that this path is a duplicate, or is "non-productive", then we return
    /// `false`, and the path-finding algorithm will skip this path.
    pub fn should_process_path<F>(&mut self, key: PathKey, path: &P, cmp: F) -> bool
    where
        F: FnMut(&P) -> std::cmp::Ordering,
    {
        let paths_with_same_nodes = self.paths.entry(key).or_default();
        let index = match paths_with_same_nodes.binary_search_by(cmp) {
            // We've already seen this exact path before; no need to process it again.
            Ok(_) => return false,
            // Otherwise add it to the list.
            Err(index) => index,
        };

        // Count how many paths we've already processed that have the same endpoints and are
        // "shorter".
        let similar_path_count = paths_with_same_nodes
            .iter()
            .filter(|similar_path| similar_path.is_shorter_than(path))
            .count();
        if similar_path_count > MAX_SIMILAR_PATH_COUNT {
            return false;
        }

        paths_with_same_nodes.insert(index, path.clone());
        true
    }
}
