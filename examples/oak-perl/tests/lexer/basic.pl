#!/usr/bin/perl
use strict;
use warnings;
use feature 'say';
use Data::Dumper;
use List::Util qw(sum max min first);
use Scalar::Util qw(looks_like_number blessed);

# Package definition
package Person;

sub new {
    my ($class, %args) = @_;
    my $self = {
        name => $args{name} || 'Unknown',
        age  => $args{age}  || 0,
        email => $args{email} || '',
    };
    bless $self, $class;
    return $self;
}

sub get_name {
    my $self = shift;
    return $self->{name};
}

sub set_name {
    my ($self, $name) = @_;
    $self->{name} = $name;
}

sub get_age {
    my $self = shift;
    return $self->{age};
}

sub set_age {
    my ($self, $age) = @_;
    if (looks_like_number($age) && $age >= 0) {
        $self->{age} = $age;
    } else {
        die "Invalid age: $age";
    }
}

sub get_email {
    my $self = shift;
    return $self->{email};
}

sub set_email {
    my ($self, $email) = @_;
    if ($email =~ /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/) {
        $self->{email} = $email;
    } else {
        die "Invalid email format: $email";
    }
}

sub to_string {
    my $self = shift;
    return sprintf("Person(name='%s', age=%d, email='%s')", 
                   $self->{name}, $self->{age}, $self->{email});
}

sub is_adult {
    my $self = shift;
    return $self->{age} >= 18;
}

# Employee class inheriting from Person
package Employee;
use parent 'Person';

sub new {
    my ($class, %args) = @_;
    my $self = $class->SUPER::new(%args);
    $self->{employee_id} = $args{employee_id} || 0;
    $self->{department} = $args{department} || 'Unknown';
    $self->{salary} = $args{salary} || 0;
    return $self;
}

sub get_employee_id {
    my $self = shift;
    return $self->{employee_id};
}

sub get_department {
    my $self = shift;
    return $self->{department};
}

sub set_department {
    my ($self, $department) = @_;
    $self->{department} = $department;
}

sub get_salary {
    my $self = shift;
    return $self->{salary};
}

sub set_salary {
    my ($self, $salary) = @_;
    if (looks_like_number($salary) && $salary >= 0) {
        $self->{salary} = $salary;
    } else {
        die "Invalid salary: $salary";
    }
}

sub to_string {
    my $self = shift;
    return sprintf("Employee(name='%s', age=%d, email='%s', id=%d, dept='%s', salary=%.2f)", 
                   $self->{name}, $self->{age}, $self->{email}, 
                   $self->{employee_id}, $self->{department}, $self->{salary});
}

# Main package
package main;

# Variables and data types
my $name = "John Doe";
my $age = 30;
my $height = 5.9;
my $is_married = 1;  # Boolean (true)
my $spouse = undef;  # Undefined

# Arrays
my @numbers = (1, 2, 3, 4, 5);
my @fruits = qw(apple banana cherry date elderberry);
my @mixed = (1, "hello", 3.14, "world");

# Hashes
my %person = (
    name => "Alice",
    age => 25,
    city => "New York",
    hobbies => ["reading", "swimming", "coding"]
);

my %colors = (
    red   => "#FF0000",
    green => "#00FF00",
    blue  => "#0000FF",
    yellow => "#FFFF00"
);

# References
my $array_ref = \@numbers;
my $hash_ref = \%person;
my $scalar_ref = \$name;

# Anonymous references
my $anon_array = [1, 2, 3, 4, 5];
my $anon_hash = {
    x => 10,
    y => 20,
    z => 30
};

# Subroutines
sub greet {
    my ($name, $greeting) = @_;
    $greeting ||= "Hello";
    return "$greeting, $name!";
}

sub factorial {
    my $n = shift;
    return 1 if $n <= 1;
    return $n * factorial($n - 1);
}

sub fibonacci {
    my $n = shift;
    return $n if $n <= 1;
    return fibonacci($n - 1) + fibonacci($n - 2);
}

sub sum_array {
    my @arr = @_;
    my $total = 0;
    $total += $_ for @arr;
    return $total;
}

sub find_max {
    my @arr = @_;
    return undef unless @arr;
    my $max = $arr[0];
    for my $num (@arr) {
        $max = $num if $num > $max;
    }
    return $max;
}

# File operations
sub read_file {
    my $filename = shift;
    open my $fh, '<', $filename or die "Cannot open file '$filename': $!";
    my @lines = <$fh>;
    close $fh;
    chomp @lines;
    return @lines;
}

sub write_file {
    my ($filename, @lines) = @_;
    open my $fh, '>', $filename or die "Cannot open file '$filename': $!";
    print $fh "$_\n" for @lines;
    close $fh;
}

# Regular expressions
sub validate_email {
    my $email = shift;
    return $email =~ /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;
}

sub extract_numbers {
    my $text = shift;
    my @numbers = $text =~ /(\d+)/g;
    return @numbers;
}

sub replace_words {
    my ($text, $old_word, $new_word) = @_;
    $text =~ s/\b$old_word\b/$new_word/g;
    return $text;
}

# Control structures
sub demonstrate_loops {
    say "=== Loop Demonstrations ===";
    
    # For loop
    say "For loop (1 to 5):";
    for my $i (1..5) {
        print "$i ";
    }
    say "";
    
    # While loop
    say "While loop (countdown from 5):";
    my $count = 5;
    while ($count > 0) {
        print "$count ";
        $count--;
    }
    say "";
    
    # Foreach loop
    say "Foreach loop (fruits):";
    foreach my $fruit (@fruits) {
        print "$fruit ";
    }
    say "";
    
    # Map function
    say "Map function (squares):";
    my @squares = map { $_ * $_ } (1..5);
    say join(", ", @squares);
    
    # Grep function
    say "Grep function (even numbers):";
    my @evens = grep { $_ % 2 == 0 } (1..10);
    say join(", ", @evens);
}

sub demonstrate_conditionals {
    my $score = shift || 85;
    
    say "=== Conditional Demonstrations ===";
    say "Score: $score";
    
    # If-elsif-else
    if ($score >= 90) {
        say "Grade: A";
    } elsif ($score >= 80) {
        say "Grade: B";
    } elsif ($score >= 70) {
        say "Grade: C";
    } elsif ($score >= 60) {
        say "Grade: D";
    } else {
        say "Grade: F";
    }
    
    # Ternary operator
    my $pass_fail = $score >= 60 ? "Pass" : "Fail";
    say "Result: $pass_fail";
    
    # Unless
    unless ($score < 60) {
        say "Student passed the exam";
    }
}

# Error handling
sub safe_divide {
    my ($a, $b) = @_;
    
    eval {
        die "Division by zero" if $b == 0;
        return $a / $b;
    };
    
    if ($@) {
        warn "Error in division: $@";
        return undef;
    }
}

# String operations
sub demonstrate_strings {
    say "=== String Demonstrations ===";
    
    my $str = "  Hello, World!  ";
    say "Original: '$str'";
    
    # String methods
    say "Length: " . length($str);
    say "Uppercase: " . uc($str);
    say "Lowercase: " . lc($str);
    say "Trimmed: '" . trim($str) . "'";
    say "Substring: " . substr($str, 2, 5);
    say "Index of 'World': " . index($str, 'World');
    
    # String interpolation
    my $name = "Alice";
    my $age = 30;
    say "Interpolation: $name is $age years old";
    
    # Here documents
    my $poem = <<'END_POEM';
Roses are red,
Violets are blue,
Perl is powerful,
And so are you!
END_POEM
    
    say "Poem:\n$poem";
}

sub trim {
    my $str = shift;
    $str =~ s/^\s+|\s+$//g;
    return $str;
}

# Hash operations
sub demonstrate_hashes {
    say "=== Hash Demonstrations ===";
    
    my %student = (
        name => "Bob",
        age => 22,
        major => "Computer Science",
        gpa => 3.8
    );
    
    # Accessing hash elements
    say "Student name: $student{name}";
    say "Student GPA: $student{gpa}";
    
    # Adding new elements
    $student{graduation_year} = 2024;
    $student{honors} = ["Dean's List", "Magna Cum Laude"];
    
    # Iterating over hash
    say "Student information:";
    for my $key (sort keys %student) {
        my $value = $student{$key};
        if (ref($value) eq 'ARRAY') {
            say "  $key: " . join(", ", @$value);
        } else {
            say "  $key: $value";
        }
    }
    
    # Hash slices
    my @info = @student{qw(name age major)};
    say "Basic info: " . join(", ", @info);
}

# Array operations
sub demonstrate_arrays {
    say "=== Array Demonstrations ===";
    
    my @numbers = (10, 5, 8, 3, 12, 7);
    say "Original array: " . join(", ", @numbers);
    
    # Array functions
    push @numbers, 15;
    say "After push: " . join(", ", @numbers);
    
    my $last = pop @numbers;
    say "Popped: $last, Array: " . join(", ", @numbers);
    
    unshift @numbers, 1;
    say "After unshift: " . join(", ", @numbers);
    
    my $first = shift @numbers;
    say "Shifted: $first, Array: " . join(", ", @numbers);
    
    # Sorting
    my @sorted = sort { $a <=> $b } @numbers;
    say "Sorted: " . join(", ", @sorted);
    
    my @reversed = reverse @numbers;
    say "Reversed: " . join(", ", @reversed);
    
    # Array statistics
    say "Sum: " . sum(@numbers);
    say "Max: " . max(@numbers);
    say "Min: " . min(@numbers);
    say "Count: " . scalar(@numbers);
}

# Main execution
sub main {
    say "=== Perl Test Script ===";
    
    # Create objects
    my $person = Person->new(
        name => "John Smith",
        age => 35,
        email => "john.smith@example.com"
    );
    
    my $employee = Employee->new(
        name => "Jane Doe",
        age => 28,
        email => "jane.doe@company.com",
        employee_id => 12345,
        department => "Engineering",
        salary => 75000
    );
    
    say $person->to_string();
    say $employee->to_string();
    
    # Test methods
    say "Is person adult? " . ($person->is_adult() ? "Yes" : "No");
    
    # Function calls
    say greet("Alice");
    say greet("Bob", "Hi");
    
    say "Factorial of 5: " . factorial(5);
    say "Fibonacci of 10: " . fibonacci(10);
    
    # Demonstrations
    demonstrate_loops();
    demonstrate_conditionals(92);
    demonstrate_strings();
    demonstrate_hashes();
    demonstrate_arrays();
    
    # Regular expressions
    my $email = "test@example.com";
    say "Email '$email' is " . (validate_email($email) ? "valid" : "invalid");
    
    my $text = "There are 123 apples and 456 oranges";
    my @nums = extract_numbers($text);
    say "Numbers found: " . join(", ", @nums);
    
    # Error handling
    my $result = safe_divide(10, 2);
    say "10 / 2 = " . (defined $result ? $result : "undefined");
    
    $result = safe_divide(10, 0);
    say "10 / 0 = " . (defined $result ? $result : "undefined");
    
    say "=== End of Test Script ===";
}

# Run main function
main() if __FILE__ eq $0;

1;  # Return true for module