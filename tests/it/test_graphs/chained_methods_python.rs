// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use crate::test_graphs::CreateStackGraph;

/// A stack graph containing:
///
/// ``` python
/// class Builder:
///     def set_a(self):
///         return self
///
///     def set_b(self):
///         return self
///
///     def set_c(self):
///         return self
///
///     def set_d(self):
///         return self
///
///     def set_e(self):
///         return self
///
/// Builder().set_a().set_b().set_c().set_d().set_e()
/// ```
#[allow(non_snake_case)]
pub fn new<T>() -> T
where
    T: CreateStackGraph + Default,
{
    let mut graph = T::default();
    let root = graph.root_node();
    let sym_call = graph.symbol("()");
    let sym_dot = graph.symbol(".");
    let sym_main = graph.symbol("__main__");
    let sym_self = graph.symbol("self");
    let sym_Builder = graph.symbol("Builder");
    let sym_set_a = graph.symbol("set_a");
    let sym_set_b = graph.symbol("set_b");
    let sym_set_c = graph.symbol("set_c");
    let sym_set_d = graph.symbol("set_d");
    let sym_set_e = graph.symbol("set_e");

    let main_file = graph.file("main.py");
    let main = graph.definition(main_file, 0, sym_main);
    let main_dot_1 = graph.pop_symbol(main_file, 1, sym_dot);
    let main_bottom_2 = graph.internal_scope(main_file, 2);
    let main_3 = graph.internal_scope(main_file, 3);
    let main_4 = graph.internal_scope(main_file, 4);
    let main_top_5 = graph.internal_scope(main_file, 5);
    graph.edge(root, main);
    graph.edge(main, main_dot_1);
    graph.edge(main_dot_1, main_bottom_2);
    graph.edge(main_bottom_2, main_3);
    graph.edge(main_3, main_4);
    graph.edge(main_4, main_top_5);

    let main_Builder = graph.definition(main_file, 100, sym_Builder);
    let Builder_dot_101 = graph.pop_symbol(main_file, 101, sym_dot);
    let Builder_class_members_bottom_102 = graph.internal_scope(main_file, 102);
    let Builder_class_members_103 = graph.internal_scope(main_file, 103);
    let Builder_class_members_104 = graph.internal_scope(main_file, 104);
    let Builder_class_members_105 = graph.internal_scope(main_file, 105);
    let Builder_class_members_106 = graph.internal_scope(main_file, 106);
    let Builder_class_members_107 = graph.internal_scope(main_file, 107);
    let Builder_class_members_top_108 = graph.internal_scope(main_file, 108);
    graph.edge(main_4, main_Builder);
    graph.edge(main_Builder, Builder_dot_101);
    graph.edge(Builder_dot_101, Builder_class_members_bottom_102);
    graph.edge(Builder_class_members_bottom_102, Builder_class_members_103);
    graph.edge(Builder_class_members_103, Builder_class_members_104);
    graph.edge(Builder_class_members_104, Builder_class_members_105);
    graph.edge(Builder_class_members_105, Builder_class_members_106);
    graph.edge(Builder_class_members_106, Builder_class_members_107);
    graph.edge(Builder_class_members_107, Builder_class_members_top_108);

    let Builder_constructor = graph.pop_scoped_symbol(main_file, 109, sym_call);
    let Builder_instance_drop = graph.drop_scopes(main_file, 110);
    let Builder_dot_111 = graph.pop_symbol(main_file, 111, sym_dot);
    let Builder_instance_members = graph.exported_scope(main_file, 112);
    graph.edge(main_Builder, Builder_constructor);
    graph.edge(Builder_constructor, Builder_instance_drop);
    graph.edge(Builder_instance_drop, Builder_dot_111);
    graph.edge(Builder_dot_111, Builder_instance_members);
    graph.edge(Builder_instance_members, Builder_class_members_bottom_102);

    let Builder_instance_dot = graph.push_symbol(main_file, 113, sym_dot);
    let Builder_instance_members = graph.push_symbol(main_file, 114, sym_self);
    graph.edge(Builder_dot_111, Builder_instance_dot);
    graph.edge(Builder_instance_dot, Builder_instance_members);

    let Builder_set_a = graph.definition(main_file, 120, sym_set_a);
    let function_set_a = graph.pop_scoped_symbol(main_file, 121, sym_call);
    let return_value_set_a = graph.exported_scope(main_file, 122);
    let return_self_set_a = graph.reference(main_file, 123, sym_self);
    let formals_set_a = graph.internal_scope(main_file, 124);
    let self_param_set_a = graph.definition(main_file, 125, sym_self);
    let self_link_set_a = graph.pop_symbol(main_file, 126, sym_self);
    graph.edge(Builder_class_members_107, Builder_set_a);
    graph.edge(Builder_set_a, function_set_a);
    graph.edge(function_set_a, return_value_set_a);
    graph.edge(return_value_set_a, return_self_set_a);
    graph.edge(return_self_set_a, formals_set_a);
    graph.edge(formals_set_a, self_param_set_a);
    graph.edge(formals_set_a, self_link_set_a);
    graph.edge(self_link_set_a, Builder_instance_drop);
    graph.edge(Builder_instance_members, formals_set_a);

    let Builder_set_b = graph.definition(main_file, 130, sym_set_b);
    let function_set_b = graph.pop_scoped_symbol(main_file, 131, sym_call);
    let return_value_set_b = graph.exported_scope(main_file, 132);
    let return_self_set_b = graph.reference(main_file, 133, sym_self);
    let formals_set_b = graph.internal_scope(main_file, 134);
    let self_param_set_b = graph.definition(main_file, 135, sym_self);
    let self_link_set_b = graph.pop_symbol(main_file, 136, sym_self);
    graph.edge(Builder_class_members_106, Builder_set_b);
    graph.edge(Builder_set_b, function_set_b);
    graph.edge(function_set_b, return_value_set_b);
    graph.edge(return_value_set_b, return_self_set_b);
    graph.edge(return_self_set_b, formals_set_b);
    graph.edge(formals_set_b, self_param_set_b);
    graph.edge(formals_set_b, self_link_set_b);
    graph.edge(self_link_set_b, Builder_instance_drop);
    graph.edge(Builder_instance_members, formals_set_b);

    let Builder_set_c = graph.definition(main_file, 140, sym_set_c);
    let function_set_c = graph.pop_scoped_symbol(main_file, 141, sym_call);
    let return_value_set_c = graph.exported_scope(main_file, 142);
    let return_self_set_c = graph.reference(main_file, 143, sym_self);
    let formals_set_c = graph.internal_scope(main_file, 144);
    let self_param_set_c = graph.definition(main_file, 145, sym_self);
    let self_link_set_c = graph.pop_symbol(main_file, 146, sym_self);
    graph.edge(Builder_class_members_105, Builder_set_c);
    graph.edge(Builder_set_c, function_set_c);
    graph.edge(function_set_c, return_value_set_c);
    graph.edge(return_value_set_c, return_self_set_c);
    graph.edge(return_self_set_c, formals_set_c);
    graph.edge(formals_set_c, self_param_set_c);
    graph.edge(formals_set_c, self_link_set_c);
    graph.edge(self_link_set_c, Builder_instance_drop);
    graph.edge(Builder_instance_members, formals_set_c);

    let Builder_set_d = graph.definition(main_file, 150, sym_set_d);
    let function_set_d = graph.pop_scoped_symbol(main_file, 151, sym_call);
    let return_value_set_d = graph.exported_scope(main_file, 152);
    let return_self_set_d = graph.reference(main_file, 153, sym_self);
    let formals_set_d = graph.internal_scope(main_file, 154);
    let self_param_set_d = graph.definition(main_file, 155, sym_self);
    let self_link_set_d = graph.pop_symbol(main_file, 156, sym_self);
    graph.edge(Builder_class_members_104, Builder_set_d);
    graph.edge(Builder_set_d, function_set_d);
    graph.edge(function_set_d, return_value_set_d);
    graph.edge(return_value_set_d, return_self_set_d);
    graph.edge(return_self_set_d, formals_set_d);
    graph.edge(formals_set_d, self_param_set_d);
    graph.edge(formals_set_d, self_link_set_d);
    graph.edge(self_link_set_d, Builder_instance_drop);
    graph.edge(Builder_instance_members, formals_set_d);

    let Builder_set_e = graph.definition(main_file, 160, sym_set_e);
    let function_set_e = graph.pop_scoped_symbol(main_file, 161, sym_call);
    let return_value_set_e = graph.exported_scope(main_file, 162);
    let return_self_set_e = graph.reference(main_file, 163, sym_self);
    let formals_set_e = graph.internal_scope(main_file, 164);
    let self_param_set_e = graph.definition(main_file, 165, sym_self);
    let self_link_set_e = graph.pop_symbol(main_file, 166, sym_self);
    graph.edge(Builder_class_members_103, Builder_set_e);
    graph.edge(Builder_set_e, function_set_e);
    graph.edge(function_set_e, return_value_set_e);
    graph.edge(return_value_set_e, return_self_set_e);
    graph.edge(return_self_set_e, formals_set_e);
    graph.edge(formals_set_e, self_param_set_e);
    graph.edge(formals_set_e, self_link_set_e);
    graph.edge(self_param_set_e, Builder_instance_drop);
    graph.edge(self_link_set_e, Builder_instance_drop);
    graph.edge(Builder_instance_members, formals_set_e);

    let _empty_params = graph.exported_scope(main_file, 200);
    let call_set_e = graph.push_scoped_symbol(main_file, 210, sym_call, main_file, 200);
    let ref_set_e = graph.reference(main_file, 211, sym_set_e);
    let dot_set_e = graph.push_symbol(main_file, 212, sym_dot);
    let call_set_d = graph.push_scoped_symbol(main_file, 220, sym_call, main_file, 200);
    let ref_set_d = graph.reference(main_file, 221, sym_set_d);
    let dot_set_d = graph.push_symbol(main_file, 222, sym_dot);
    let call_set_c = graph.push_scoped_symbol(main_file, 230, sym_call, main_file, 200);
    let ref_set_c = graph.reference(main_file, 231, sym_set_c);
    let dot_set_c = graph.push_symbol(main_file, 232, sym_dot);
    let call_set_b = graph.push_scoped_symbol(main_file, 240, sym_call, main_file, 200);
    let ref_set_b = graph.reference(main_file, 241, sym_set_b);
    let dot_set_b = graph.push_symbol(main_file, 242, sym_dot);
    let call_set_a = graph.push_scoped_symbol(main_file, 250, sym_call, main_file, 200);
    let ref_set_a = graph.reference(main_file, 251, sym_set_a);
    let dot_set_a = graph.push_symbol(main_file, 252, sym_dot);
    let call_Builder = graph.push_scoped_symbol(main_file, 260, sym_call, main_file, 200);
    let ref_Builder = graph.reference(main_file, 261, sym_Builder);
    graph.edge(call_set_e, ref_set_e);
    graph.edge(ref_set_e, dot_set_e);
    graph.edge(dot_set_e, call_set_d);
    graph.edge(call_set_d, ref_set_d);
    graph.edge(ref_set_d, dot_set_d);
    graph.edge(dot_set_d, call_set_c);
    graph.edge(call_set_c, ref_set_c);
    graph.edge(ref_set_c, dot_set_c);
    graph.edge(dot_set_c, call_set_b);
    graph.edge(call_set_b, ref_set_b);
    graph.edge(ref_set_b, dot_set_b);
    graph.edge(dot_set_b, call_set_a);
    graph.edge(call_set_a, ref_set_a);
    graph.edge(ref_set_a, dot_set_a);
    graph.edge(dot_set_a, call_Builder);
    graph.edge(call_Builder, ref_Builder);
    graph.edge(ref_Builder, main_3);

    graph
}
