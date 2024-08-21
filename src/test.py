def precedence(op):
    if op in ('&', '|'):
        return 2
    if op in ('>', '<'):
        return 1
    return 0

def expression_to_infix(expression) -> list:
    stack = []
    for op in expression:
        if op != ' ':
            stack.append(op)

    return stack

def infix_to_postfix(infix):
    postfix = []
    stack = []

    for op in infix:
        if op.isnumeric():
            postfix.append(op)
        elif op == '(':
            stack.append(op)
        elif op == ')':
            while len(stack) > 0 and stack[-1] != '(':
                postfix.append(stack.pop())
            stack.pop()
        else:
            while len(stack) > 0 and precedence(stack[-1]) >= precedence(op):
                postfix.append(stack.pop())
            stack.append(op)
    
    while len(stack) > 0:
        postfix.append(stack.pop())

    return postfix

def evaluate_infix_logic_expression(expression):
    stack = []

    postfix = infix_to_postfix(expression)
    print(postfix)
    for op in postfix:
        if op.isnumeric():
            stack.append(int(op))
        else:
            operand2 = stack.pop()
            operand1 = stack.pop()

            match op:
                case "<":
                    stack.append(operand1 < operand2)
                case ">":
                    stack.append(operand1 > operand2)
                case "&":
                    stack.append(operand1 and operand2)
                case "|":
                    stack.append(operand1 or operand2)

    return stack.pop()


expr = '3 > 4 | 1 < 2'
infix = expression_to_infix(expr)
result = evaluate_infix_logic_expression(infix)
print(result)
