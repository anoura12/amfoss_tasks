#include <cs50.h>
#include <stdio.h>

int main (void)
{
    int i = 0;
    int k = 0; 
    int j = 0;
    int length = 2;
    int height = 0;
    int temp_height = 0;
    
 do{
     height = get_int("Height: ");
       temp_height = height;
 }  
      
 while(height < 0 || height > 23);  
 
 for(i=0;i<height;i++){
    
     for( j=temp_height-1;j>=1;j--){
       printf(" ");
     }
     temp_height = temp_height - 1;
     
         for(k = 1;k<=length;k++){
             printf("#");
         }
         printf("  \n");
        length = length + 1;
     
     }
 }
 
