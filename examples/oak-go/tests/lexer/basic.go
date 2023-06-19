package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log"
	"math"
	"net/http"
	"os"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"time"
)

// Constants
const (
	MaxRetries = 3
	Timeout    = 30 * time.Second
	Version    = "1.0.0"
)

// Custom types
type UserID int64
type Email string
type Status int

// Enums using iota
const (
	StatusPending Status = iota
	StatusActive
	StatusInactive
	StatusSuspended
)

func (s Status) String() string {
	switch s {
	case StatusPending:
		return "pending"
	case StatusActive:
		return "active"
	case StatusInactive:
		return "inactive"
	case StatusSuspended:
		return "suspended"
	default:
		return "unknown"
	}
}

// Interfaces
type Validator interface {
	Validate() error
}

type Serializer interface {
	ToJSON() ([]byte, error)
	FromJSON([]byte) error
}

type Repository interface {
	Save(ctx context.Context, entity interface{}) error
	FindByID(ctx context.Context, id interface{}) (interface{}, error)
	Delete(ctx context.Context, id interface{}) error
}

// Structs
type Address struct {
	Street   string `json:"street"`
	City     string `json:"city"`
	State    string `json:"state"`
	ZipCode  string `json:"zip_code"`
	Country  string `json:"country"`
}

func (a Address) Validate() error {
	if a.Street == "" {
		return errors.New("street is required")
	}
	if a.City == "" {
		return errors.New("city is required")
	}
	if a.ZipCode == "" {
		return errors.New("zip code is required")
	}
	return nil
}

type User struct {
	ID        UserID    `json:"id"`
	Username  string    `json:"username"`
	Email     Email     `json:"email"`
	FirstName string    `json:"first_name"`
	LastName  string    `json:"last_name"`
	Age       int       `json:"age"`
	Status    Status    `json:"status"`
	Address   *Address  `json:"address,omitempty"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
	Tags      []string  `json:"tags"`
	Metadata  map[string]interface{} `json:"metadata"`
}

func NewUser(username string, email Email) *User {
	return &User{
		Username:  username,
		Email:     email,
		Status:    StatusPending,
		CreatedAt: time.Now(),
		UpdatedAt: time.Now(),
		Tags:      make([]string, 0),
		Metadata:  make(map[string]interface{}),
	}
}

func (u *User) Validate() error {
	if u.Username == "" {
		return errors.New("username is required")
	}
	if len(u.Username) < 3 {
		return errors.New("username must be at least 3 characters")
	}
	if u.Email == "" {
		return errors.New("email is required")
	}
	if !isValidEmail(string(u.Email)) {
		return errors.New("invalid email format")
	}
	if u.Age < 0 || u.Age > 150 {
		return errors.New("invalid age")
	}
	if u.Address != nil {
		if err := u.Address.Validate(); err != nil {
			return fmt.Errorf("address validation failed: %w", err)
		}
	}
	return nil
}

func (u *User) ToJSON() ([]byte, error) {
	return json.Marshal(u)
}

func (u *User) FromJSON(data []byte) error {
	return json.Unmarshal(data, u)
}

func (u *User) FullName() string {
	if u.FirstName == "" && u.LastName == "" {
		return u.Username
	}
	return strings.TrimSpace(u.FirstName + " " + u.LastName)
}

func (u *User) IsActive() bool {
	return u.Status == StatusActive
}

func (u *User) AddTag(tag string) {
	for _, t := range u.Tags {
		if t == tag {
			return // Tag already exists
		}
	}
	u.Tags = append(u.Tags, tag)
	u.UpdatedAt = time.Now()
}

func (u *User) RemoveTag(tag string) {
	for i, t := range u.Tags {
		if t == tag {
			u.Tags = append(u.Tags[:i], u.Tags[i+1:]...)
			u.UpdatedAt = time.Now()
			return
		}
	}
}

func (u *User) SetMetadata(key string, value interface{}) {
	u.Metadata[key] = value
	u.UpdatedAt = time.Now()
}

func (u *User) GetMetadata(key string) (interface{}, bool) {
	value, exists := u.Metadata[key]
	return value, exists
}

// Generic functions (Go 1.18+)
func Max[T comparable](a, b T) T {
	if a > b {
		return a
	}
	return b
}

func Min[T comparable](a, b T) T {
	if a < b {
		return a
	}
	return b
}

func Contains[T comparable](slice []T, item T) bool {
	for _, v := range slice {
		if v == item {
			return true
		}
	}
	return false
}

func Map[T, U any](slice []T, fn func(T) U) []U {
	result := make([]U, len(slice))
	for i, v := range slice {
		result[i] = fn(v)
	}
	return result
}

func Filter[T any](slice []T, fn func(T) bool) []T {
	var result []T
	for _, v := range slice {
		if fn(v) {
			result = append(result, v)
		}
	}
	return result
}

func Reduce[T, U any](slice []T, initial U, fn func(U, T) U) U {
	result := initial
	for _, v := range slice {
		result = fn(result, v)
	}
	return result
}

// Error types
type ValidationError struct {
	Field   string
	Message string
}

func (e ValidationError) Error() string {
	return fmt.Sprintf("validation error on field '%s': %s", e.Field, e.Message)
}

type NotFoundError struct {
	Resource string
	ID       interface{}
}

func (e NotFoundError) Error() string {
	return fmt.Sprintf("%s with ID %v not found", e.Resource, e.ID)
}

// Service layer
type UserService struct {
	repo Repository
	mu   sync.RWMutex
}

func NewUserService(repo Repository) *UserService {
	return &UserService{
		repo: repo,
	}
}

func (s *UserService) CreateUser(ctx context.Context, username string, email Email) (*User, error) {
	s.mu.Lock()
	defer s.mu.Unlock()

	user := NewUser(username, email)
	if err := user.Validate(); err != nil {
		return nil, fmt.Errorf("user validation failed: %w", err)
	}

	if err := s.repo.Save(ctx, user); err != nil {
		return nil, fmt.Errorf("failed to save user: %w", err)
	}

	return user, nil
}

func (s *UserService) GetUser(ctx context.Context, id UserID) (*User, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	entity, err := s.repo.FindByID(ctx, id)
	if err != nil {
		return nil, err
	}

	user, ok := entity.(*User)
	if !ok {
		return nil, errors.New("invalid user type")
	}

	return user, nil
}

func (s *UserService) UpdateUser(ctx context.Context, user *User) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if err := user.Validate(); err != nil {
		return fmt.Errorf("user validation failed: %w", err)
	}

	user.UpdatedAt = time.Now()
	return s.repo.Save(ctx, user)
}

func (s *UserService) DeleteUser(ctx context.Context, id UserID) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	return s.repo.Delete(ctx, id)
}

// Utility functions
func isValidEmail(email string) bool {
	pattern := `^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$`
	matched, _ := regexp.MatchString(pattern, email)
	return matched
}

func calculateAge(birthDate time.Time) int {
	now := time.Now()
	age := now.Year() - birthDate.Year()
	if now.YearDay() < birthDate.YearDay() {
		age--
	}
	return age
}

func formatDuration(d time.Duration) string {
	if d < time.Minute {
		return fmt.Sprintf("%.0fs", d.Seconds())
	}
	if d < time.Hour {
		return fmt.Sprintf("%.0fm", d.Minutes())
	}
	return fmt.Sprintf("%.1fh", d.Hours())
}

func retry(fn func() error, maxRetries int, delay time.Duration) error {
	var err error
	for i := 0; i <= maxRetries; i++ {
		if err = fn(); err == nil {
			return nil
		}
		if i < maxRetries {
			time.Sleep(delay)
		}
	}
	return fmt.Errorf("failed after %d retries: %w", maxRetries, err)
}

// HTTP handlers
func healthCheckHandler(w http.ResponseWriter, r *http.Request) {
	response := map[string]interface{}{
		"status":    "ok",
		"timestamp": time.Now().Unix(),
		"version":   Version,
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

func userHandler(service *UserService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodGet:
			getUserHandler(service, w, r)
		case http.MethodPost:
			createUserHandler(service, w, r)
		case http.MethodPut:
			updateUserHandler(service, w, r)
		case http.MethodDelete:
			deleteUserHandler(service, w, r)
		default:
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	}
}

func getUserHandler(service *UserService, w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	if idStr == "" {
		http.Error(w, "ID parameter is required", http.StatusBadRequest)
		return
	}

	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		http.Error(w, "Invalid ID format", http.StatusBadRequest)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), Timeout)
	defer cancel()

	user, err := service.GetUser(ctx, UserID(id))
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}

func createUserHandler(service *UserService, w http.ResponseWriter, r *http.Request) {
	var req struct {
		Username string `json:"username"`
		Email    string `json:"email"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), Timeout)
	defer cancel()

	user, err := service.CreateUser(ctx, req.Username, Email(req.Email))
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(user)
}

func updateUserHandler(service *UserService, w http.ResponseWriter, r *http.Request) {
	var user User
	if err := json.NewDecoder(r.Body).Decode(&user); err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), Timeout)
	defer cancel()

	if err := service.UpdateUser(ctx, &user); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(user)
}

func deleteUserHandler(service *UserService, w http.ResponseWriter, r *http.Request) {
	idStr := r.URL.Query().Get("id")
	if idStr == "" {
		http.Error(w, "ID parameter is required", http.StatusBadRequest)
		return
	}

	id, err := strconv.ParseInt(idStr, 10, 64)
	if err != nil {
		http.Error(w, "Invalid ID format", http.StatusBadRequest)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), Timeout)
	defer cancel()

	if err := service.DeleteUser(ctx, UserID(id)); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusNoContent)
}

// Concurrency examples
func worker(id int, jobs <-chan int, results chan<- int, wg *sync.WaitGroup) {
	defer wg.Done()
	for job := range jobs {
		fmt.Printf("Worker %d processing job %d\n", id, job)
		time.Sleep(time.Millisecond * 100) // Simulate work
		results <- job * 2
	}
}

func demonstrateConcurrency() {
	const numWorkers = 3
	const numJobs = 10

	jobs := make(chan int, numJobs)
	results := make(chan int, numJobs)
	var wg sync.WaitGroup

	// Start workers
	for i := 1; i <= numWorkers; i++ {
		wg.Add(1)
		go worker(i, jobs, results, &wg)
	}

	// Send jobs
	for i := 1; i <= numJobs; i++ {
		jobs <- i
	}
	close(jobs)

	// Wait for workers to finish
	go func() {
		wg.Wait()
		close(results)
	}()

	// Collect results
	for result := range results {
		fmt.Printf("Result: %d\n", result)
	}
}

// Mathematical functions
func fibonacci(n int) int {
	if n <= 1 {
		return n
	}
	return fibonacci(n-1) + fibonacci(n-2)
}

func factorial(n int) int {
	if n <= 1 {
		return 1
	}
	return n * factorial(n-1)
}

func isPrime(n int) bool {
	if n < 2 {
		return false
	}
	for i := 2; i <= int(math.Sqrt(float64(n))); i++ {
		if n%i == 0 {
			return false
		}
	}
	return true
}

func gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

// File operations
func readFile(filename string) ([]byte, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	return io.ReadAll(file)
}

func writeFile(filename string, data []byte) error {
	file, err := os.Create(filename)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.Write(data)
	return err
}

// Main function
func main() {
	fmt.Println("=== Go Test Program ===")

	// Create a user
	user := NewUser("johndoe", "john.doe@example.com")
	user.FirstName = "John"
	user.LastName = "Doe"
	user.Age = 30
	user.Status = StatusActive

	// Add address
	user.Address = &Address{
		Street:  "123 Main St",
		City:    "Anytown",
		State:   "CA",
		ZipCode: "12345",
		Country: "USA",
	}

	// Add tags and metadata
	user.AddTag("developer")
	user.AddTag("golang")
	user.SetMetadata("department", "Engineering")
	user.SetMetadata("level", "Senior")

	// Validate and serialize
	if err := user.Validate(); err != nil {
		log.Printf("Validation error: %v", err)
	} else {
		fmt.Printf("User: %s (%s)\n", user.FullName(), user.Email)
		fmt.Printf("Status: %s\n", user.Status)
		fmt.Printf("Tags: %v\n", user.Tags)
	}

	// JSON serialization
	jsonData, err := user.ToJSON()
	if err != nil {
		log.Printf("JSON serialization error: %v", err)
	} else {
		fmt.Printf("JSON: %s\n", string(jsonData))
	}

	// Array operations
	numbers := []int{5, 2, 8, 1, 9, 3}
	fmt.Printf("Original: %v\n", numbers)

	// Sort
	sort.Ints(numbers)
	fmt.Printf("Sorted: %v\n", numbers)

	// Generic functions
	fmt.Printf("Max of 10 and 20: %d\n", Max(10, 20))
	fmt.Printf("Min of 10 and 20: %d\n", Min(10, 20))

	// Map function
	doubled := Map(numbers, func(n int) int { return n * 2 })
	fmt.Printf("Doubled: %v\n", doubled)

	// Filter function
	evens := Filter(numbers, func(n int) bool { return n%2 == 0 })
	fmt.Printf("Evens: %v\n", evens)

	// Reduce function
	sum := Reduce(numbers, 0, func(acc, n int) int { return acc + n })
	fmt.Printf("Sum: %d\n", sum)

	// Mathematical functions
	fmt.Printf("Fibonacci(10): %d\n", fibonacci(10))
	fmt.Printf("Factorial(5): %d\n", factorial(5))
	fmt.Printf("Is 17 prime? %t\n", isPrime(17))
	fmt.Printf("GCD(48, 18): %d\n", gcd(48, 18))

	// String operations
	text := "Hello, World!"
	fmt.Printf("Original: %s\n", text)
	fmt.Printf("Uppercase: %s\n", strings.ToUpper(text))
	fmt.Printf("Lowercase: %s\n", strings.ToLower(text))
	fmt.Printf("Contains 'World': %t\n", strings.Contains(text, "World"))
	fmt.Printf("Split by comma: %v\n", strings.Split(text, ","))

	// Time operations
	now := time.Now()
	fmt.Printf("Current time: %s\n", now.Format(time.RFC3339))
	fmt.Printf("Unix timestamp: %d\n", now.Unix())

	// Duration formatting
	duration := 2*time.Hour + 30*time.Minute + 45*time.Second
	fmt.Printf("Duration: %s\n", formatDuration(duration))

	// Demonstrate concurrency
	fmt.Println("\n=== Concurrency Demo ===")
	demonstrateConcurrency()

	fmt.Println("\n=== End of Test Program ===")
}