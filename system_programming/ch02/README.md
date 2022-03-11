cmd: 1
Parser.new("1") => 1) tokenizer = {'1'} 2) tokenizer.iter() = {Num(1), EOF} 3) current_token = Num(1)
parse() => 1) current_token = EOF 2) left_expr_0 = Number(1) 3) DefaultZero < DefaultZero == false 4) return Number(1)

cmd: 1+1
Parser.new("1+1") => 1) tokenizer = {'1','+','1'} 2) tokenizer.iter() = {Num(1), Add, Num(1), EOF} 3) current_token = Num(1)
parse() => 1) current_token = Add 2) left_expr_0 = Number(1) 3) DefaultZero < AddSub == true 4) current_token = Num(1) 5) current_token = EOF 6) left_expr_1 = Number(1) 7) AddSub < DefaultZero == false 8) return Number(1) 9) right_expr_1 = Number(1) 10) right_expr_0 = Add(Number(1),Number(1)) 11) DefaultZero < DefaultZero == false 12) ruturn Add(Number(1),Number(1))

cmd: -1+1
Parser.new("-1+1") => 1) tokenizer = {'-','1','+','1'} 2) tokenizer.iter() = {Subtract, Num(1), Add, Num(1), EOF} 3) current_token = Num(1)
parse() => 1) current_token = Num(1) 2) left_expr_0 = Number(1) 3) AddSub < DefaultZero == false 4) return Number(1) 5) left_expr_1 = Negative(Number(1))
... 6) ruturn Add(Negative(Number(1)), Number(1))
