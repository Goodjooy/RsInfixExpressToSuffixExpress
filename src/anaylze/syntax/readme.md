 # 生成式

stmt ->   expr ;
        | ifexpr
        | forexpr
        | foreachexpr
        | whileexpr
        | matchexpr


block -> { stmts }

stmts -> stmt stmts
        | Nil

ifexpr -> if ( expr ) stmts ifsubexpr ifendexpr

ifsubexpr -> else if (expr)stmts
            | Nil
ifendexpr -> else stmts
            | Nil

forexpr -> for( optexpr ; optexpr ; optexpr)stmts

foreachexpr -> foreach ( expr : expr)stmts

whileexpr -> while ( expr )stmts

matchexpr -> match expr { matcharms }

matcharm -> expr => block,  

matcharms -> matcharm matcharms

--

expr -> item subexpr  

subexpr -> + item subexpr
         | - item subexpr
         | Nil (stop sign)

item -> factor subitem

subitem -> * factor subitem
         | / factor subitem
         | Nil (stop sign)

factor -> digit
        | ( expr )

optexpr -> Nil (stop sign)
         | expr
