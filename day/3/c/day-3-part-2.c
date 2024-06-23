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

bool isPartNumberOf (Number number, Symbol symbol) {
  return (
    symbol.row + 1 >= number.row &&
    symbol.row <= number.row + 1 &&
    symbol.col + 1 >= number.start &&
    symbol.col <= number.stop + 1
  );
}

bool isPartNumber (Number number, SymbolList *symbolList) {
  while (symbolList != NULL) {
    Symbol symbol = symbolList->head;
    if (isPartNumberOf(number, symbol)) {
      return true;
    }
    symbolList = symbolList->tail;
  }

  return false;
}

int getGearRatio (Symbol symbol, NumberList *numberList) {
  int count = 0;
  int firstNumber = 0;
  int secondNumber = 0;
  NumberList *currentNumberList = numberList;

  while (currentNumberList != NULL) {
    Number number = currentNumberList->head;
    if (isPartNumberOf(number, symbol)) {
      switch (count) {
        case 0:
          firstNumber = atoi(number.value);
          count++;
          break;
        case 1:
          secondNumber = atoi(number.value);
          count++;
          break;
      }
    }

    currentNumberList = currentNumberList->tail;
  }
  int ratio = (count == 2)
    ? firstNumber * secondNumber
    : 0;

  return ratio;
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

      if (currentChar == '*') {
        Symbol *newSymbol = malloc(sizeof(Symbol));
        newSymbol->row = rowsCount;
        newSymbol->col = i;
        symbolList = addSymbol(*newSymbol, symbolList);
      }
    }
    rowsCount++;
  }

  int sum = 0;

  SymbolList *currentSymbolList = symbolList;
  while (currentSymbolList != NULL) {
    sum += getGearRatio(currentSymbolList->head, numberList);

    currentSymbolList = currentSymbolList->tail;
  }

  printf("%d\n", sum);
}

