#[cfg(test)]
mod tests;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value<'a> {
	Single(&'a str),
	List(Vec<(Option<i64>, Option<i64>, &'a str)>),
	Map(HashMap<&'a str, &'a str>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumOrFormatting<'a> {
	Number(i64),
	#[serde(borrow)]
	Formatting(HashMap<&'a str, &'a str>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextOptions<'a> {
	#[serde(borrow)]
	pub matches: HashMap<&'a str, &'a str>,
	#[serde(borrow)]
	pub values: HashMap<&'a str, Value<'a>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataOptions<'a> {
	#[serde(borrow)]
	pub values: Option<HashMap<&'a str, Value<'a>>>,
	#[serde(borrow)]
	pub contexts: Option<Vec<ContextOptions<'a>>>    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translator<'a, 'b> {
	#[serde(borrow)]
	data: Option<DataOptions<'a>>,
	#[serde(borrow)]
	global_context: HashMap<&'a str, &'a str>,

	#[serde(skip)]
	extension: Option<fn(
		text: &'b str,
		num: Option<i64>,
		formatting: Option<&'b HashMap<&'a str, &'a str>>,
		data: Option<&'a HashMap<&'a str, &'a str>>	
	) -> String>
}

impl<'a, 'b> Translator<'a, 'b> {
	pub fn add(&mut self, data: &'b DataOptions<'a>) {
		if let Some(self_data) = &mut self.data {
			if let Some(data_values) = &data.values {
				match &mut self_data.values {
					Some(self_data_values) => {
						self_data_values
							.extend(
								data_values
											.iter()
											.map(|(key, value)| (key.clone(), value.clone()))
							);
					},
					None => {
						self_data.values = Some(data_values.clone());
					}
				}
			}	
			
			if let Some(data_contexts) = &data.contexts {
				match &mut self_data.contexts {
					Some(self_data_contexts) => {
						self_data_contexts
							.extend(
								data_contexts
									.iter()
									.map(|context| context.clone())
							);
					},
					None => {
						self_data.contexts = Some(data_contexts.clone());
					}
				}
			}
		} else {
			self.data = Some(data.clone());
		}
	}

	pub fn set_context(&mut self, key: &'a str, value: &'a str) {
		self.global_context.insert(key.clone(), value.clone());
	}

	pub fn clear_context(&mut self, key: &'a str) {
		self.global_context.remove(key);
	}

	pub fn reset_data(&mut self) {
		self.data = None;
	}

	pub fn reset_context(&mut self) {
		self.global_context = HashMap::new();
	}

	pub fn reset(&mut self) {
		self.reset_data();
		self.reset_context();
	}

	pub fn extend(
		&mut self, 
		extension: fn(
			text: &'b str,
			num: Option<i64>,
			formatting: Option<&'b HashMap<&'a str, &'a str>>,
			data: Option<&'a HashMap<&'a str, &'a str>>
		) -> String
	) {
		self.extension = Some(extension);
	}

	pub fn translate(
		&'a self,
		text: &'b str,
		default_num_or_formatting: Option<&'b NumOrFormatting<'a>>,
		num_or_formatting_or_context: Option<&'b NumOrFormatting<'a>>,
		formatting_or_context: Option<&'b HashMap<&'a str, &'a str>>
	) -> String {
		let mut num = None;
		let mut formatting = None;
		let mut context = &self.global_context;

		if let Some(default_num_or_formatting_val) = default_num_or_formatting {
			match default_num_or_formatting_val {
				NumOrFormatting::Formatting(default_formatting) => {
					formatting = Some(default_formatting);

					if let Some(num_or_formatting_or_context_val) = num_or_formatting_or_context {
						if let NumOrFormatting::Formatting(default_context) = num_or_formatting_or_context_val {
							context = default_context;
						}
					}
				},
				NumOrFormatting::Number(default_num) => {
					num = Some(*default_num);

					if let Some(num_or_formatting_or_context_val) = num_or_formatting_or_context {
						if let NumOrFormatting::Formatting(default_formatting) = num_or_formatting_or_context_val {
							formatting = Some(default_formatting);
						}
					}
	
					if let Some(default_context) = formatting_or_context {
						context = default_context;
					}
				}
			}			
		} else if let Some(num_or_formatting_or_context_val) = num_or_formatting_or_context {
			match &num_or_formatting_or_context_val {
				NumOrFormatting::Number(default_num) => {
					num = Some(*default_num);
					formatting = formatting_or_context;
				},
				&NumOrFormatting::Formatting(default_formatting) => {
					formatting = Some(default_formatting);

					if let Some(default_context) = formatting_or_context {
						context = default_context;
					}
				}
			}			
		}

		return self.translate_text(text, num, formatting, context);
	}

	pub fn translate_text(
		&'a self, 
		text: &'b str,
		num: Option<i64>,
		formatting: Option<&'b HashMap<&'a str, &'a str>>,
		context: &'b HashMap<&'a str, &'a str>
	) -> String {
		match &self.data {
			Some(data) => {
				let context_data = get_context_data(data, context);

				let mut text_val = String::new();
				let mut text_is_null = true;

				if let Some(context_data) = context_data {
					if let Some(text) = self.find_translation(text, num, formatting, Some(&context_data.values)) {
						text_val = text;
						text_is_null = false;
					}
				}

				if text_is_null {
					if let Some(text) = self.find_translation(text, num, formatting, data.values.as_ref()) {
						text_val = text;
						text_is_null = false;
					}
				}

				if text_is_null {
					text_val = use_original_text(text, num, formatting);
				}

				text_val
			},
			None => use_original_text(text, num, formatting)
		}		
	}

	pub fn find_translation(
		&self,
		text: &'b str,
		num: Option<i64>,
		formatting: Option<&'b HashMap<&'a str, &'a str>>,
		data: Option<&'a HashMap<&'a str, Value>>
	) -> Option<String> {
		let value = data?.get(text)?;

		match value {
			Value::Single(value) => Some(apply_formatting(value, formatting)),
			Value::Map(value) => {
				match self.extension {
					Some(func) => {
						let text = func(text, num, formatting, Some(value));
						let text = apply_numbers(&text, if let Some(num) = num { num } else { 0 });

						Some(apply_formatting(&text, formatting))
					},
					None => {
						Some(use_original_text(text, num, formatting))
					}
				}
			},
			Value::List(value) => {
				let mut num_val = 0;
				let mut num_is_null = true;

				if let Some(num) = num {
					num_val = num;
					num_is_null = false;
				}

				for triple in value {
					let mut low = 0;
					let mut low_is_null = true;

					let mut high = 0;
					let mut high_is_null = true;

					if let Some(num) = triple.0 {
						low = num;
						low_is_null = false;
					}

					if let Some(num) = triple.1 {
						high = num;
						high_is_null = false;
					}

					if num_is_null && low_is_null && high_is_null ||
						!num_is_null && (
							!low_is_null && num_val >= low && (high_is_null || num_val <= high) ||
							low_is_null && !high_is_null && num_val <= high
					) {
						let text = apply_numbers(&triple.2, num_val);

						return Some(apply_formatting(&text, formatting));
					}
				}

				None
			}
		}
	}

	pub fn create(data: &'b DataOptions<'a>) -> Translator<'a, 'b> {
		let mut translator = Translator {
			data: None,
			global_context: HashMap::new(),
			extension: None
		};

		translator.add(data);

		return translator;
	}
}

pub fn apply_numbers<'a>(str: &'a str, num: i64) -> String {
	let str = str.replace("-%n", &(-num).to_string());

	return str.replace("%n", &num.to_string());
}

pub fn apply_formatting<'a>(text: &'a str, formatting: Option<&'a HashMap<&'a str, &'a str>>) -> String {
	let mut text = String::from(text);

	if let Some(formatting) = formatting {
		for (key, value) in formatting {				
			text = text.replace(&("%{".to_owned() + key + "}"), value);
		}
	}

	return text;
}

pub fn get_context_data<'a, 'b>(data: &'a DataOptions<'a>, context: &'b HashMap<&'a str, &'a str>) -> Option<&'a ContextOptions<'a>> {
	for ctx in data.contexts.as_ref()? {
		let mut equal = true;

		for (key, value) in &ctx.matches {
			equal = equal && *value == context[key];

			if !equal {
				break;
			}
		}

		if equal {
			return Some(ctx);
		}
	}

	return None;
}

pub fn use_original_text<'a>(text: &'a str, num: Option<i64>, formatting: Option<&'a HashMap<&'a str, &'a str>>) -> String {
	match num {
		Some(num) => apply_formatting(&text.replace("%n", &num.to_string()), formatting),
		None => apply_formatting(text, formatting)
	}
}
