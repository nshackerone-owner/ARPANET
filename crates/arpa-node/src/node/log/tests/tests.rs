mod logger;

#[cfg(test)]
mod tests {
    use crate::logger::{self, SimpleLogger, SL};
    use arpa_node_log::LogModel;
    use arpa_node_log_impl::log_function;
    use log::debug;
    use std::sync::Once;

    #[log_function]
    fn omit_return(_a: usize) {}

    #[log_function]
    fn return_nothing(_a: usize) {
        return;
    }

    #[log_function]
    async fn async_and_return_value(a: usize, b: usize) -> usize {
        return a + b;
    }

    #[log_function]
    fn more_than_seven_inputs(
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        aa: usize,
        bb: usize,
        cc: usize,
        dd: usize,
    ) -> usize {
        a + b + c + d + aa + bb + cc + dd
    }

    #[log_function]
    fn expr_stmt_in_sub_blocks(a: usize, b: usize) -> usize {
        if true {
            a + b
        } else {
            a - b
        }
    }

    #[log_function]
    fn return_match_expr(_a: usize) -> usize {
        match 1 > 0 {
            true => 1,
            false => 0,
        }
    }

    #[log_function("ignore-return")]
    fn ignore_return(a: usize, b: usize) -> usize {
        a + b
    }

    #[log_function]
    fn return_error_by_question_mark_operator() -> anyhow::Result<()> {
        std::fs::read_to_string("noexist")?;

        Ok(())
    }

    static START: Once = Once::new();
    static LOGGER: &SimpleLogger = &SL;
    // Sure to run this once
    fn setup_tests() {
        START.call_once(|| {
            build_logger();
        });
    }

    fn build_logger() -> &'static SimpleLogger {
        logger::init().unwrap()
    }

    #[tokio::test]
    async fn test_async_and_return_value() {
        setup_tests();
        async_and_return_value(3, 5).await;
        let expected = build_expected_log("async_and_return_value", &["a: 3", "b: 5"], "8");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_omit_return() {
        setup_tests();
        omit_return(1);
        let expected = build_expected_log("omit_return", &["_a: 1"], "\"nothing\"");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_return_nothing() {
        setup_tests();
        return_nothing(1);
        let expected = build_expected_log("return_nothing", &["_a: 1"], "\"nothing\"");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_more_than_seven_inputs() {
        setup_tests();
        more_than_seven_inputs(1, 2, 3, 4, 1, 2, 3, 4);
        let expected = build_expected_log(
            "more_than_seven_inputs",
            &[
                "a: 1", "b: 2", "c: 3", "d: 4", "aa: 1", "bb: 2", "cc: 3", "dd: 4",
            ],
            "20",
        );
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_expr_stmt_in_sub_blocks() {
        setup_tests();
        expr_stmt_in_sub_blocks(3, 5);
        let expected = build_expected_log("expr_stmt_in_sub_blocks", &["a: 3", "b: 5"], "8");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_return_match_expr() {
        setup_tests();
        return_match_expr(0);
        let expected = build_expected_log("return_match_expr", &["_a: 0"], "1");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_ignore_return() {
        setup_tests();
        ignore_return(1, 2);
        let expected = build_expected_log("ignore_return", &["a: 1", "b: 2"], "\"ignored\"");
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    #[test]
    fn test_return_error_by_question_mark_operator() {
        setup_tests();
        return_error_by_question_mark_operator()
            .expect_err("This should fail for file no existing.");
        let expected = build_expected_log(
            "return_error_by_question_mark_operator",
            &[],
            "Err(No such file or directory (os error 2))",
        );
        assert_eq!(expected, LOGGER.last_message().unwrap());
    }

    fn build_expected_log(fn_name: &str, fn_args: &[&str], fn_return: &str) -> String {
        let log = LogModel {
            fn_name,
            fn_args,
            fn_return,
        };
        let expected = format!(
            "{{\"message\":{:?},\"level\":\"DEBUG\",\"target\":\"{}\",\"mdc\":{{\"fn_name\":\"{}\"}}}}",
            format!("{:?}",log), fn_name, fn_name
        );
        expected
    }
}
