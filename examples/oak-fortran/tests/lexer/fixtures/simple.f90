program hello
    implicit none
    integer :: i
    real :: x = 3.14
    character(len=20) :: name = "World"
    
    ! This is a comment
    print *, "Hello, ", name
    
    do i = 1, 10
        x = x + 1.0
    end do
    
    if (x > 10.0) then
        print *, "x is greater than 10"
    else
        print *, "x is not greater than 10"
    end if
end program hello