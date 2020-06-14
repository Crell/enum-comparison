#include<stdio.h>

typedef enum { Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday } Day;


void printer(Day d) {
  printf("The day is: %d\n", d);
}


int main() {

  Day d = Tuesday;

  printer(d);

  printer(4);

   return 0;
}
