       IDENTIFICATION DIVISION.
       PROGRAM-ID. BASIC-TEST.
       AUTHOR. TEST-AUTHOR.
       DATE-WRITTEN. 2024-01-01.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       SOURCE-COMPUTER. IBM-370.
       OBJECT-COMPUTER. IBM-370.

       DATA DIVISION.
       FILE SECTION.
       WORKING-STORAGE SECTION.
       01  CUSTOMER-RECORD.
           05  CUSTOMER-ID        PIC 9(5).
           05  CUSTOMER-NAME      PIC X(30).
           05  CUSTOMER-BALANCE   PIC 9(7)V99.

       01  WS-VARIABLES.
           05  WS-COUNTER         PIC 9(3) VALUE 0.
           05  WS-TOTAL           PIC 9(7)V99 VALUE 0.
           05  WS-MESSAGE         PIC X(50).

       77  WS-STATUS             PIC X(2).
       77  WS-RESULT             PIC 9(3).

       PROCEDURE DIVISION.
       MAIN-PROCEDURE.
           PERFORM INITIALIZE-PROGRAM
           PERFORM PROCESS-CUSTOMERS
           PERFORM DISPLAY-RESULTS
           STOP RUN.

       INITIALIZE-PROGRAM.
           MOVE 0 TO WS-COUNTER
           MOVE 0 TO WS-TOTAL
           MOVE "PROGRAM STARTED" TO WS-MESSAGE
           DISPLAY WS-MESSAGE
           .

       PROCESS-CUSTOMERS.
           PERFORM VARYING WS-COUNTER FROM 1 BY 1
                   UNTIL WS-COUNTER > 10
               COMPUTE WS-TOTAL = WS-TOTAL + 100.50
               DISPLAY "Processing customer: " WS-COUNTER
           END-PERFORM
           .

       DISPLAY-RESULTS.
           MOVE "Processing complete" TO WS-MESSAGE
           DISPLAY WS-MESSAGE
           DISPLAY "Total processed: " WS-TOTAL
           DISPLAY "Customer count: " WS-COUNTER
           .

       END PROGRAM BASIC-TEST.