       IDENTIFICATION DIVISION.
       PROGRAM-ID.  PAYROLL-SYSTEM.
       AUTHOR.      OAK-TESTER.
       INSTALLATION. DATA-CENTER-1.
       DATE-WRITTEN. 2023-10-27.
       DATE-COMPILED. 2023-10-28.
       SECURITY.    CONFIDENTIAL.

       ENVIRONMENT DIVISION.
       CONFIGURATION SECTION.
       SOURCE-COMPUTER. IBM-370.
       OBJECT-COMPUTER. IBM-370.
       SPECIAL-NAMES.
           C01 IS TO-TOP-OF-PAGE.

       INPUT-OUTPUT SECTION.
       FILE-CONTROL.
           SELECT EMPLOYEE-FILE ASSIGN TO 'EMP.DAT'
               ORGANIZATION IS LINE SEQUENTIAL.
           SELECT REPORT-FILE   ASSIGN TO 'REPORT.OUT'
               ORGANIZATION IS LINE SEQUENTIAL.

       DATA DIVISION.
       FILE SECTION.
       FD  EMPLOYEE-FILE.
       01  EMPLOYEE-RECORD.
           05  EMP-ID           PIC 9(5).
           05  EMP-NAME         PIC X(20).
           05  EMP-HOURS        PIC 9(3).
           05  EMP-RATE         PIC 9(3)V99.

       FD  REPORT-FILE.
       01  REPORT-LINE          PIC X(80).

       WORKING-STORAGE SECTION.
       01  WS-FLAGS.
           05  EOF-FLAG         PIC X VALUE 'N'.
               88  END-OF-FILE  VALUE 'Y'.
       
       01  WS-CALCS.
           05  WS-GROSS-PAY     PIC 9(5)V99.
           05  WS-TAX           PIC 9(4)V99.
           05  WS-NET-PAY       PIC 9(5)V99.
           05  WS-TOTAL-PAY     PIC 9(7)V99 VALUE 0.

       01  WS-DATE.
           05  WS-YEAR          PIC 9(4).
           05  WS-MONTH         PIC 9(2).
           05  WS-DAY           PIC 9(2).

       01  WS-TABLE-DATA.
           05  WS-TAX-BRACKET OCCURS 5 TIMES INDEXED BY IDX.
               10  LIMIT-AMT    PIC 9(5).
               10  TAX-RATE     PIC V99.

       LINKAGE SECTION.
       01  LS-PARAM             PIC X(10).

       PROCEDURE DIVISION USING LS-PARAM.
       000-MAIN-CONTROL.
           PERFORM 100-INITIALIZE.
           PERFORM 200-PROCESS-FILE
               UNTIL END-OF-FILE.
           PERFORM 300-TERMINATE.
           STOP RUN.

       100-INITIALIZE.
           OPEN INPUT EMPLOYEE-FILE
                OUTPUT REPORT-FILE.
           READ EMPLOYEE-FILE
               AT END MOVE 'Y' TO EOF-FLAG.
           MOVE FUNCTION CURRENT-DATE(1:8) TO WS-DATE.
           
           * Initialize Tax Table
           MOVE 10000 TO LIMIT-AMT(1)
           MOVE 0.10  TO TAX-RATE(1)
           MOVE 20000 TO LIMIT-AMT(2)
           MOVE 0.15  TO TAX-RATE(2).

       200-PROCESS-FILE.
           COMPUTE WS-GROSS-PAY = EMP-HOURS * EMP-RATE.
           
           IF WS-GROSS-PAY > 5000
               COMPUTE WS-TAX = WS-GROSS-PAY * 0.20
           ELSE
               COMPUTE WS-TAX = WS-GROSS-PAY * 0.10
           END-IF.
           
           SUBTRACT WS-TAX FROM WS-GROSS-PAY GIVING WS-NET-PAY.
           ADD WS-NET-PAY TO WS-TOTAL-PAY.
           
           STRING EMP-NAME DELIMITED BY SPACE
                  ' EARNED $'
                  WS-NET-PAY
                  INTO REPORT-LINE.
           
           WRITE REPORT-LINE AFTER ADVANCING 1 LINE.
           
           READ EMPLOYEE-FILE
               AT END SET END-OF-FILE TO TRUE.

       300-TERMINATE.
           DISPLAY 'TOTAL PAYROLL: ' WS-TOTAL-PAY.
           CLOSE EMPLOYEE-FILE
                 REPORT-FILE.
           EXIT PROGRAM.
