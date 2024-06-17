use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::path::{Path, PathBuf};

use super::CompilerError;

pub struct Module {
    pub name: String,
    // Doesn't include the module's own name.
    pub path: Vec<String>,
    pub content: TokenStream,
    pub children: Vec<Module>,
}

#[allow(unused)]
impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self::new_with_path(name, Vec::<String>::new())
    }

    pub fn new_with_path(
        name: impl Into<String>,
        path: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let path = path.into_iter().map(Into::into).collect::<Vec<_>>();

        Self {
            name: name.into(),
            path,
            content: quote::quote!(),
            children: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn all_segments(&self) -> Vec<String> {
        let mut segments: Vec<_> = self.path.clone();

        segments.push(self.name.clone());
        segments
    }

    pub fn all_idents(&self) -> Vec<Ident> {
        let mut segments: Vec<_> = self
            .path
            .iter()
            .map(|segment| quote::format_ident!("{}", segment))
            .collect();

        segments.push(quote::format_ident!("{}", self.name));
        segments
    }

    pub fn full_path(&self) -> TokenStream {
        let idents = self.all_idents();
        quote::quote! {
            #(#idents)::*
        }
    }

    pub fn prepend(&mut self, token_stream: impl ToTokens) {
        let mut token_stream = token_stream.to_token_stream();
        std::mem::swap(&mut self.content, &mut token_stream);
        self.content.extend(token_stream);
    }

    pub fn write(&self, target: impl Into<PathBuf>) -> Result<(), CompilerError> {
        let target = target.into();
        let module_path = self.write_inner(target)?;

        // @TODO jeremy.barrow - 08 Feb 2024: Add a way to disable this.
        let output = std::process::Command::new("rustfmt")
            .arg(module_path)
            .output()
            .expect("Unable to handle process");

        let out = String::from_utf8(output.stdout).expect("Unable to handle process");
        if !out.is_empty() {
            panic!("{}", out);
        }
        let out = String::from_utf8(output.stderr).expect("Unable to handle process");
        if !out.is_empty() {
            panic!("{}", out);
        }

        Ok(())
    }

    fn write_inner(&self, target: impl AsRef<Path>) -> Result<PathBuf, CompilerError> {
        let target = target.as_ref();

        std::fs::create_dir_all(target)?;

        // We need to write the file, so the developer can figure out what was generated incorrectly.
        let file: Result<syn::File, _> = syn::parse2(self.content.clone()).map_err(Into::into);

        let (content, parse_err): (TokenStream, Option<CompilerError>) = match file {
            Ok(mut file) => {
                let mods: Vec<syn::Item> = self
                    .children
                    .iter()
                    .map(|child| {
                        let name = quote::format_ident!("{}", child.name);
                        syn::parse_quote! {
                            pub mod #name;
                        }
                    })
                    .collect();

                file.items.extend(mods);

                // @TODO jeremy.barrow - 03 Jan 2024: Should we only sort `use` up to the first non-`use` item?
                file.items.sort_by_key(|item| match item {
                    syn::Item::Use(_) => 16u8,
                    syn::Item::Mod(_) => 32,
                    _ => 64,
                });

                (file.to_token_stream(), None)
            }
            Err(err) => (self.content.clone(), Some(err)),
        };

        let content = quote::quote! {
            //!THIS FILE HAS BEEN GENERATED

            #content
        };

        let file = format!("{}.rs", self.name);
        let module_path = target.join(file);
        write(content, &module_path)?;

        // Now that the file is written, we can abort "cleanly".
        if let Some(err) = parse_err {
            return Err(err);
        }

        let target = target.join(&self.name);
        for module in self.children.iter() {
            module.write_inner(&target)?;
        }

        Ok(module_path)
    }

    pub fn create_child(&mut self, name: &str) -> &mut Module {
        self.create_child_from_path(std::iter::once(name))
    }

    pub fn create_child_from_path(
        &mut self,
        path: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> &mut Module {
        let mut path = path.into_iter();

        let mut module = self;
        loop {
            let segment = match path.next() {
                Some(value) => value,
                None => {
                    break module;
                }
            };
            let segment = segment.as_ref();

            let next = module
                .children
                .iter_mut()
                .position(|item| item.name == segment);

            module = if let Some(index) = next {
                &mut module.children[index]
            } else {
                let mut path = module.path.clone();
                path.push(module.name.clone());
                let child = Module::new_with_path(segment, path);
                module.children.push(child);
                module.children.last_mut().expect("We just added it")
            }
        }
    }
}

impl Extend<proc_macro2::TokenTree> for Module {
    fn extend<I: IntoIterator<Item = proc_macro2::TokenTree>>(&mut self, streams: I) {
        self.content.extend(streams);
    }
}

fn write(tokens: TokenStream, out: impl AsRef<Path>) -> Result<(), CompilerError> {
    let path = out.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let content = format!("{}", tokens);

    #[cfg(feature = "pretty")]
    fn format(content: String) -> Result<String, CompilerError> {
        let file = syn::parse_file(&content).context("Unable to parse tokens")?;

        let formatted = prettyplease::unparse(&file);

        Ok(formatted)
    }
    #[cfg(not(feature = "pretty"))]
    fn format(content: String) -> Result<String, CompilerError> {
        Ok(content)
    }

    let content = format(content)?;

    std::fs::write(path, content)?;

    Ok(())
}
