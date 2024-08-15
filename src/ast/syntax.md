## The syntax of the programming language

**code block:**
```
{
    <body>
}
```
body: the code to be run

**expression:**
*a group of tokens with no consecutive identifiers, for example:*
```
a + b
```
is an expression, but
```
a + b c
```
are *two expressions* - first one, 'a + b', adds 'a' and 'b' together. the second one is just 'c'. another example:
```
a = 123 * b c = x
```
now it's more clear that those are two expressions. when written in multiple lines, it looks way less confusing.


**variable declaration:**
```
var/mut <varName> = <expr>
```
var/mut: either 'var' or 'mut', determines whether the variable is mutable
varName: the name of the variable
expr: the expression to be evaluated and assigned to the variable

**variable assignment:**
```
<varName> <op> = <expr>
```
varName: the name of the variable
op: can be one of the following: "=", "+=", "-=", "/=", "*="
expr: the expression to be evaluated and assigned to the variable

**function declaration:**
```
fun <funcName>(<params,...>) <codeBlock>
```
funcName: the name of the function
params: comma (,) separated list of alphanumeric words (the function's parameters)
codeBlock: the code block to be assigned to the function

**function call:**
```
funcName(<expr,...>)
```
funcName: the name of the function
expr: comma separated list of expressions to be passed as parameters

**if statement:**
```
if <expr> <codeBlock>
```
expr: the expression whose result must be *truthy* (evaluates as true)
codeBlock: the code block to be run if the expression is evaluated as true

**while loop:**
```
while <expr> <codeBlock>
```
expr: the expression whose result must be *truthy* (evaluates as true)
codeBlock: the code block to be run repeatedly as long as the expression is truthy

--
## Math features
Most programming languages lack a lot of things 'math syntax' has, for example '!' for factorial