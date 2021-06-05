use std::collections::HashMap;

// 语法分析器
// 分析语法

struct SignTables{
    table : HashMap<(),()>,
    prevent:Option<Box<SignTables>>
}

