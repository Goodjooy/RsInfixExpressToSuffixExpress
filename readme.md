# 中缀表达式转后缀表达式

* 句法分析器
    * 终结符号集合={'(',')',0-9,'+','-'}
    * 开始标记=exp
    * 产生式=
    ```
        //优先级 0
        factor-> digit | (exp)
        //优先级 1
        /*
        item-> item * factor
        item-> item / factor
        item-> factor
        */
        // 优先级 2
        exp-> exp + item
        exp-> exp - item
        exp-> item

        digit -> (0|1|2|3|4|5|6|7|8|9)(0|1|2|3|4|5|6|7|8|9)*
    ```
    
