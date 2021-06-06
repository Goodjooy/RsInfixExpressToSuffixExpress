expr -> item subexpr 

subexpr -> + item subexpr
      | = item subexpr
      | Nil

item -> factor subitem

subitem -> * factor subitem
         | / factor subitem

factor -> digit
        | ( expr )

optexpr -> Nil
         | expr


