-- SQL test file
-- Database schema creation

-- Create database
CREATE DATABASE IF NOT EXISTS test_db;
USE test_db;

-- Create tables
CREATE TABLE users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    date_of_birth DATE,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE categories (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    parent_id INT,
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL
);

CREATE TABLE products (
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    price DECIMAL(10, 2) NOT NULL,
    stock_quantity INT DEFAULT 0,
    category_id INT,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE SET NULL,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL,
    INDEX idx_price (price),
    INDEX idx_category (category_id),
    INDEX idx_name (name)
);

CREATE TABLE orders (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT NOT NULL,
    total_amount DECIMAL(10, 2) NOT NULL,
    status ENUM('pending', 'processing', 'shipped', 'delivered', 'cancelled') DEFAULT 'pending',
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    shipping_address TEXT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE order_items (
    id INT PRIMARY KEY AUTO_INCREMENT,
    order_id INT NOT NULL,
    product_id INT NOT NULL,
    quantity INT NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    UNIQUE KEY unique_order_product (order_id, product_id)
);

-- Insert sample data
INSERT INTO users (username, email, password_hash, first_name, last_name, date_of_birth) VALUES
('john_doe', 'john@example.com', 'hashed_password_1', 'John', 'Doe', '1990-05-15'),
('jane_smith', 'jane@example.com', 'hashed_password_2', 'Jane', 'Smith', '1985-08-22'),
('bob_wilson', 'bob@example.com', 'hashed_password_3', 'Bob', 'Wilson', '1992-12-03');

INSERT INTO categories (name, description) VALUES
('Electronics', 'Electronic devices and accessories'),
('Books', 'Physical and digital books'),
('Clothing', 'Apparel and fashion items'),
('Home & Garden', 'Home improvement and gardening supplies');

INSERT INTO categories (name, description, parent_id) VALUES
('Smartphones', 'Mobile phones and accessories', 1),
('Laptops', 'Portable computers', 1),
('Fiction', 'Fiction books and novels', 2),
('Non-Fiction', 'Educational and reference books', 2);

INSERT INTO products (name, description, price, stock_quantity, category_id, created_by) VALUES
('iPhone 15', 'Latest Apple smartphone', 999.99, 50, 5, 1),
('Samsung Galaxy S24', 'Android flagship phone', 899.99, 30, 5, 1),
('MacBook Pro', '16-inch laptop with M3 chip', 2499.99, 15, 6, 2),
('The Great Gatsby', 'Classic American novel', 12.99, 100, 7, 3),
('Clean Code', 'Programming best practices', 45.99, 25, 8, 2);

-- Complex queries
-- Select with joins
SELECT 
    u.username,
    u.email,
    COUNT(o.id) as order_count,
    SUM(o.total_amount) as total_spent
FROM users u
LEFT JOIN orders o ON u.id = o.user_id
WHERE u.is_active = TRUE
GROUP BY u.id, u.username, u.email
HAVING total_spent > 100
ORDER BY total_spent DESC;

-- Subquery example
SELECT 
    p.name,
    p.price,
    c.name as category_name
FROM products p
JOIN categories c ON p.category_id = c.id
WHERE p.price > (
    SELECT AVG(price) 
    FROM products 
    WHERE category_id = p.category_id
);

-- Window functions
SELECT 
    name,
    price,
    category_id,
    ROW_NUMBER() OVER (PARTITION BY category_id ORDER BY price DESC) as price_rank,
    AVG(price) OVER (PARTITION BY category_id) as avg_category_price,
    LAG(price) OVER (ORDER BY price) as previous_price
FROM products;

-- Common Table Expression (CTE)
WITH category_stats AS (
    SELECT 
        c.id,
        c.name,
        COUNT(p.id) as product_count,
        AVG(p.price) as avg_price,
        MAX(p.price) as max_price,
        MIN(p.price) as min_price
    FROM categories c
    LEFT JOIN products p ON c.id = p.category_id
    GROUP BY c.id, c.name
)
SELECT 
    name,
    product_count,
    ROUND(avg_price, 2) as avg_price,
    max_price,
    min_price
FROM category_stats
WHERE product_count > 0
ORDER BY avg_price DESC;

-- Update statements
UPDATE products 
SET stock_quantity = stock_quantity - 1 
WHERE id IN (
    SELECT product_id 
    FROM order_items 
    WHERE order_id = 1
);

-- Delete with conditions
DELETE FROM orders 
WHERE status = 'cancelled' 
AND order_date < DATE_SUB(NOW(), INTERVAL 30 DAY);

-- Create indexes
CREATE INDEX idx_user_email ON users(email);
CREATE INDEX idx_product_name_price ON products(name, price);
CREATE INDEX idx_order_status_date ON orders(status, order_date);

-- Create view
CREATE VIEW active_user_orders AS
SELECT 
    u.username,
    u.email,
    o.id as order_id,
    o.total_amount,
    o.status,
    o.order_date
FROM users u
JOIN orders o ON u.id = o.user_id
WHERE u.is_active = TRUE;

-- Stored procedure
DELIMITER //
CREATE PROCEDURE GetUserOrderSummary(IN user_id INT)
BEGIN
    SELECT 
        COUNT(*) as total_orders,
        SUM(total_amount) as total_spent,
        AVG(total_amount) as avg_order_value,
        MAX(order_date) as last_order_date
    FROM orders 
    WHERE user_id = user_id;
END //
DELIMITER ;

-- Trigger
DELIMITER //
CREATE TRIGGER update_product_stock 
AFTER INSERT ON order_items
FOR EACH ROW
BEGIN
    UPDATE products 
    SET stock_quantity = stock_quantity - NEW.quantity 
    WHERE id = NEW.product_id;
END //
DELIMITER ;