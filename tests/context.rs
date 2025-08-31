use magic_params::{context_as_params, define_context};

define_context!(MyContext {
    value1: i32,
    value2: u32,
    value3: String,
});

#[test]
fn test_from_context() {
    let ctx = MyContext {
        value1: 50,
        value2: 100,
        value3: "Hello".to_string(),
    };

    let v1: i32 = i32::from_context(&ctx);
    let v2: u32 = u32::from_context(&ctx);
    let v3: String = String::from_context(&ctx);

    assert_eq!(v1, 50);
    assert_eq!(v2, 100);
    assert_eq!(v3, "Hello");
}


#[test]
fn test_handlers() {
    context_as_params!(MyContext, 3);

    let ctx = MyContext {
        value1: 50,
        value2: 100,
        value3: "Hello".to_string(),
    };

    fn handler(txt: String, fifty: i32) {
        assert_eq!(txt, "Hello");
        assert_eq!(fifty, 50);
    }

    let closures= |v1: i32, v2: u32, v3: String| {
        assert_eq!(v1, 50);
        assert_eq!(v2, 100);
        assert_eq!(v3, "Hello");
    };

    handler.call(&ctx);
    closures.call(&ctx);
}
