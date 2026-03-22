# Reescrita de Algoritmos em Rust

**Aluno:** Pedro Henrique Alves Barbosa  
**RA:** 123222053  
**Unidade:** UniBH Estoril  
**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Professor:** Alexandre de Oliveira

Atividade de reimplementação em Rust dos algoritmos vistos em Python, com análise de complexidade de cada um.

## Como executar

```bash
cargo run    # roda as demos de todos os exercícios
cargo test   # executa todos os testes
```

---

## Exercício 1 — Verificar Primeiro · O(1)

Retorna o primeiro elemento da lista ou `None` se ela estiver vazia. Não percorre nada — acesso direto ao índice 0, sempre uma única operação.

## Exercício 2 — Somar Lista · O(n)

Percorre a lista uma vez somando todos os elementos em um acumulador. O tempo cresce linearmente com o tamanho da entrada.

## Exercício 3 — Busca Binária · O(log n)

Divide o intervalo de busca ao meio a cada iteração, descartando metade dos elementos. A lista precisa estar ordenada. Para 1 milhão de elementos, são apenas ~20 iterações.

## Exercício 4 — Pares com Soma · O(n²)

Para cada elemento, verifica todos os seguintes em busca de pares que somem ao alvo. Dois loops aninhados resultam em n(n-1)/2 comparações.

## Exercício 5 — Imprimir Pares e Pares · O(n²)

Dois blocos: o primeiro imprime cada elemento (O(n)), o segundo imprime todos os pares possíveis com dois loops aninhados (O(n²)). O termo dominante define a complexidade final.

## Exercício 6 — Potências de Dois · O(log n)

Começa em 1 e dobra o valor a cada passo até alcançar `n`. Como o valor cresce geometricamente, o loop executa apenas log₂(n) vezes.

## Exercício 7 — Fibonacci Recursivo · O(2ⁿ)

Calcula fib(n) chamando fib(n-1) e fib(n-2) recursivamente. Cada chamada gera duas novas, formando uma árvore com ~2ⁿ nós. Não testar com n > 40.

## Exercício 8 — Ordenação Bolha · O(n²)

Compara pares adjacentes repetidamente e troca os fora de ordem. A cada passagem o maior elemento sobe para o fim. Dois loops aninhados com n(n-1)/2 comparações.

## Exercício 9 — Produto de Matrizes · O(n³)

Para cada célula (i, j) da matriz resultado, percorre a linha i de A e a coluna j de B acumulando os produtos. Três loops aninhados resultam em n³ operações.

## Exercício 10 — Merge Sort · O(n log n)

Divide a lista ao meio recursivamente até sublistas de tamanho 1, depois as funde em ordem. São log₂(n) níveis de divisão, cada um com O(n) trabalho de fusão.
