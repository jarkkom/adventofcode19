       IDENTIFICATION DIVISION.
       PROGRAM-ID.    DAY01.
       ENVIRONMENT DIVISION.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 MASS PIC 9(9) VALUE 0.
       01 FUEL_SUM PIC 9(9) VALUE 0.
       01 REQUIREDFUEL PIC S9(9) VALUE 0.
       01 TOTALFUEL PIC 9(9) VALUE 0.
       PROCEDURE DIVISION. 
           MAIN.
           INITIALIZE MASS REQUIREDFUEL TOTALFUEL.
           PERFORM UNTIL MASS=1
           ACCEPT MASS FROM SYSIN
           COMPUTE FUEL_SUM ROUNDED MODE IS TRUNCATION = 
    -      (MASS / 3) - 2
           COMPUTE REQUIREDFUEL ROUNDED MODE IS TRUNCATION 
    -      = (FUEL_SUM / 3) - 2
           DISPLAY 'FUEL_SUM ' FUEL_SUM ' REQUIREDFUEL ' REQUIREDFUEL
           PERFORM UNTIL REQUIREDFUEL < 0
           COMPUTE FUEL_SUM = FUEL_SUM + REQUIREDFUEL
           COMPUTE REQUIREDFUEL ROUNDED MODE IS TRUNCATION 
    -      = (REQUIREDFUEL / 3) - 2
           DISPLAY 'FUEL_SUM ' FUEL_SUM ' REQUIREDFUEL ' REQUIREDFUEL
           END-PERFORM
           COMPUTE TOTALFUEL = TOTALFUEL + FUEL_SUM
           DISPLAY 'MASS ' MASS ' FUEL_SUM ' FUEL_SUM
           DISPLAY 'TOTALFUEL ' TOTALFUEL
           END-PERFORM

           DISPLAY 'TOTAL FUEL ' TOTALFUEL .

        STOP RUN.
