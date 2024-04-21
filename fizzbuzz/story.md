# Fizzbuzz

## Introdução

João gostaria de ensinar aos seus filhos sobre divisão. Entusiasta por tecnologia, ele descobre sobre o algoritmo "fizzbuzz".

## Desafio 1

Já que ele não sabe programar, ele pede para que você implemente uma função que implemente esse algoritmo. Para tal, ele te passa a seguinte especificação.

### Especificação

Entrada: um número inteiro N
Saída: uma string

O algoritmo deve retornar:
* "fizz" se N é divisível por 3;
* "buzz" se N é divisível por 5;
* "fizzbuzz" se N é divisível por 3 e 5.
* N como string se N não é divisível por 3 ou por 5.

### Objetivo
Implemente o algoritmo usando TDD e testes parametrizados.

## Desafio 2

Após receber sua implementação, João percebeu que na verdade não queria que N pudesse ser negativo ou zero. Após pesquisar um pouco, ele sabe que pode resolver parte do problema a partir de inteiros "unsigned". Portanto, pediu que faça a seguinte modificação:

### Especificação

Entrada: um número inteiro não-negativo N
Saída: uma string

O algoritmo deve retornar:
* "erro" se N é igual a 0;
* "fizz" se N é um inteiro positivo divisível por 3;
* "buzz" se N é um inteiro positivo divisível por 5;
* "fizzbuzz" se N é um inteiro positivo divisível por 3 e 5.
* N como string se N é um inteiro positivo que não é divisível por 3 ou por 5.


### Objetivo
Atualize a sua implementação para respeitar a nova especificação dada por João. Utilize TDD.

## Desafio 3

Após utilizar um pouco a função, João não gostou de utilizar os retornos para o caso de erro quando a entrada é igual a 0. Portanto, ele gostaria que esse caso fosse tratado de forma diferente. Infelizmente, como ele não sabe muito de programação, não pode te especificar tão bem o que quer. Ele te deu a liberdade de mudar a saída do algoritmo, mas isso não significa que você não pode continuar retornando string.

### Especificação

Entrada: um número inteiro não-negativo N
Saída: string ou a critério

O algoritmo ainda deve respeitar a lógica anterior do algoritmo, com possíveis mudanças no tipo específico da saída, mas não pode retornar uma string para o caso em que N é igual a 0.


### Objetivo
Trate o caso de erro da forma que achar melhor.

## Desafio 4

Após a implementação da sua solução, João ainda se encontra insatisfeito. Isso porque, mesmo quando tinha certeza que a sua entrada estava certa, tinha que tratar quaisquer possíveis casos de erro. Sabendo um pouco sobre matemática, ele percebe que a função utilizada até o momento é, de certa forma, uma função parcial. Portanto, ele pede que a última implementação seja uma função total. Ele não se importa se tem de tratar erros antes de chamar o algoritmo -  o importante é que o uso do algoritmo em si seja conveniente, retornando sempre string.

### Especificação

Entrada: a critério, mas deve representar um número inteiro positivo N
Saída: string

O algoritmo ainda deve respeitar a lógica anterior.

### Objetivo
Transforme a função do algoritmo em uma função total.