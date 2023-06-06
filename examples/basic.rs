use v8::{CreateParams, HandleScope, Isolate, V8, Script};
fn main() {
    //initialize v8
    init();
    //create an isolate
    let params = CreateParams::default();
    let mut isolate = Isolate::new(params);

    //create handle scope
    let handle_scop = &mut HandleScope::new(&mut isolate);
    //create context
    let context = v8::Context::new(handle_scop);
    //create context scop
    let context_scop = &mut v8::ContextScope::new(handle_scop, context);

    //javascript code

    let soure = r#"
        function hello() {
            return "Hello World";
        }
        hello();
    "#;
    let source = v8::String::new(context_scop, soure).unwrap();
    let script = Script::compile(context_scop, source, None).unwrap();
    let result = script
        .run(context_scop)
        .unwrap();
    let value: String = serde_v8::from_v8(context_scop, result).unwrap();
    println!("Result is: {}", value);
}

fn init() {
    let platform = v8::new_default_platform(0, false).make_shared();
    V8::initialize_platform(platform);
    V8::initialize();
}
