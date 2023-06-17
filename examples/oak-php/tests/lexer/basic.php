<?php
// PHP test file

declare(strict_types=1);

namespace App\Models;

use DateTime;
use Exception;
use PDO;
use PDOException;

/**
 * User model class
 */
class User
{
    private int $id;
    private string $username;
    private string $email;
    private ?string $firstName;
    private ?string $lastName;
    private DateTime $createdAt;
    private bool $isActive;

    public function __construct(
        int $id,
        string $username,
        string $email,
        ?string $firstName = null,
        ?string $lastName = null,
        ?DateTime $createdAt = null,
        bool $isActive = true
    ) {
        $this->id = $id;
        $this->username = $username;
        $this->email = $email;
        $this->firstName = $firstName;
        $this->lastName = $lastName;
        $this->createdAt = $createdAt ?? new DateTime();
        $this->isActive = $isActive;
    }

    // Getters
    public function getId(): int
    {
        return $this->id;
    }

    public function getUsername(): string
    {
        return $this->username;
    }

    public function getEmail(): string
    {
        return $this->email;
    }

    public function getFirstName(): ?string
    {
        return $this->firstName;
    }

    public function getLastName(): ?string
    {
        return $this->lastName;
    }

    public function getFullName(): string
    {
        $parts = array_filter([$this->firstName, $this->lastName]);
        return implode(' ', $parts) ?: $this->username;
    }

    public function getCreatedAt(): DateTime
    {
        return $this->createdAt;
    }

    public function isActive(): bool
    {
        return $this->isActive;
    }

    // Setters
    public function setEmail(string $email): void
    {
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new Exception("Invalid email format: {$email}");
        }
        $this->email = $email;
    }

    public function setFirstName(?string $firstName): void
    {
        $this->firstName = $firstName;
    }

    public function setLastName(?string $lastName): void
    {
        $this->lastName = $lastName;
    }

    public function activate(): void
    {
        $this->isActive = true;
    }

    public function deactivate(): void
    {
        $this->isActive = false;
    }

    public function toArray(): array
    {
        return [
            'id' => $this->id,
            'username' => $this->username,
            'email' => $this->email,
            'first_name' => $this->firstName,
            'last_name' => $this->lastName,
            'full_name' => $this->getFullName(),
            'created_at' => $this->createdAt->format('Y-m-d H:i:s'),
            'is_active' => $this->isActive,
        ];
    }

    public function toJson(): string
    {
        return json_encode($this->toArray(), JSON_THROW_ON_ERROR);
    }
}

/**
 * User repository class
 */
class UserRepository
{
    private PDO $pdo;

    public function __construct(PDO $pdo)
    {
        $this->pdo = $pdo;
    }

    public function findById(int $id): ?User
    {
        $stmt = $this->pdo->prepare('SELECT * FROM users WHERE id = :id');
        $stmt->execute(['id' => $id]);
        
        $row = $stmt->fetch(PDO::FETCH_ASSOC);
        if (!$row) {
            return null;
        }

        return $this->createUserFromRow($row);
    }

    public function findByEmail(string $email): ?User
    {
        $stmt = $this->pdo->prepare('SELECT * FROM users WHERE email = :email');
        $stmt->execute(['email' => $email]);
        
        $row = $stmt->fetch(PDO::FETCH_ASSOC);
        if (!$row) {
            return null;
        }

        return $this->createUserFromRow($row);
    }

    public function findAll(int $limit = 100, int $offset = 0): array
    {
        $stmt = $this->pdo->prepare('SELECT * FROM users LIMIT :limit OFFSET :offset');
        $stmt->bindValue(':limit', $limit, PDO::PARAM_INT);
        $stmt->bindValue(':offset', $offset, PDO::PARAM_INT);
        $stmt->execute();

        $users = [];
        while ($row = $stmt->fetch(PDO::FETCH_ASSOC)) {
            $users[] = $this->createUserFromRow($row);
        }

        return $users;
    }

    public function save(User $user): bool
    {
        try {
            if ($user->getId() > 0) {
                return $this->update($user);
            } else {
                return $this->insert($user);
            }
        } catch (PDOException $e) {
            error_log("Failed to save user: " . $e->getMessage());
            return false;
        }
    }

    private function insert(User $user): bool
    {
        $sql = 'INSERT INTO users (username, email, first_name, last_name, is_active) 
                VALUES (:username, :email, :first_name, :last_name, :is_active)';
        
        $stmt = $this->pdo->prepare($sql);
        return $stmt->execute([
            'username' => $user->getUsername(),
            'email' => $user->getEmail(),
            'first_name' => $user->getFirstName(),
            'last_name' => $user->getLastName(),
            'is_active' => $user->isActive() ? 1 : 0,
        ]);
    }

    private function update(User $user): bool
    {
        $sql = 'UPDATE users SET 
                username = :username, 
                email = :email, 
                first_name = :first_name, 
                last_name = :last_name, 
                is_active = :is_active 
                WHERE id = :id';
        
        $stmt = $this->pdo->prepare($sql);
        return $stmt->execute([
            'id' => $user->getId(),
            'username' => $user->getUsername(),
            'email' => $user->getEmail(),
            'first_name' => $user->getFirstName(),
            'last_name' => $user->getLastName(),
            'is_active' => $user->isActive() ? 1 : 0,
        ]);
    }

    public function delete(int $id): bool
    {
        $stmt = $this->pdo->prepare('DELETE FROM users WHERE id = :id');
        return $stmt->execute(['id' => $id]);
    }

    private function createUserFromRow(array $row): User
    {
        return new User(
            (int) $row['id'],
            $row['username'],
            $row['email'],
            $row['first_name'],
            $row['last_name'],
            new DateTime($row['created_at']),
            (bool) $row['is_active']
        );
    }
}

// Utility functions
function validateEmail(string $email): bool
{
    return filter_var($email, FILTER_VALIDATE_EMAIL) !== false;
}

function sanitizeString(string $input): string
{
    return htmlspecialchars(trim($input), ENT_QUOTES, 'UTF-8');
}

function generateRandomString(int $length = 10): string
{
    $characters = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';
    $charactersLength = strlen($characters);
    $randomString = '';
    
    for ($i = 0; $i < $length; $i++) {
        $randomString .= $characters[rand(0, $charactersLength - 1)];
    }
    
    return $randomString;
}

// Array operations
$users = [
    ['id' => 1, 'name' => 'John', 'age' => 30],
    ['id' => 2, 'name' => 'Jane', 'age' => 25],
    ['id' => 3, 'name' => 'Bob', 'age' => 35],
];

// Array functions
$names = array_map(fn($user) => $user['name'], $users);
$adults = array_filter($users, fn($user) => $user['age'] >= 18);
$totalAge = array_reduce($users, fn($carry, $user) => $carry + $user['age'], 0);

// String operations
$text = "Hello, World!";
$upperText = strtoupper($text);
$lowerText = strtolower($text);
$words = explode(' ', $text);
$joined = implode('-', $words);

// Regular expressions
$pattern = '/^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/';
$email = 'test@example.com';
$isValidEmail = preg_match($pattern, $email);

// Date and time
$now = new DateTime();
$tomorrow = (new DateTime())->modify('+1 day');
$formatted = $now->format('Y-m-d H:i:s');

// File operations
$filename = 'test.txt';
$content = "This is a test file.\nWith multiple lines.";

file_put_contents($filename, $content);
$readContent = file_get_contents($filename);
$lines = file($filename, FILE_IGNORE_NEW_LINES);

// JSON operations
$data = ['name' => 'John', 'age' => 30, 'city' => 'New York'];
$json = json_encode($data, JSON_PRETTY_PRINT);
$decoded = json_decode($json, true);

// Error handling
try {
    $result = 10 / 0;
} catch (DivisionByZeroError $e) {
    echo "Error: " . $e->getMessage() . "\n";
} catch (Exception $e) {
    echo "General error: " . $e->getMessage() . "\n";
} finally {
    echo "Cleanup code here\n";
}

// Traits
trait Timestampable
{
    private DateTime $createdAt;
    private ?DateTime $updatedAt = null;

    public function getCreatedAt(): DateTime
    {
        return $this->createdAt;
    }

    public function getUpdatedAt(): ?DateTime
    {
        return $this->updatedAt;
    }

    public function touch(): void
    {
        $this->updatedAt = new DateTime();
    }
}

// Interface
interface Cacheable
{
    public function getCacheKey(): string;
    public function getCacheTtl(): int;
}

// Abstract class
abstract class Model implements Cacheable
{
    use Timestampable;

    abstract public function getId(): int;
    abstract public function toArray(): array;

    public function getCacheKey(): string
    {
        return static::class . ':' . $this->getId();
    }

    public function getCacheTtl(): int
    {
        return 3600; // 1 hour
    }
}

// Constants
const DEFAULT_TIMEOUT = 30;
const MAX_RETRIES = 3;

define('APP_VERSION', '1.0.0');
define('DEBUG_MODE', true);

// Global variables
$GLOBALS['config'] = [
    'database' => [
        'host' => 'localhost',
        'port' => 3306,
        'name' => 'test_db',
    ],
    'cache' => [
        'driver' => 'redis',
        'ttl' => 3600,
    ],
];

// Anonymous functions and closures
$calculator = function (int $a, int $b) use ($GLOBALS): int {
    $operation = $GLOBALS['config']['operation'] ?? 'add';
    
    return match ($operation) {
        'add' => $a + $b,
        'subtract' => $a - $b,
        'multiply' => $a * $b,
        'divide' => $b !== 0 ? intval($a / $b) : 0,
        default => 0,
    };
};

// Match expression (PHP 8+)
$httpCode = 404;
$message = match ($httpCode) {
    200 => 'OK',
    404 => 'Not Found',
    500 => 'Internal Server Error',
    default => 'Unknown Status',
};

echo "HTTP {$httpCode}: {$message}\n";

?>