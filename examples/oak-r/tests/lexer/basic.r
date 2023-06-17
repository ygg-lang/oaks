# R test file for lexer testing

# Load required libraries
library(ggplot2)
library(dplyr)
library(tidyr)
library(readr)
library(stringr)

# Constants and variables
PI <- 3.14159
MAX_ITERATIONS <- 1000
DEFAULT_SEED <- 42

# Set random seed for reproducibility
set.seed(DEFAULT_SEED)

# Basic data types
numeric_var <- 42.5
integer_var <- 42L
character_var <- "Hello, R!"
logical_var <- TRUE
complex_var <- 3 + 4i
factor_var <- factor(c("low", "medium", "high"), levels = c("low", "medium", "high"))

# Vectors
numeric_vector <- c(1, 2, 3, 4, 5)
character_vector <- c("apple", "banana", "cherry", "date")
logical_vector <- c(TRUE, FALSE, TRUE, FALSE)
named_vector <- c(a = 1, b = 2, c = 3)

# Sequences
seq1 <- 1:10
seq2 <- seq(0, 1, by = 0.1)
seq3 <- seq(0, 1, length.out = 11)
rep1 <- rep(1:3, times = 3)
rep2 <- rep(1:3, each = 3)

# Matrices
matrix1 <- matrix(1:12, nrow = 3, ncol = 4)
matrix2 <- matrix(1:12, nrow = 3, ncol = 4, byrow = TRUE)
identity_matrix <- diag(3)

# Arrays
array1 <- array(1:24, dim = c(2, 3, 4))

# Lists
list1 <- list(
  numbers = 1:5,
  characters = c("a", "b", "c"),
  logical = c(TRUE, FALSE),
  nested = list(x = 1, y = 2)
)

# Data frames
df <- data.frame(
  id = 1:10,
  name = paste("Person", 1:10),
  age = sample(18:65, 10, replace = TRUE),
  salary = runif(10, 30000, 100000),
  department = sample(c("HR", "IT", "Finance", "Marketing"), 10, replace = TRUE),
  stringsAsFactors = FALSE
)

# Tibbles (modern data frames)
tbl <- tibble(
  x = 1:10,
  y = x^2,
  z = letters[1:10]
)

# Functions
# Simple function
square <- function(x) {
  return(x^2)
}

# Function with default parameters
greet <- function(name, greeting = "Hello") {
  paste(greeting, name, "!")
}

# Function with multiple return values
calculate_stats <- function(x) {
  list(
    mean = mean(x, na.rm = TRUE),
    median = median(x, na.rm = TRUE),
    sd = sd(x, na.rm = TRUE),
    min = min(x, na.rm = TRUE),
    max = max(x, na.rm = TRUE)
  )
}

# Recursive function
factorial <- function(n) {
  if (n <= 1) {
    return(1)
  } else {
    return(n * factorial(n - 1))
  }
}

# Fibonacci function
fibonacci <- function(n) {
  if (n <= 1) {
    return(n)
  } else {
    return(fibonacci(n - 1) + fibonacci(n - 2))
  }
}

# Function with error handling
safe_divide <- function(a, b) {
  tryCatch({
    if (b == 0) {
      stop("Division by zero is not allowed")
    }
    return(a / b)
  }, error = function(e) {
    warning("Error in division: ", e$message)
    return(NA)
  })
}

# Higher-order functions
apply_function <- function(x, fun) {
  fun(x)
}

# Anonymous functions (lambda)
square_lambda <- function(x) x^2
add_lambda <- function(x, y) x + y

# Control structures
# If-else statements
check_number <- function(x) {
  if (x > 0) {
    return("positive")
  } else if (x < 0) {
    return("negative")
  } else {
    return("zero")
  }
}

# For loops
for (i in 1:5) {
  print(paste("Iteration:", i))
}

# While loops
counter <- 1
while (counter <= 5) {
  print(paste("Counter:", counter))
  counter <- counter + 1
}

# Repeat loops
counter <- 1
repeat {
  print(paste("Repeat counter:", counter))
  counter <- counter + 1
  if (counter > 5) break
}

# Switch statement
get_day_type <- function(day) {
  switch(day,
    "Monday" = "Weekday",
    "Tuesday" = "Weekday",
    "Wednesday" = "Weekday",
    "Thursday" = "Weekday",
    "Friday" = "Weekday",
    "Saturday" = "Weekend",
    "Sunday" = "Weekend",
    "Unknown"
  )
}

# Apply family functions
# lapply - returns list
list_result <- lapply(1:5, square)

# sapply - returns vector/matrix
vector_result <- sapply(1:5, square)

# mapply - multiple arguments
mapply_result <- mapply(function(x, y) x + y, 1:5, 6:10)

# apply - for matrices/arrays
matrix_sums <- apply(matrix1, 1, sum)  # row sums
matrix_means <- apply(matrix1, 2, mean)  # column means

# Data manipulation with dplyr
df_processed <- df %>%
  filter(age >= 25) %>%
  mutate(
    age_group = case_when(
      age < 30 ~ "Young",
      age < 50 ~ "Middle",
      TRUE ~ "Senior"
    ),
    salary_category = ifelse(salary > 60000, "High", "Low")
  ) %>%
  group_by(department) %>%
  summarise(
    count = n(),
    avg_age = mean(age),
    avg_salary = mean(salary),
    .groups = "drop"
  ) %>%
  arrange(desc(avg_salary))

# String operations
text <- "Hello, World! This is R programming."
upper_text <- toupper(text)
lower_text <- tolower(text)
substr_text <- substr(text, 1, 5)
split_text <- strsplit(text, " ")[[1]]
replaced_text <- gsub("World", "R", text)

# Regular expressions
pattern <- "\\b[A-Za-z]+\\b"
words <- str_extract_all(text, pattern)[[1]]
word_count <- str_count(text, "\\b\\w+\\b")

# Date and time operations
current_date <- Sys.Date()
current_time <- Sys.time()
formatted_date <- format(current_date, "%Y-%m-%d")
formatted_time <- format(current_time, "%Y-%m-%d %H:%M:%S")

# Create date sequences
date_seq <- seq(as.Date("2023-01-01"), as.Date("2023-12-31"), by = "month")

# Statistical functions
# Generate random data
normal_data <- rnorm(100, mean = 50, sd = 10)
uniform_data <- runif(100, min = 0, max = 100)
poisson_data <- rpois(100, lambda = 5)

# Descriptive statistics
stats_summary <- list(
  mean = mean(normal_data),
  median = median(normal_data),
  sd = sd(normal_data),
  var = var(normal_data),
  min = min(normal_data),
  max = max(normal_data),
  quantiles = quantile(normal_data, probs = c(0.25, 0.5, 0.75))
)

# Correlation and covariance
x <- rnorm(50)
y <- 2 * x + rnorm(50, sd = 0.5)
correlation <- cor(x, y)
covariance <- cov(x, y)

# Linear regression
lm_model <- lm(y ~ x)
model_summary <- summary(lm_model)
predictions <- predict(lm_model, newdata = data.frame(x = c(0, 1, 2)))

# Hypothesis testing
# t-test
group1 <- rnorm(30, mean = 10, sd = 2)
group2 <- rnorm(30, mean = 12, sd = 2)
t_test_result <- t.test(group1, group2)

# Chi-square test
observed <- c(20, 30, 25, 25)
expected <- c(25, 25, 25, 25)
chi_test_result <- chisq.test(observed, p = expected/sum(expected))

# Plotting with base R
# Scatter plot
plot(x, y, main = "Scatter Plot", xlab = "X values", ylab = "Y values", col = "blue")
abline(lm_model, col = "red")

# Histogram
hist(normal_data, main = "Histogram of Normal Data", xlab = "Values", col = "lightblue")

# Box plot
boxplot(salary ~ department, data = df, main = "Salary by Department")

# Plotting with ggplot2
scatter_plot <- ggplot(data.frame(x, y), aes(x = x, y = y)) +
  geom_point(color = "blue", alpha = 0.7) +
  geom_smooth(method = "lm", color = "red") +
  labs(title = "Scatter Plot with Regression Line",
       x = "X values", y = "Y values") +
  theme_minimal()

histogram_plot <- ggplot(data.frame(values = normal_data), aes(x = values)) +
  geom_histogram(bins = 20, fill = "lightblue", color = "black", alpha = 0.7) +
  labs(title = "Histogram of Normal Data",
       x = "Values", y = "Frequency") +
  theme_minimal()

boxplot_ggplot <- ggplot(df, aes(x = department, y = salary, fill = department)) +
  geom_boxplot(alpha = 0.7) +
  labs(title = "Salary Distribution by Department",
       x = "Department", y = "Salary") +
  theme_minimal() +
  theme(legend.position = "none")

# Data reshaping
# Wide to long
df_long <- df %>%
  select(id, name, age, salary) %>%
  pivot_longer(cols = c(age, salary), names_to = "metric", values_to = "value")

# Long to wide
df_wide <- df_long %>%
  pivot_wider(names_from = metric, values_from = value)

# File I/O operations
# Write data to CSV
write_csv(df, "sample_data.csv")

# Read data from CSV (commented out as file may not exist)
# df_read <- read_csv("sample_data.csv")

# Write to RDS (R data format)
saveRDS(df, "sample_data.rds")

# Read from RDS (commented out as file may not exist)
# df_rds <- readRDS("sample_data.rds")

# Environment and workspace operations
# List objects in environment
objects_list <- ls()

# Get object information
object_info <- sapply(objects_list, function(x) class(get(x)))

# Memory usage
memory_usage <- object.size(df)

# Working directory
current_wd <- getwd()

# Session information
session_info <- sessionInfo()

# Custom S3 class
# Define a simple class
create_person <- function(name, age) {
  person <- list(name = name, age = age)
  class(person) <- "person"
  return(person)
}

# Method for print
print.person <- function(x, ...) {
  cat("Person:", x$name, "Age:", x$age, "\n")
}

# Method for summary
summary.person <- function(object, ...) {
  cat("Person Summary:\n")
  cat("Name:", object$name, "\n")
  cat("Age:", object$age, "\n")
  cat("Adult:", ifelse(object$age >= 18, "Yes", "No"), "\n")
}

# Create person object
john <- create_person("John Doe", 30)

# Package development functions
# Create documentation
#' Calculate the area of a circle
#'
#' @param radius The radius of the circle
#' @return The area of the circle
#' @examples
#' circle_area(5)
#' @export
circle_area <- function(radius) {
  if (radius < 0) {
    stop("Radius must be non-negative")
  }
  return(pi * radius^2)
}

# Unit testing (using testthat syntax)
# test_that("circle_area calculates correctly", {
#   expect_equal(circle_area(1), pi)
#   expect_equal(circle_area(0), 0)
#   expect_error(circle_area(-1))
# })

# Parallel processing
# library(parallel)
# cl <- makeCluster(detectCores() - 1)
# parallel_result <- parLapply(cl, 1:10, square)
# stopCluster(cl)

# Advanced data structures
# Environment
env <- new.env()
env$x <- 10
env$y <- 20

# Closure example
make_counter <- function() {
  count <- 0
  function() {
    count <<- count + 1
    count
  }
}

counter1 <- make_counter()
counter2 <- make_counter()

# R6 class (object-oriented programming)
# library(R6)
# Person <- R6Class("Person",
#   public = list(
#     name = NULL,
#     age = NULL,
#     initialize = function(name, age) {
#       self$name <- name
#       self$age <- age
#     },
#     greet = function() {
#       paste("Hello, my name is", self$name)
#     }
#   )
# )

# Functional programming
# Compose functions
compose <- function(f, g) {
  function(x) f(g(x))
}

# Partial application
partial <- function(f, ...) {
  args <- list(...)
  function(...) do.call(f, c(args, list(...)))
}

# Example usage
add_ten <- partial(`+`, 10)
multiply_by_two <- partial(`*`, 2)
add_ten_then_double <- compose(multiply_by_two, add_ten)

# Print results
cat("=== R Test Script Results ===\n")
cat("Square of 5:", square(5), "\n")
cat("Greeting:", greet("Alice"), "\n")
cat("Factorial of 5:", factorial(5), "\n")
cat("Fibonacci of 8:", fibonacci(8), "\n")
cat("Safe division 10/2:", safe_divide(10, 2), "\n")
cat("Safe division 10/0:", safe_divide(10, 0), "\n")
cat("Check number 5:", check_number(5), "\n")
cat("Day type for Monday:", get_day_type("Monday"), "\n")
cat("Correlation between x and y:", correlation, "\n")
cat("Circle area with radius 3:", circle_area(3), "\n")
cat("Counter 1 calls:", counter1(), counter1(), counter1(), "\n")
cat("Add ten then double 5:", add_ten_then_double(5), "\n")

# Print data frame summary
cat("\nData frame summary:\n")
print(head(df))
cat("\nProcessed data summary:\n")
print(df_processed)

cat("\n=== End of R Test Script ===\n")