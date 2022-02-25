use std::collections::HashMap;

use crate::{Translator, DataOptions, Value, NumOrFormatting, ContextOptions};

#[test]
fn translate_hello() {
  let key = String::from("Hello");
  let value = String::from("Hello translated");

  let values = HashMap::from([
    (key.clone(), Value::Single(value.clone()))
  ]);  

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(values)
  });

  let actual = translator.translate(
    &key, 
    None, 
    None, 
    None
  );

  assert_eq!(actual, value);
}

#[test]
fn translate_plural_text() {
  let key = String::from("%n comments");

  let zero_comments = String::from("0 comments");
  let one_comment = String::from("1 comment");
  let two_comments = String::from("2 comments");
  let ten_comments = String::from("10 comments");

  let values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), String::from("%n comments")),
      (Some(1), Some(1), String::from("%n comment")),
      (Some(2), None, String::from("%n comments"))
    ]))
  ]);

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(values)
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(0)), 
    None, 
    None
  );

  assert_eq!(actual, zero_comments);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None
  );

  assert_eq!(actual, one_comment);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(2)), 
    None, 
    None
  );

  assert_eq!(actual, two_comments);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(10)), 
    None, 
    None
  );

  assert_eq!(actual, ten_comments);
}

#[test]
fn translate_plural_text_with_negative_number() {
  let key = String::from("Due in %n days");

  let due_ten_days_ago = String::from("Due 10 days ago");
  let due_two_days_ago = String::from("Due 2 days ago");
  let due_yesterday = String::from("Due Yesterday");
  let due_today = String::from("Due Today");
  let due_tomorrow = String::from("Due Tomorrow");
  let due_in_two_days = String::from("Due in 2 days");
  let due_in_ten_days = String::from("Due in 10 days");

  let values = HashMap::from([
    (key.clone(), Value::List(vec![
      (None, Some(-2), String::from("Due -%n days ago")),
      (Some(-1), Some(-1), String::from("Due Yesterday")),
      (Some(0), Some(0), String::from("Due Today")),
      (Some(1), Some(1), String::from("Due Tomorrow")),
      (Some(2), None, String::from("Due in %n days"))
    ]))
  ]);

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(values)
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(-10)),
    None,
    None
  );

  assert_eq!(actual, due_ten_days_ago);

  let actual = translator.translate(&key, 
    Some(&NumOrFormatting::Number(-2)), 
    None, 
    None
  );

  assert_eq!(actual, due_two_days_ago);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(-1)), 
    None, 
    None
  );

  assert_eq!(actual, due_yesterday);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(0)), 
    None, 
    None
  );

  assert_eq!(actual, due_today);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None
  );

  assert_eq!(actual, due_tomorrow);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(2)), 
    None, 
    None
  );

  assert_eq!(actual, due_in_two_days);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(10)), 
    None, 
    None
  );

  assert_eq!(actual, due_in_ten_days);
}

#[test]
fn translate_text_with_formatting() {
  let key = String::from("Welcome %{name}");
  let value = String::from("Welcome John");

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: None
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("name"), String::from("John"))
    ]))), 
    None, 
    None
  );

  assert_eq!(actual, value);
}

#[test]
fn translate_text_using_contexts() {
  let key = String::from("%{name} updated their profile");

  let john_value = String::from("John updated his profile");
  let jane_value = String::from("Jane updated her profile");

  let male_values = HashMap::from([
    (key.clone(), Value::Single(String::from("%{name} updated his profile")))
  ]);

  let female_values = HashMap::from([
    (key.clone(), Value::Single(String::from("%{name} updated her profile")))
  ]);

  let contexts = vec![
    ContextOptions {
      matches: HashMap::from([
        (String::from("gender"), String::from("male"))
      ]),
      values: male_values
    },
    ContextOptions {
      matches: HashMap::from([
        (String::from("gender"), String::from("female"))
      ]),
      values: female_values
    }
  ];

  let translator = Translator::create(&DataOptions {
    contexts: Some(contexts),
    values: None
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("name"), String::from("John"))
    ]))), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("gender"), String::from("male"))
    ]))), 
    None
  );

  assert_eq!(actual, john_value);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("name"), String::from("Jane"))
    ]))), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("gender"), String::from("female"))
    ]))), 
    None
  );

  assert_eq!(actual, jane_value);
}

#[test]
fn translate_plural_text_using_contexts() {
  let key = String::from("%{name} uploaded %n photos to their %{album} album");

  let john_value = String::from("John uploaded 1 photo to his Buck's Night album");
  let jane_value = String::from("Jane uploaded 4 photos to her Hen's Night album");

  let male_values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), String::from("%{name} uploaded %n photos to his %{album} album")),
      (Some(1), Some(1), String::from("%{name} uploaded %n photo to his %{album} album")),
      (Some(2), None, String::from("%{name} uploaded %n photos to his %{album} album"))
    ]))
  ]);

  let female_values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), String::from("%{name} uploaded %n photos to her %{album} album")),
      (Some(1), Some(1), String::from("%{name} uploaded %n photo to her %{album} album")),
      (Some(2), None, String::from("%{name} uploaded %n photos to her %{album} album"))
    ]))
  ]);

  let contexts = vec![
    ContextOptions {
      matches: HashMap::from([(String::from("gender"), String::from("male"))]),
      values: male_values
    },
    ContextOptions {
      matches: HashMap::from([(String::from("gender"), String::from("female"))]),
      values: female_values
    }
  ];

  let translator = Translator::create(&DataOptions {
    contexts: Some(contexts),
    values: None
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("name"), String::from("John")),
      (String::from("album"), String::from("Buck's Night"))
    ]))), 
    Some(&HashMap::from([
      (String::from("gender"), String::from("male"))
    ]))
  );

  assert_eq!(actual, john_value);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(4)), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      (String::from("name"), String::from("Jane")),
      (String::from("album"), String::from("Hen's Night"))
    ]))), 
    Some(&HashMap::from([
      (String::from("gender"), String::from("female"))
    ]))
  );

  assert_eq!(actual, jane_value);
}

#[test]
fn translate_plural_text_using_extension() {
  let key = String::from("%n results");

  let zero_results = String::from("нет результатов");
  let one_result = String::from("1 результат");
  let eleven_results = String::from("11 результатов");
  let four_results = String::from("4 результата");
  let results = String::from("101 результат");

  let data = HashMap::from([
    (String::from("zero"), String::from("нет результатов")),
    (String::from("one"), String::from("%n результат")),
    (String::from("few"), String::from("%n результата")),
    (String::from("many"), String::from("%n результатов")),
    (String::from("other"), String::from("%n результаты"))
  ]);

  let values = HashMap::from([
    (key.clone(), Value::Map(data))
  ]);

  let mut translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(values)
  });

  fn russian_extension(
    _: &String, 
    num: Option<&i64>, 
    _: Option<&HashMap<String, String>>, 
    data: Option<&HashMap<String, String>>
  ) -> String {
    let key = match num {
      Some(0) => String::from("zero"),
      Some(num) => {
        if num % 10 == 1 && num % 100 != 11 { String::from("one") }
        else if vec![2, 3, 4].contains(&(num % 10)) && !vec![12, 13, 14].contains(&(num % 100)) { String::from("few") }
        else if num % 10 == 0 || vec![5, 6, 7, 8, 9].contains(&(num % 10)) || vec![11, 12, 13, 14].contains(&(num % 100)) { String::from("many") }
        else { String::from("other") }
      },
      _ => String::from("zero")
    };

    match data {
      Some(data) => {
        match data.get(&key) {
          Some(value) => value.clone(),
          None => String::new()
        }
      },
      None => String::new()
    }
  }

  translator.extend(russian_extension);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(0)), 
    None, 
    None
  );

  assert_eq!(actual, zero_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None
  );

  assert_eq!(actual, one_result);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(11)), 
    None, 
    None
  );

  assert_eq!(actual, eleven_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(4)), 
    None, 
    None
  );

  assert_eq!(actual, four_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(101)), 
    None, 
    None
  );

  assert_eq!(actual, results);
}
