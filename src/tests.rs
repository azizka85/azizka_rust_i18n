use std::collections::HashMap;

use crate::{Translator, DataOptions, Value, NumOrFormatting, ContextOptions};

#[test]
fn translate_hello() {
  let key = "Hello";
  let value = "Hello translated";

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
    None,
    None
  );

  assert_eq!(actual, value);
}

#[test]
fn translate_plural_text() {
  let key = "%n comments";

  let zero_comments = "0 comments";
  let one_comment = "1 comment";
  let two_comments = "2 comments";
  let ten_comments = "10 comments";

  let values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), "%n comments"),
      (Some(1), Some(1), "%n comment"),
      (Some(2), None, "%n comments")
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
    None,
    None
  );

  assert_eq!(actual, zero_comments);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, one_comment);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(2)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, two_comments);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(10)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, ten_comments);
}

#[test]
fn translate_plural_text_with_negative_number() {
  let key = "Due in %n days";

  let due_ten_days_ago = "Due 10 days ago";
  let due_two_days_ago = "Due 2 days ago";
  let due_yesterday = "Due Yesterday";
  let due_today = "Due Today";
  let due_tomorrow = "Due Tomorrow";
  let due_in_two_days = "Due in 2 days";
  let due_in_ten_days = "Due in 10 days";

  let values = HashMap::from([
    (key.clone(), Value::List(vec![
      (None, Some(-2), "Due -%n days ago"),
      (Some(-1), Some(-1), "Due Yesterday"),
      (Some(0), Some(0), "Due Today"),
      (Some(1), Some(1), "Due Tomorrow"),
      (Some(2), None, "Due in %n days")
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
    None,
    None
  );

  assert_eq!(actual, due_ten_days_ago);

  let actual = translator.translate(&key, 
    Some(&NumOrFormatting::Number(-2)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_two_days_ago);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(-1)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_yesterday);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(0)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_today);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_tomorrow);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(2)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_in_two_days);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(10)), 
    None, 
    None,
    None
  );

  assert_eq!(actual, due_in_ten_days);
}

#[test]
fn translate_text_with_formatting() {
  let key = "Welcome %{name}";
  let value = "Welcome John";

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: None
  });

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      ("name", "John")
    ]))), 
    None, 
    None,
    None
  );

  assert_eq!(actual, value);
}

#[test]
fn translate_text_using_contexts() {
  let key = "%{name} updated their profile";

  let john_value = "John updated his profile";
  let jane_value = "Jane updated her profile";

  let male_values = HashMap::from([
    (key.clone(), Value::Single("%{name} updated his profile"))
  ]);

  let female_values = HashMap::from([
    (key.clone(), Value::Single("%{name} updated her profile"))
  ]);

  let contexts = vec![
    ContextOptions {
      matches: HashMap::from([
        ("gender", "male")
      ]),
      values: male_values
    },
    ContextOptions {
      matches: HashMap::from([
        ("gender", "female")
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
      ("name", "John")
    ]))), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      ("gender", "male")
    ]))), 
    None,
    None
  );

  assert_eq!(actual, john_value);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      ("name", "Jane")
    ]))), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      ("gender", "female")
    ]))), 
    None,
    None
  );

  assert_eq!(actual, jane_value);
}

#[test]
fn translate_plural_text_using_contexts() {
  let key = "%{name} uploaded %n photos to their %{album} album";

  let john_value = "John uploaded 1 photo to his Buck's Night album";
  let jane_value = "Jane uploaded 4 photos to her Hen's Night album";

  let male_values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), "%{name} uploaded %n photos to his %{album} album"),
      (Some(1), Some(1), "%{name} uploaded %n photo to his %{album} album"),
      (Some(2), None, "%{name} uploaded %n photos to his %{album} album")
    ]))
  ]);

  let female_values = HashMap::from([
    (key.clone(), Value::List(vec![
      (Some(0), Some(0), "%{name} uploaded %n photos to her %{album} album"),
      (Some(1), Some(1), "%{name} uploaded %n photo to her %{album} album"),
      (Some(2), None, "%{name} uploaded %n photos to her %{album} album")
    ]))
  ]);

  let contexts = vec![
    ContextOptions {
      matches: HashMap::from([("gender", "male")]),
      values: male_values
    },
    ContextOptions {
      matches: HashMap::from([("gender", "female")]),
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
      ("name", "John"),
      ("album", "Buck's Night")
    ]))), 
    Some(&HashMap::from([
      ("gender", "male")
    ])),
    None
  );

  assert_eq!(actual, john_value);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(4)), 
    Some(&NumOrFormatting::Formatting(HashMap::from([
      ("name", "Jane"),
      ("album", "Hen's Night")
    ]))), 
    Some(&HashMap::from([
      ("gender", "female")
    ])),
    None
  );

  assert_eq!(actual, jane_value);
}

#[test]
fn translate_plural_text_using_extension() {
  let key = "%n results";

  let zero_results = "нет результатов";
  let one_result = "1 результат";
  let eleven_results = "11 результатов";
  let four_results = "4 результата";
  let results = "101 результат";

  let data = HashMap::from([
    ("zero", "нет результатов"),
    ("one", "%n результат"),
    ("few", "%n результата"),
    ("many", "%n результатов"),
    ("other", "%n результаты")
  ]);

  let values = HashMap::from([
    (key.clone(), Value::Map(data))
  ]);

  let mut translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(values)
  });

  fn russian_extension(
    _: &str, 
    num: Option<i64>, 
    _: Option<&HashMap<&str, &str>>, 
    data: Option<&HashMap<&str, &str>>
  ) -> String {
    let key = match num {
      Some(0) => "zero",
      Some(num) => {
        if num % 10 == 1 && num % 100 != 11 {"one" }
        else if vec![2, 3, 4].contains(&(num % 10)) && !vec![12, 13, 14].contains(&(num % 100)) { "few" }
        else if num % 10 == 0 || vec![5, 6, 7, 8, 9].contains(&(num % 10)) || vec![11, 12, 13, 14].contains(&(num % 100)) { "many" }
        else { "other" }
      },
      _ => "zero"
    };

    match data {
      Some(data) => {
        match data.get(&key) {
          Some(value) => String::from(value.to_owned()),
          None => String::new()
        }
      },
      None => String::new()       
    }
  }

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(0)), 
    None, 
    None,
    Some(russian_extension)
  );

  assert_eq!(actual, zero_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(1)), 
    None, 
    None,
    Some(russian_extension)
  );

  assert_eq!(actual, one_result);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(11)), 
    None, 
    None,
    Some(russian_extension)
  );

  assert_eq!(actual, eleven_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(4)), 
    None, 
    None,
    Some(russian_extension)
  );

  assert_eq!(actual, four_results);

  let actual = translator.translate(
    &key, 
    Some(&NumOrFormatting::Number(101)), 
    None, 
    None,
    Some(russian_extension)
  );

  assert_eq!(actual, results);
}
