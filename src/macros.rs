#[macro_export]
macro_rules! list_commands {
    () => {{
        let mut c:Vec<&str> = Vec::new();
        c.push("up");
        c.push("halt");
        c.push("destroy");
        c.push("status");
        c.push("suspend");
        c.push("reload");
        c.push("resume");
        c
    }}
}
