#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>


typedef struct Number {
  char *value;
  uint row;
  uint start;
  uint stop;
} Number;

typedef struct NumberList {
  Number head;
  struct NumberList *tail;
} NumberList;

NumberList *addNumber (Number number, NumberList *numberList) {
    NumberList *newList = malloc(sizeof(NumberList));
    newList->head = number;
    newList->tail = numberList;
    return newList;
};

typedef struct Symbol {
  uint row;
  uint col;
} Symbol;

typedef struct SymbolList {
  Symbol head;
  struct SymbolList *tail;
} SymbolList;

SymbolList *addSymbol (Symbol symbol, SymbolList *symbolList) {
    SymbolList *newList = malloc(sizeof(SymbolList));
    newList->head = symbol;
    newList->tail = symbolList;
    return newList;
};

bool isPartNumber (Number number, SymbolList *symbolList) {
  while (symbolList != NULL) {
    Symbol symbol = symbolList->head;
    if (
      symbol.row + 1 >= number.row &&
      symbol.row <= number.row + 1 &&
      symbol.col + 1 >= number.start &&
      symbol.col <= number.stop + 1
    ) {
      return true;
    }
    symbolList = symbolList->tail;
  }

  return false;
}

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    return 1;
  }

  FILE *file;
  
  file = fopen(argv[1], "r");

  if (file == NULL)
    return 1;

  char *line;
  size_t lineLength;
  size_t lineSize;

  int rowsCount = 0;

  Number *currentNumber = NULL;

  NumberList *numberList = NULL;
  SymbolList *symbolList = NULL;

  while ((getline(&line, &lineSize, file) != -1)) {
    lineLength = strlen(line);

    for (uint i = 0; i < lineLength; i++) {
      char currentChar = line [i];

      if (isdigit(currentChar)) {
        if (currentNumber == NULL) {
          currentNumber = malloc(sizeof(Number));
          currentNumber->value = malloc(sizeof(char) * 2);
          currentNumber->value[0] = currentChar;
          currentNumber->row = rowsCount;
          currentNumber->start=i;
          currentNumber->stop=i;
        } else {
          currentNumber->stop=i;
          char *newVal = malloc(strlen(currentNumber->value) + 2);
          strcpy(newVal, currentNumber->value);
          char *newChar = malloc(sizeof(char) * 2);
          newChar[0] = currentChar;
          strcat(newVal, newChar);
          currentNumber->value = newVal;
        }
        continue;
      }

      if (currentNumber != NULL) {
        numberList = addNumber(*currentNumber, numberList);
        currentNumber = NULL;
      }

      if (currentChar == '.' || currentChar == '\n') {
        continue;
      }

      Symbol *newSymbol = malloc(sizeof(Symbol));
      newSymbol->row = rowsCount;
      newSymbol->col = i;
      symbolList = addSymbol(*newSymbol, symbolList);
    }
    rowsCount++;
  }

  int sum = 0;

  NumberList *currentNumberList = numberList;
  while (currentNumberList != NULL) {
    if (isPartNumber(currentNumberList->head, symbolList)) {
      sum += atoi(currentNumberList->head.value);
    }

    currentNumberList = currentNumberList->tail;
  }

  printf("%d\n", sum);
}

