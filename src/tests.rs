use std::collections::HashMap;

use crate::{Translator, DataOptions, Value, NumOrFormatting, ContextOptions};

#[test]
fn translate_hello() {
  let key = "Hello";
  let value = "Hello translated";  

  let translator = Translator::create(
    &DataOptions {
      contexts: None,
      values: Some(
        HashMap::from([
          (key.clone(), Value::Single(value.clone()))
        ])
      )
    }
  );

  let actual = translator.translate(
    &key, 
    &None, 
    &None, 
    &None    
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

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(
      HashMap::from([
        (key.clone(), Value::List(
          vec![
            (Some(0), Some(0), "%n comments"),
            (Some(1), Some(1), "%n comment"),
            (Some(2), None, "%n comments")
          ]
        ))
      ])
    )
  });

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(0)), 
    &None, 
    &None    
  );

  assert_eq!(actual, zero_comments);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(1)), 
    &None, 
    &None
  );

  assert_eq!(actual, one_comment);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(2)), 
    &None, 
    &None
  );

  assert_eq!(actual, two_comments);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(10)), 
    &None, 
    &None
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

  let translator = Translator::create(&DataOptions {
    contexts: None,
    values: Some(
      HashMap::from([
        (key.clone(), Value::List(
          vec![
            (None, Some(-2), "Due -%n days ago"),
            (Some(-1), Some(-1), "Due Yesterday"),
            (Some(0), Some(0), "Due Today"),
            (Some(1), Some(1), "Due Tomorrow"),
            (Some(2), None, "Due in %n days")
          ]
        ))
      ])
    )
  });

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(-10)),
    &None,
    &None    
  );

  assert_eq!(actual, due_ten_days_ago);

  let actual = translator.translate(&key, 
    &Some(NumOrFormatting::Number(-2)), 
    &None, 
    &None    
  );

  assert_eq!(actual, due_two_days_ago);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(-1)), 
    &None, 
    &None    
  );

  assert_eq!(actual, due_yesterday);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(0)), 
    &None, 
    &None
  );

  assert_eq!(actual, due_today);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(1)), 
    &None, 
    &None    
  );

  assert_eq!(actual, due_tomorrow);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(2)), 
    &None, 
    &None    
  );

  assert_eq!(actual, due_in_two_days);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(10)), 
    &None, 
    &None    
  );

  assert_eq!(actual, due_in_ten_days);
}

#[test]
fn translate_text_with_formatting() {
  let key = "Welcome %{name}";
  let value = "Welcome John";

  let translator = Translator::create(
    &DataOptions {
      contexts: None,
      values: None
    }
  );

  let actual = translator.translate(
    &key, 
    &Some(
      NumOrFormatting::Formatting(
        HashMap::from([
          ("name", "John")
        ])
      )
    ), 
    &None, 
    &None    
  );

  assert_eq!(actual, value);
}

#[test]
fn translate_text_using_contexts() {
  let key = "%{name} updated their profile";

  let john_value = "John updated his profile";
  let jane_value = "Jane updated her profile";  

  let translator = Translator::create(
    &DataOptions {
      contexts: Some(
        vec![
          ContextOptions {
            matches: HashMap::from([
              ("gender", "male")
            ]),
            values: HashMap::from([
              (key.clone(), Value::Single("%{name} updated his profile"))
            ])
          },
          ContextOptions {
            matches: HashMap::from([
              ("gender", "female")
            ]),
            values: HashMap::from([
              (key.clone(), Value::Single("%{name} updated her profile"))
            ])
          }
        ]
      ),
      values: None
    }
  );

  let john_formatting = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("name", "John")
      ])
    )
  );

  let john_context = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("gender", "male")
      ])
    )
  );

  let actual = translator.translate(
    &key, 
    john_formatting, 
    john_context, 
    &None    
  );

  assert_eq!(actual, john_value);

  let jane_formatting = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("name", "Jane")
      ])
    )
  );

  let jane_context = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("gender", "female")
      ])
    )
  );

  let actual = translator.translate(
    &key, 
    jane_formatting, 
    jane_context, 
    &None
  );

  assert_eq!(actual, jane_value);
}

#[test]
fn translate_plural_text_using_contexts() {
  let key = "%{name} uploaded %n photos to their %{album} album";

  let john_value = "John uploaded 1 photo to his Buck's Night album";
  let jane_value = "Jane uploaded 4 photos to her Hen's Night album";

  let translator = Translator::create(
    &DataOptions {
      contexts: Some(
        vec![
          ContextOptions {
            matches: HashMap::from([("gender", "male")]),
            values: HashMap::from([
              (key.clone(), Value::List(
                vec![
                  (Some(0), Some(0), "%{name} uploaded %n photos to his %{album} album"),
                  (Some(1), Some(1), "%{name} uploaded %n photo to his %{album} album"),
                  (Some(2), None, "%{name} uploaded %n photos to his %{album} album")
                ]
              ))
            ])
          },
          ContextOptions {
            matches: HashMap::from([("gender", "female")]),
            values: HashMap::from([
              (key.clone(), Value::List(
                vec![
                  (Some(0), Some(0), "%{name} uploaded %n photos to her %{album} album"),
                  (Some(1), Some(1), "%{name} uploaded %n photo to her %{album} album"),
                  (Some(2), None, "%{name} uploaded %n photos to her %{album} album")
                ]
              ))
            ])
          }
        ]
      ),
      values: None
    }
  );

  let john_formatting = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("name", "John"),
        ("album", "Buck's Night")
      ])
    )
  );

  let john_context = &Some(
    HashMap::from([
      ("gender", "male")
    ])
  );

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(1)), 
    john_formatting, 
    john_context        
  );  

  assert_eq!(actual, john_value);

  let jane_formatting = &Some(
    NumOrFormatting::Formatting(
      HashMap::from([
        ("name", "Jane"),
        ("album", "Hen's Night")
      ])
    )
  );

  let jane_context = &Some(
    HashMap::from([
      ("gender", "female")
    ])
  );

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(4)), 
    jane_formatting, 
    jane_context
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

  let mut translator = Translator::create(
    &DataOptions {
      contexts: None,
      values: Some(
        HashMap::from([
          (key.clone(), Value::Map(
            HashMap::from([
              ("zero", "нет результатов"),
              ("one", "%n результат"),
              ("few", "%n результата"),
              ("many", "%n результатов"),
              ("other", "%n результаты")
            ])
          ))
        ])
      )
    }
  );

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

  translator.extend(russian_extension);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(0)), 
    &None, 
    &None
  );

  assert_eq!(actual, zero_results);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(1)), 
    &None, 
    &None
  );

  assert_eq!(actual, one_result);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(11)), 
    &None, 
    &None
  );

  assert_eq!(actual, eleven_results);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(4)), 
    &None, 
    &None
  );

  assert_eq!(actual, four_results);

  let actual = translator.translate(
    &key, 
    &Some(NumOrFormatting::Number(101)), 
    &None, 
    &None
  );

  assert_eq!(actual, results);
}
