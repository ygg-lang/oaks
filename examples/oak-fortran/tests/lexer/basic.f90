! Fortran test file for lexer testing
! Modern Fortran (90/95/2003/2008) syntax

! Program structure
program fortran_test
    implicit none
    
    ! Variable declarations
    integer :: i, j, k, n
    real :: x, y, z, pi
    double precision :: dx, dy, dz
    complex :: c1, c2
    logical :: flag, is_valid
    character(len=20) :: name, message
    character(len=1) :: char_val
    
    ! Constants
    integer, parameter :: MAX_SIZE = 1000
    real, parameter :: PI = 3.141592653589793
    double precision, parameter :: EPSILON = 1.0d-10
    
    ! Arrays
    integer, dimension(10) :: int_array
    real, dimension(0:9) :: real_array
    double precision, dimension(5, 5) :: matrix
    character(len=30), dimension(100) :: names
    
    ! Dynamic arrays
    integer, allocatable :: dynamic_array(:)
    real, allocatable :: matrix_2d(:, :)
    
    ! Derived types
type :: person_type
        character(len=50) :: name
        integer :: age
        real :: height
        character(len=100) :: address
    end type person_type
    
    type(person_type) :: person1, person2
    
    ! Interfaces and modules
    interface
        function external_func(x, y) result(z)
            real, intent(in) :: x, y
            real :: z
        end function external_func
    end interface
    
    ! Initialization
    n = 10
    x = 1.5
    y = 2.7
    pi = 3.14159
    flag = .true.
    is_valid = .false.
    name = "Fortran Test"
    
    ! Array initialization
    int_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    real_array = [(0.1 * i, i = 0, 9)]
    
    ! Array operations
    matrix = reshape([1.0, 2.0, 3.0, 4.0, 5.0, &
                      6.0, 7.0, 8.0, 9.0, 10.0, &
                      11.0, 12.0, 13.0, 14.0, 15.0, &
                      16.0, 17.0, 18.0, 19.0, 20.0, &
                      21.0, 22.0, 23.0, 24.0, 25.0], [5, 5])
    
    ! Conditional statements
    if (x > y) then
        z = x
    else if (x < y) then
        z = y
    else
        z = 0.0
    end if
    
    ! Select case (switch)
    select case (n)
        case (1:5)
            print *, "Small number"
        case (6:10)
            print *, "Medium number"
        case default
            print *, "Large number"
    end select
    
    ! Loops
    do i = 1, n
        print *, i, i**2, sqrt(real(i))
    end do
    
    ! While loop
    i = 1
    do while (i <= n)
        print *, i
        i = i + 1
    end do
    
    ! Infinite loop with exit
    do
        if (i > 20) exit
        i = i + 1
    end do
    
    ! Loop with cycle (continue)
    do i = 1, 100
        if (mod(i, 2) == 0) cycle
        print *, i
    end do
    
    ! Nested loops
    do i = 1, 5
        do j = 1, 5
            matrix(i, j) = real(i * j)
        end do
    end do
    
    ! Array operations with where
    where (matrix > 10.0)
        matrix = matrix * 2.0
    elsewhere (matrix < 5.0)
        matrix = matrix + 1.0
    elsewhere
        matrix = matrix - 1.0
    end where
    
    ! Functions and subroutines
    call print_matrix(matrix, 5, 5)
    
    ! Function calls
    z = calculate_area(x, y)
    print *, "Area:", z
    
    ! Intrinsic functions
    print *, "Max:", max(x, y, z)
    print *, "Min:", min(x, y, z)
    print *, "Sum:", sum(real_array)
    print *, "Product:", product(int_array)
    print *, "Dot product:", dot_product(real_array, real_array)
    
    ! Mathematical functions
    print *, "Sin:", sin(pi/4)
    print *, "Cos:", cos(pi/3)
    print *, "Exp:", exp(1.0)
    print *, "Log:", log(2.71828)
    print *, "Sqrt:", sqrt(16.0)
    
    ! String operations
    message = trim(name) // " completed successfully"
    print *, message
    
    ! File operations
    open(unit=10, file='output.txt', status='replace', action='write')
    write(10, *) 'Fortran test output'
    write(10, *) 'Results:'
    write(10, '(A, F10.4)') 'Value of x: ', x
    close(10)
    
    ! Format statements
100 format('The result is: ', F10.4)
    print 100, x
    
    ! Stop program
    stop
    
contains
    ! Internal subroutine
    subroutine print_matrix(mat, rows, cols)
        real, dimension(:,:), intent(in) :: mat
        integer, intent(in) :: rows, cols
        integer :: i, j
        
        do i = 1, rows
            do j = 1, cols
                write(*, '(F8.2, 1X)', advance='no') mat(i, j)
            end do
            print *
        end do
    end subroutine print_matrix
    
    ! Internal function
    real function calculate_area(length, width) result(area)
        real, intent(in) :: length, width
        area = length * width
    end function calculate_area
    
end program fortran_test

! Separate module
module math_utils
    implicit none
    private
    public :: gcd, lcm, is_prime, factorial
    
contains
    
    function gcd(a, b) result(result)
        integer, intent(in) :: a, b
        integer :: result
        integer :: temp, x, y
        
        x = abs(a)
        y = abs(b)
        
        do while (y /= 0)
            temp = mod(x, y)
            x = y
            y = temp
        end do
        
        result = x
    end function gcd
    
    function lcm(a, b) result(result)
        integer, intent(in) :: a, b
        integer :: result
        result = abs(a * b) / gcd(a, b)
    end function lcm
    
    function is_prime(n) result(result)
        integer, intent(in) :: n
        logical :: result
        integer :: i
        
        if (n <= 1) then
            result = .false.
            return
        end if
        
        do i = 2, int(sqrt(real(n)))
            if (mod(n, i) == 0) then
                result = .false.
                return
            end if
        end do
        
        result = .true.
    end function is_prime
    
    recursive function factorial(n) result(result)
        integer, intent(in) :: n
        integer :: result
        
        if (n <= 1) then
            result = 1
        else
            result = n * factorial(n - 1)
        end if
    end function factorial
    
end module math_utils