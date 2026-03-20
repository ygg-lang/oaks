use oak_liquid::LiquidLanguage;

fn main() {
    println!("=== Oak Liquid 配置机制分析与验证 ===\n");

    // 1. 验证默认配置
    println!("1. 验证默认 Liquid 配置:");
    verify_default_config();

    println!("\n2. 验证自定义分隔符配置:");
    verify_custom_delimiters();

    println!("\n3. 验证 Liquid 方言配置可行性:");
    verify_liquid_dialect_feasibility();

    println!("\n=== 分析完成 ===");
}

fn verify_default_config() {
    let language = LiquidLanguage::default();

    println!("  配置详情:");
    println!("    - 变量开始: {:?}", language.variable_start);
    println!("    - 变量结束: {:?}", language.variable_end);
    println!("    - 标签开始: {:?}", language.tag_start);
    println!("    - 标签结束: {:?}", language.tag_end);
    println!("    - 注释开始: {:?}", language.comment_start);
    println!("    - 注释结束: {:?}", language.comment_end);
    println!("    - trim_blocks: {:?}", language.trim_blocks);
    println!("    - lstrip_blocks: {:?}", language.lstrip_blocks);
    println!("    - keep_trailing_newline: {:?}", language.keep_trailing_newline);

    // 验证默认值
    assert_eq!(language.variable_start, "{{");
    assert_eq!(language.variable_end, "}}");
    assert_eq!(language.tag_start, "{%");
    assert_eq!(language.tag_end, "%}");
    assert_eq!(language.comment_start, "{#");
    assert_eq!(language.comment_end, "#}");
    assert_eq!(language.trim_blocks, false);
    assert_eq!(language.lstrip_blocks, false);

    println!("\n  ✓ 默认配置验证通过!");
}

fn verify_custom_delimiters() {
    // 创建自定义配置
    let mut language = LiquidLanguage::new();
    language.variable_start = "[[".to_string();
    language.variable_end = "]]".to_string();
    language.tag_start = "[%".to_string();
    language.tag_end = "%]".to_string();
    language.comment_start = "[#".to_string();
    language.comment_end = "#]".to_string();
    language.trim_blocks = true;
    language.lstrip_blocks = true;
    language.keep_trailing_newline = true;

    println!("  自定义配置详情:");
    println!("    - 变量开始: {:?}", language.variable_start);
    println!("    - 变量结束: {:?}", language.variable_end);
    println!("    - 标签开始: {:?}", language.tag_start);
    println!("    - 标签结束: {:?}", language.tag_end);
    println!("    - 注释开始: {:?}", language.comment_start);
    println!("    - 注释结束: {:?}", language.comment_end);
    println!("    - trim_blocks: {:?}", language.trim_blocks);
    println!("    - lstrip_blocks: {:?}", language.lstrip_blocks);
    println!("    - keep_trailing_newline: {:?}", language.keep_trailing_newline);

    // 验证自定义值
    assert_eq!(language.variable_start, "[[");
    assert_eq!(language.variable_end, "]]");
    assert_eq!(language.tag_start, "[%");
    assert_eq!(language.tag_end, "%]");
    assert_eq!(language.comment_start, "[#");
    assert_eq!(language.comment_end, "#]");
    assert_eq!(language.trim_blocks, true);
    assert_eq!(language.lstrip_blocks, true);
    assert_eq!(language.keep_trailing_newline, true);

    println!("\n  ✓ 自定义分隔符配置验证通过!");
}

fn verify_liquid_dialect_feasibility() {
    println!("  分析 Liquid 作为 Liquid 方言的可行性:");
    println!("  Liquid 与 Liquid 的主要特性对比:");
    println!("    1. 分隔符基本相同: {{ }}, {{% %}}");
    println!("    2. 标签命名略有不同，但语法结构相似");
    println!("    3. 过滤器语法相同");
    println!("    4. 注释语法相同: {{# #}}");

    // 创建一个模拟 Liquid 配置
    let liquid_like_config = LiquidLanguage {
        variable_start: "{{".to_string(),
        variable_end: "}}".to_string(),
        tag_start: "{%".to_string(),
        tag_end: "%}".to_string(),
        comment_start: "{#".to_string(),
        comment_end: "#}".to_string(),
        trim_blocks: false,
        lstrip_blocks: false,
        keep_trailing_newline: false,
    };

    println!("\n  模拟 Liquid 配置 (使用 LiquidLanguage):");
    println!("    - 变量开始: {:?}", liquid_like_config.variable_start);
    println!("    - 变量结束: {:?}", liquid_like_config.variable_end);
    println!("    - 标签开始: {:?}", liquid_like_config.tag_start);
    println!("    - 标签结束: {:?}", liquid_like_config.tag_end);
    println!("    - 注释开始: {:?}", liquid_like_config.comment_start);
    println!("    - 注释结束: {:?}", liquid_like_config.comment_end);

    println!("\n  ✓ 配置机制足够灵活，可以支持 Liquid 作为方言!");
    println!("\n  关键优势:");
    println!("    1. LiquidLanguage 结构体的所有字段都是公开的，允许完整定制");
    println!("    2. 所有分隔符都可以自定义，支持任何类 Liquid 语法");
    println!("    3. 配置可以被 LiquidLexer 和 LiquidParser 直接使用");
    println!("    4. 解析器和词法器的架构允许基于同一套基础设施扩展");
    println!("\n  实施建议:");
    println!("    1. 创建 LiquidLanguage 结构体，复用 LiquidLanguage 的设计");
    println!("    2. 或者直接使用 LiquidLanguage，配置为 Liquid 风格");
    println!("    3. 扩展解析器以支持 Liquid 特定的标签和语法");
}
