#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[doc = include_str!("../README.md")]
#[proc_macro_attribute]
pub fn namefn(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	__internal_namefn(attr, item)
}

#[doc(hidden)]
fn __internal_namefn(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let mut function = syn::parse_macro_input!(item as syn::ItemFn);

	let attr = attr.to_string();
	let mut attr = attr.split_ascii_whitespace();

	let mut const_name: String = String::from("NAME");
	let mut function_name: String = String::new();

	// Parse attributes
	while let Some(token) = attr.next() {
		match token {
			"const" => {
				if attr.next() == Some("=") {
					if let Some(const_token) = attr.next() {
						const_name = const_token.to_uppercase().trim_matches('"').to_owned()
					}
				} else {
					break;
				}
			},
			"alias" => {
				if attr.next() == Some("=") {
					if let Some(function_token) = attr.next() {
						function_name = function_token.trim_matches('"').to_owned()
					}
				} else {
					break;
				}
			},
			_ => (),
		}
	}

	// If no attribute `alias` is present we get the real function name
	// Fixme: For some reason there is a difference between `function_name.is_empty()` and `function_name == ""`
	#[allow(clippy::comparison_to_empty)]
	if function_name == "" {
		function_name = function.sig.ident.to_string();
	}

	let const_name = proc_macro2::Ident::new(&const_name, proc_macro2::Span::call_site());
	let function_name = syn::LitStr::new(&function_name, proc_macro2::Span::call_site());
	let const_stmt: syn::Stmt = syn::parse2(quote::quote! {
		const #const_name: &str = #function_name;
	})
	.unwrap();

	function.block.stmts.insert(0, const_stmt);

	let tokens = quote::quote! {
		#function
	};

	tokens.into()
}
