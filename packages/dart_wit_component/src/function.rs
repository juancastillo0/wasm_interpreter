use wit_parser::*;

use crate::{
    generate::add_docs,
    strings::Normalize,
    types::{function_resource, Parsed},
};

pub enum FuncKind {
    Resource(TypeOwner),
    MethodCall,
    Method,
    Field,
}

impl Parsed<'_> {
    pub fn function_import(&self, key: Option<&WorldKey>, id: &str, f: &Function) -> String {
        let interface_name_m = match key {
            Some(k) => self.0.name_world_key(k),
            None => "$root".to_string(),
        };
        let interface_name = key.map(|key| self.world_key_type_name(key));
        let getter = format!(
            "imports.{}{}",
            interface_name
                .map(|v| format!("{}.", v.as_var()))
                .unwrap_or("".to_string()),
            id.as_var(),
        );

        let ft = self.function_spec(f);
        let exec = self.function_exec(&getter, f);
        let exec_name = getter.as_fn_suffix();
        format!(
            "{{
                const ft = {ft};
                {exec}
                final lowered = loweredImportFunction(r'{interface_name_m}#{id}', ft, exec{exec_name}, getLib);
                builder.addImport(r'{interface_name_m}', '{id}', lowered);
            }}",
        )
    }

    pub fn function_spec(&self, function: &Function) -> String {
        let params = function
            .params
            .iter()
            .map(|(name, ty)| format!("('{name}', {})", self.type_to_spec(ty)))
            .collect::<Vec<_>>()
            .join(", ");
        let results = match &function.results {
            Results::Anon(a) => format!("('', {})", self.type_to_spec(a)),
            Results::Named(results) => results
                .iter()
                .map(|(name, ty)| format!("('{name}', {})", self.type_to_spec(ty)))
                .collect::<Vec<_>>()
                .join(", "),
        };
        format!("FuncType([{params}], [{results}])")
    }

    pub fn function_exec(&self, getter: &str, function: &Function) -> String {
        // TODO: post function
        let results_def = if function.results.len() == 0 {
            ""
        } else {
            "final results = "
        };
        let n = getter.as_fn_suffix();
        let args = function
            .params
            .iter()
            .enumerate()
            .map(|(i, (_name, _ty))| format!("final args{i} = args[{i}];"))
            .collect::<String>();
        let args_from_json = function
            .params
            .iter()
            .enumerate()
            .map(|(i, (name, ty))| {
                let value = self.type_from_json(&format!("args{i}"), ty);
                format!("{}: {value}", name.as_var())
            })
            .collect::<Vec<_>>()
            .join(", ");
        let ret = match &function.results {
            Results::Anon(a) => format!("[{}]", self.type_to_wasm("results", a)),
            Results::Named(results) => {
                if results.is_empty() {
                    "const []".to_string()
                } else {
                    let values = results
                        .iter()
                        .map(|(name, ty)| {
                            self.type_to_wasm(&format!("results.{}", name.as_var()), ty)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("[{values}]")
                }
            }
        };
        format!(
            "
        (ListValue, void Function()) exec{n}(ListValue args) {{
        {args}
        {results_def}{getter}({args_from_json});
        return ({ret}, () {{}});
        }}
        "
        )
    }

    pub fn world_key_type_name<'a>(&'a self, key: &'a WorldKey) -> &'a str {
        match key {
            WorldKey::Name(name) => name,
            WorldKey::Interface(id) => {
                let interface = self.0.interfaces.get(*id).unwrap();
                interface.name.as_ref().unwrap()
            }
        }
    }

    pub fn add_interfaces(
        &self,
        mut s: &mut String,
        map: &mut dyn Iterator<Item = (&WorldKey, &WorldItem)>,
        is_export: bool,
        mut func_imports: Option<&mut String>,
    ) {
        map.for_each(|(key, item)| match item {
            WorldItem::Interface(interface_id) => {
                let interface = self.0.interfaces.get(*interface_id).unwrap();
                self.add_interface(&mut s, key, interface, is_export, &mut func_imports)
            }
            _ => {}
        });
    }

    pub fn add_interface(
        &self,
        mut s: &mut String,
        key: &WorldKey,
        interface: &Interface,
        is_export: bool,
        func_imports: &mut Option<&mut String>,
    ) {
        let world_prefix = self.0.name_world_key(key);
        let interface_id = self.world_key_type_name(key);
        let world_name = heck::AsPascalCase(&self.0.worlds.iter().next().unwrap().1.name);
        let name = heck::AsPascalCase(interface_id);
        add_docs(&mut s, &interface.docs);
        if is_export {
            s.push_str(&format!(
                "class {name} {{ final {world_name}World _world; {name}(this._world)"
            ));
            if interface.functions.is_empty() {
                s.push_str(";");
            } else {
                s.push_str(":");
                interface
                    .functions
                    .iter()
                    .enumerate()
                    .for_each(|(index, (id, f))| {
                        let fn_name = if self.2.async_worker {
                            "getComponentFunctionWorker"
                        } else {
                            "getComponentFunction"
                        };
                        s.push_str(&format!(
                            "_{} = _world.library.{fn_name}('{world_prefix}#{id}', const {},)!",
                            id.as_var(),
                            self.function_spec(f)
                        ));
                        if index != interface.functions.len() - 1 {
                            s.push_str(",");
                        }
                    });
                s.push_str(";");
            }

            interface.functions.iter().for_each(|(_id, f)| {
                self.add_function(&mut s, f, FuncKind::MethodCall, false);
            });
            s.push_str("}");
        } else {
            if interface.functions.is_empty() {
                return;
            }
            s.push_str(&format!("abstract class {name}Import {{",));
            interface.functions.iter().for_each(|(id, f)| {
                self.add_function(&mut s, f, FuncKind::Method, false);
                if let Some(func_imports) = func_imports {
                    func_imports.push_str(&self.function_import(Some(key), id, f));
                }
            });
            s.push_str("}");
        }
    }

    pub fn is_option(&self, ty: &Type) -> bool {
        match ty {
            Type::Id(ty_id) => {
                let ty_def = self.0.types.get(*ty_id).unwrap();
                matches!(ty_def.kind, TypeDefKind::Option(_))
            }
            _ => false,
        }
    }

    pub fn type_param(&self, name: &str, ty: &Type, is_constructor: bool) -> String {
        let type_or_this = if is_constructor {
            "this.".to_string()
        } else {
            self.type_to_str(ty)
        };
        if !self.2.required_option && self.is_option(ty) {
            if self.2.use_null_for_option {
                format!("{type_or_this} {},", name.as_var())
            } else {
                format!("{type_or_this} {} = const None(),", name.as_var())
            }
        } else {
            format!("required {type_or_this} {},", name.as_var())
        }
    }

    pub fn is_unit(&self, ty: &Type) -> bool {
        match ty {
            Type::Id(ty_id) => {
                let ty_def = self.0.types.get(*ty_id).unwrap();
                if let TypeDefKind::Tuple(Tuple { types }) = &ty_def.kind {
                    types.is_empty()
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn add_function(&self, mut s: &mut String, f: &Function, kind: FuncKind, from_world: bool) {
        let is_static = match f.kind {
            FunctionKind::Static(_) | FunctionKind::Constructor(_) => true,
            _ => false,
        };
        let is_resource = match kind {
            FuncKind::Resource(_) => true,
            _ => false,
        };
        let mut params = f
            .params
            .iter()
            .filter(|(name, _)| !is_resource || name != "self")
            .map(|(name, ty)| self.type_param(name, ty, false))
            .collect::<String>();
        if params.len() > 0 {
            params = format!("{{{params}}}");
        }

        let mut results = match &f.results {
            Results::Anon(ty) => self.type_to_str(ty),
            Results::Named(list) => {
                if list.is_empty() {
                    "void".to_string()
                } else {
                    let types = list
                        .iter()
                        .map(|(name, ty)| format!("{} {}", self.type_to_str(ty), name.as_var()))
                        .collect::<Vec<_>>()
                        .join(",");
                    format!("({{{types}}})")
                }
            }
        };

        let name = &f.name.as_fn();
        match kind {
            FuncKind::Field => {
                add_docs(&mut s, &f.docs);
                s.push_str(&format!("final {results} Function({params}) {name};",))
            }
            FuncKind::Method => {
                add_docs(&mut s, &f.docs);
                s.push_str(&format!("{results} {name}({params});",))
            }
            FuncKind::Resource(owner) => {
                add_docs(&mut s, &f.docs);
                let async_ = if self.2.async_worker { "async " } else { "" };
                let static_ = if let (FunctionKind::Static(_), _)
                | (FunctionKind::Constructor(_), true) =
                    (&f.kind, self.2.async_worker)
                {
                    "static "
                } else {
                    ""
                };

                let owner_getter = match owner {
                    TypeOwner::World(_) => {
                        if is_static {
                            "world.".to_string()
                        } else {
                            "_world.".to_string()
                        }
                    }
                    TypeOwner::Interface(i) => {
                        if is_static {
                            format!("{}.", self.0.interfaces[i].name.as_ref().unwrap().as_var())
                        } else {
                            format!(
                                "_world.{}.",
                                self.0.interfaces[i].name.as_ref().unwrap().as_var()
                            )
                        }
                    }
                    TypeOwner::None => "".to_string(),
                };
                if is_static {
                    let w_name = heck::AsPascalCase(&self.0.worlds.iter().next().unwrap().1.name);
                    match owner {
                        TypeOwner::Interface(i) => {
                            let n = self.0.interfaces[i].name.as_ref().unwrap();
                            params = format!("{} {}, {params}", n.as_type(), n.as_var());
                        }
                        _ => {
                            params = format!("{w_name}World world, {params}");
                        }
                    }
                }
                if let (FunctionKind::Constructor(_), false) = (&f.kind, self.2.async_worker) {
                    s.push_str(&format!("factory {results}.{name}({params}) {{"));
                } else {
                    let n = if let Some(r) = function_resource(&f.kind) {
                        let ty_name =
                            heck::AsPascalCase(self.0.types[*r].name.as_ref().unwrap()).to_string();
                        let mut m = name.match_indices(&ty_name);
                        (&name.as_str()[m.next().unwrap().0 + ty_name.len()..]).as_fn()
                    } else {
                        name.clone()
                    };
                    s.push_str(&format!("{static_}{results} {n}({params}) {async_}{{"));
                }
                {
                    let args = f
                        .params
                        .iter()
                        .map(|(name, _)| {
                            let g = if !is_static && name == "self" {
                                "this".to_string()
                            } else {
                                name.as_var()
                            };
                            format!("{}: {}", name.as_var(), g,)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    s.push_str(&format!("return {owner_getter}{name}({args});"));
                }
                s.push_str("}");
            }
            FuncKind::MethodCall => {
                // s.push_str(&format!("late final _{} = lookup('{}');", f.name, f.name));
                if self.2.async_worker {
                    results = format!("Future<{results}>");
                    s.push_str(&format!(
                        "final Future<ListValue> Function(ListValue) _{name};"
                    ));
                } else {
                    s.push_str(&format!("final ListValue Function(ListValue) _{name};"));
                }

                add_docs(&mut s, &f.docs);
                let async_ = if self.2.async_worker { "async " } else { "" };
                s.push_str(&format!("{results} {name}({params}) {async_}{{"));
                {
                    let await_ = if self.2.async_worker { "await " } else { "" };
                    let results_assignments = if f.results.len() == 0
                        || (f.results.len() == 1
                            && self.is_unit(f.results.iter_types().next().unwrap()))
                    {
                        ""
                    } else {
                        "final results = "
                    };
                    let params = f
                        .params
                        .iter()
                        .map(|(name, ty)| self.type_to_wasm(&name.as_var(), ty))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let results_with_ctx = match &f.results {
                        Results::Anon(Type::Id(_)) => true,
                        Results::Named(p) => !p.is_empty(),
                        _ => false,
                    };
                    let ret = match &f.results {
                        Results::Anon(a) => {
                            if self.is_unit(&a) {
                                "return ();".to_string()
                            } else {
                                if results_with_ctx {
                                    let parse_prefix = if from_world { "" } else { "_world." };
                                    format!(
                                        "final result = results[0];return {parse_prefix}withContext(() => {});",
                                        self.type_from_json("result", a)
                                    )
                                } else {
                                    format!(
                                        "final result = results[0];return {};",
                                        self.type_from_json("result", a)
                                    )
                                }
                            }
                        }
                        Results::Named(results) => {
                            if results.is_empty() {
                                "".to_string()
                            } else {
                                let assign = (0..results.len())
                                    .map(|i| format!("final r{i} = results[{i}];"))
                                    .collect::<String>();
                                let values = results
                                    .iter()
                                    .enumerate()
                                    .map(|(i, (name, ty))| {
                                        format!(
                                            "{}: {}",
                                            name.as_var(),
                                            self.type_from_json(&format!("r{i}"), ty),
                                        )
                                    })
                                    .collect::<Vec<_>>()
                                    .join(", ");
                                if results_with_ctx {
                                    let parse_prefix = if from_world { "" } else { "_world." };
                                    format!("{assign}return {parse_prefix}withContext(() => ({values},));")
                                } else {
                                    format!("{assign}return ({values},);")
                                }
                            }
                        }
                    };
                    s.push_str(&format!(
                        "{results_assignments}{await_}_{name}([{params}]);{ret}"
                    ));
                }
                s.push_str("}");
            }
        }
    }
}
