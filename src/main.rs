mod parser;
mod token;

fn main() {
    let p = parser::Parser::new();

    let tokens = p.parser(
        "# markdown解析器`rsmark`哈哈哈哈`op`end
       
>> rsmark取`rust`与*markdown*，表示一个由rust构建的markdown解析渲染器，
>> rsmark使用node表示各种类型的markdown元素。

## 解析器`架构设计`

### 这是三级标题

这是一段文本内容。

![Rust学习路线](https://picsum.photos/200/200)",
    );

    println!("{:?}", serde_json::json!(tokens));
}
