# Partial Struct

Forked from optional_struct, adding support for enum structs

To test:

```sh
cargo watch -x 'test && say "ok" || say "ko"'
```

To view the result of macro expansion:

```sh
cargo watch -x 'expand --test 008-enum' 
```

## Goal

This crate allows the user to generate a structure containing the same fields as the original struct but wrapped in Option<T>.
A method is also implemented for the original struct, `apply_options`. It consumes the generated partial_struct, and for every Some(x) field, it assigns the original structure's value with the partial_struct one.

Now that's some confusing explanation (my English skills could use some help), but basically:


```rust
#[derive(PartialStruct)]
struct Foo {
	meow: u32,
	woof: String,
}
```

will generate:

```rust
struct PartialFoo {
	meow: Option<u32>,
	woof: Option<String>,
}

impl Foo {
	pub fn apply_options(&mut self, partial_struct: PartialFoo) {
		if Some(field) = partial_struct.meow {
			self.meow = field;
		}

		if Some(field) = partial_struct.woof {
			self.woof = field;
		}

	}
}
```

## Usage

You can use this to generate a configuration for you program more easily.
If you use [toml-rs](https://github.com/alexcrichton/toml-rs) to parse your config file (using serde),
you'll need to wrap your values in Option<T>, or you need them present in the config file.
With this crate, you can easily generate your whole Config struct with an Option<T> wrap for each field.
This means that if a config is missing in the file, you'll get a None.

You can then easily handle default values for your config:

```rust
impl Config {
	pub fn get_user_conf() -> PartialConfig {
		toml::from_str<PartialConfig>(r#"
			ip = '127.0.0.1'

			[keys]
			github = 'xxxxxxxxxxxxxxxxx'
			travis = 'yyyyyyyyyyyyyyyyy'
		    "#).unwrap()
	}
}

let mut conf = Config::get_default();
let user_conf = Config::get_user_conf();
conf.apply_options(user_conf);
```

## Features

* Option<T> inside the original structs are handled. The generated struct will have the exact same field, not an Option<Option<T>>
* You can rename the generated struct:
```rust
#[derive(PartialStruct)]
#[partial_name = "FoorBarMeowWoof"]
```
* You can also add derives to the generated struct:
```rust
#[derive(PartialStruct)]
#[partial_derive(Serialize, Copy, Display)]
```
* You can also nest your generated struct by mapping the original types to their new names:
```rust
#[derive(PartialStruct)]
#[opt_nested_original(LogConfig)]
#[opt_nested_generated(PartialLogConfig)]
struct Config {
    timeout: Option<u32>,
    log_config: LogConfig,
}

#[derive(PartialStruct)]
struct LogConfig {
    log_file: String,
    log_level: usize,
}
```

You'll find some examples in the tests folder (yes I know).
