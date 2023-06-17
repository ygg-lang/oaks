-- Lean 4 Test File - Comprehensive Syntax Coverage
-- This file tests various Lean 4 syntax elements for lexer testing

-- Universe declarations
universe u v w

-- Namespace declarations
namespace BasicTypes

-- Basic type definitions
inductive Bool : Type where
  | true : Bool
  | false : Bool

inductive Nat : Type where
  | zero : Nat
  | succ : Nat → Nat

-- Pattern matching and recursion
def not : Bool → Bool
  | Bool.true => Bool.false
  | Bool.false => Bool.true

def add : Nat → Nat → Nat
  | Nat.zero, n => n
  | Nat.succ m, n => Nat.succ (add m n)

def mul : Nat → Nat → Nat
  | Nat.zero, _ => Nat.zero
  | Nat.succ m, n => add n (mul m n)

-- Function definitions with where clause
def factorial : Nat → Nat := fun n =>
  match n with
  | Nat.zero => Nat.succ Nat.zero
  | Nat.succ k => mul (Nat.succ k) (factorial k)
  where
    helper : Nat → Nat → Nat
      | Nat.zero, acc => acc
      | Nat.succ n, acc => helper n (mul (Nat.succ n) acc)

end BasicTypes

-- Import statements
import Std.Data.List.Basic
import Std.Data.Nat.Basic
import Mathlib.Data.Real.Basic

-- Open namespaces
open List Nat

-- Variable declarations
variable (α β γ : Type*) [Inhabited α]
variable (p q r : Prop)
variable (x y z : ℕ)
variable (f g : α → β)
variable (h : β → γ)

-- Axiom declarations
axiom choice : ∀ {α : Type*} (p : α → Prop), (∃ x, p x) → α

-- Constant declarations
constant π : ℝ
constant e : ℝ

-- Structure definitions
structure Point (α : Type*) where
  x : α
  y : α

structure Vector (α : Type*) [Add α] where
  components : List α
  
-- Class definitions
class Monoid (α : Type*) where
  one : α
  mul : α → α → α
  left_one : ∀ a : α, mul one a = a
  right_one : ∀ a : α, mul a one = a
  assoc : ∀ a b c : α, mul (mul a b) c = mul a (mul b c)

class Group (α : Type*) extends Monoid α where
  inv : α → α
  left_inv : ∀ a : α, mul (inv a) a = one

-- Instance definitions
instance : Monoid ℕ where
  one := 0
  mul := (· + ·)
  left_one := Nat.zero_add
  right_one := Nat.add_zero
  assoc := Nat.add_assoc

-- Inductive types with parameters
inductive List (α : Type*) : Type* where
  | nil : List α
  | cons : α → List α → List α

inductive Tree (α : Type*) : Type* where
  | leaf : α → Tree α
  | node : Tree α → Tree α → Tree α

-- Mutual inductive types
mutual
  inductive Even : ℕ → Prop where
    | zero : Even 0
    | succ : ∀ n, Odd n → Even (n + 1)
  
  inductive Odd : ℕ → Prop where
    | one : Odd 1
    | succ : ∀ n, Even n → Odd (n + 1)
end

-- Dependent types
def Vector' (α : Type*) (n : ℕ) : Type* := { l : List α // l.length = n }

-- Function types and lambda expressions
def compose : (β → γ) → (α → β) → (α → γ) := fun g f => fun x => g (f x)

def curry : (α × β → γ) → (α → β → γ) := fun f => fun a b => f (a, b)

def uncurry : (α → β → γ) → (α × β → γ) := fun f => fun ⟨a, b⟩ => f a b

-- Pattern matching with complex patterns
def listLength : List α → ℕ
  | [] => 0
  | _ :: xs => 1 + listLength xs

def listMap (f : α → β) : List α → List β
  | [] => []
  | x :: xs => f x :: listMap f xs

def listFilter (p : α → Bool) : List α → List α
  | [] => []
  | x :: xs => if p x then x :: listFilter p xs else listFilter p xs

-- Theorem statements and proofs
theorem add_comm (a b : ℕ) : a + b = b + a := by
  induction a with
  | zero => simp [Nat.zero_add, Nat.add_zero]
  | succ a ih => 
    rw [Nat.succ_add, Nat.add_succ, ih]

theorem add_assoc (a b c : ℕ) : (a + b) + c = a + (b + c) := by
  induction a with
  | zero => simp [Nat.zero_add]
  | succ a ih => 
    rw [Nat.succ_add, Nat.succ_add, Nat.succ_add, ih]

-- Lemma with detailed proof
lemma list_length_append (l₁ l₂ : List α) : 
  (l₁ ++ l₂).length = l₁.length + l₂.length := by
  induction l₁ with
  | nil => simp [List.nil_append, List.length_nil, Nat.zero_add]
  | cons h t ih => 
    simp [List.cons_append, List.length_cons, Nat.succ_add, ih]

-- Proof by cases
theorem bool_cases (b : Bool) : b = true ∨ b = false := by
  cases b with
  | true => left; rfl
  | false => right; rfl

-- Proof by contradiction
theorem not_not (p : Prop) : ¬¬p → p := by
  intro h
  by_contra hnp
  exact h hnp

-- Existential proofs
theorem exists_succ (n : ℕ) : ∃ m, m = n + 1 := by
  use n + 1
  rfl

-- Universal quantification
theorem forall_add_zero (n : ℕ) : n + 0 = n := by
  rw [Nat.add_zero]

-- Tactics and proof terms
example (h₁ : p → q) (h₂ : q → r) (h₃ : p) : r := by
  apply h₂
  apply h₁
  exact h₃

example (h₁ : p → q) (h₂ : q → r) (h₃ : p) : r := h₂ (h₁ h₃)

-- Simp lemmas
@[simp]
theorem list_length_nil : ([] : List α).length = 0 := rfl

@[simp]
theorem list_length_cons (x : α) (xs : List α) : 
  (x :: xs).length = xs.length + 1 := rfl

-- Decidable instances
instance (n m : ℕ) : Decidable (n ≤ m) := by
  induction m generalizing n with
  | zero => 
    cases n with
    | zero => exact isTrue (Nat.le_refl 0)
    | succ n => exact isFalse (Nat.not_succ_le_zero n)
  | succ m ih =>
    cases n with
    | zero => exact isTrue (Nat.zero_le (m + 1))
    | succ n => 
      have : Decidable (n ≤ m) := ih n
      cases this with
      | isTrue h => exact isTrue (Nat.succ_le_succ h)
      | isFalse h => exact isFalse (fun h' => h (Nat.le_of_succ_le_succ h'))

-- Type classes with operations
class Functor (F : Type* → Type*) where
  map : {α β : Type*} → (α → β) → F α → F β
  map_id : ∀ {α : Type*} (x : F α), map id x = x
  map_comp : ∀ {α β γ : Type*} (g : β → γ) (f : α → β) (x : F α), 
    map (g ∘ f) x = map g (map f x)

instance : Functor List where
  map := List.map
  map_id := by
    intro α x
    induction x with
    | nil => rfl
    | cons h t ih => simp [List.map, ih]
  map_comp := by
    intro α β γ g f x
    induction x with
    | nil => rfl
    | cons h t ih => simp [List.map, ih]

-- Monad instance
class Monad (M : Type* → Type*) extends Functor M where
  pure : {α : Type*} → α → M α
  bind : {α β : Type*} → M α → (α → M β) → M β
  left_id : ∀ {α β : Type*} (a : α) (f : α → M β), bind (pure a) f = f a
  right_id : ∀ {α : Type*} (m : M α), bind m pure = m
  assoc : ∀ {α β γ : Type*} (m : M α) (f : α → M β) (g : β → M γ),
    bind (bind m f) g = bind m (fun a => bind (f a) g)

-- Option monad
instance : Monad Option where
  pure := some
  bind := fun m f => match m with
    | none => none
    | some a => f a
  left_id := by
    intro α β a f
    rfl
  right_id := by
    intro α m
    cases m with
    | none => rfl
    | some a => rfl
  assoc := by
    intro α β γ m f g
    cases m with
    | none => rfl
    | some a => rfl

-- Quotient types
def Equiv (α : Type*) := α → α → Prop

variable (r : Equiv α)

axiom equiv_refl : ∀ a : α, r a a
axiom equiv_symm : ∀ a b : α, r a b → r b a
axiom equiv_trans : ∀ a b c : α, r a b → r b c → r a c

def Quotient (r : Equiv α) : Type* := Quot r

-- Inductive-recursive definitions
inductive Code : Type where
  | nat : Code
  | pi : Code → (ℕ → Code) → Code

def decode : Code → Type
  | Code.nat => ℕ
  | Code.pi a b => (x : decode a) → decode (b (sorry : ℕ))

-- Coinductive types
coinductive Stream (α : Type*) : Type* where
  | cons : α → Stream α → Stream α

def Stream.head : Stream α → α
  | Stream.cons h _ => h

def Stream.tail : Stream α → Stream α
  | Stream.cons _ t => t

-- Partial functions
partial def collatz : ℕ → ℕ
  | 1 => 1
  | n => if n % 2 = 0 then collatz (n / 2) else collatz (3 * n + 1)

-- Well-founded recursion
def ackermann : ℕ → ℕ → ℕ := fun m n =>
  match m with
  | 0 => n + 1
  | m + 1 => 
    match n with
    | 0 => ackermann m 1
    | n + 1 => ackermann m (ackermann (m + 1) n)

-- Termination proofs
def gcd : ℕ → ℕ → ℕ
  | 0, n => n
  | m, 0 => m
  | m, n => if m ≤ n then gcd m (n - m) else gcd (m - n) n
termination_by gcd m n => m + n

-- Macro definitions
macro "my_simp" : tactic => `(tactic| simp_all)

macro "my_intro" : tactic => `(tactic| intro)

-- Notation declarations
notation:65 a " ⊕ " b => Xor a b

infixl:70 " ⊗ " => fun a b => (a, b)

prefix:75 "¬" => Not

postfix:80 "!" => factorial

-- Attribute declarations
@[inline]
def fastAdd (a b : ℕ) : ℕ := a + b

@[simp, norm_cast]
theorem cast_add (a b : ℕ) : ↑(a + b) = (↑a : ℤ) + ↑b := by
  rfl

-- Section with variables
section GroupTheory

variable {G : Type*} [Group G]

theorem inv_inv (a : G) : (a⁻¹)⁻¹ = a := by
  apply Group.left_inv

theorem inv_mul_cancel_left (a b : G) : a⁻¹ * (a * b) = b := by
  rw [← Group.assoc, Group.left_inv, Group.left_one]

end GroupTheory

-- Namespace with definitions
namespace LinearAlgebra

variable {K : Type*} [Field K]

structure VectorSpace (K : Type*) [Field K] where
  carrier : Type*
  add : carrier → carrier → carrier
  smul : K → carrier → carrier
  zero : carrier
  neg : carrier → carrier

end LinearAlgebra

-- Calc proofs
example (a b c : ℕ) : a + b + c = c + b + a := by
  calc a + b + c 
    = (a + b) + c := by rw [Nat.add_assoc]
    _ = (b + a) + c := by rw [Nat.add_comm a b]
    _ = b + a + c := by rw [← Nat.add_assoc]
    _ = b + (a + c) := by rw [Nat.add_assoc]
    _ = b + (c + a) := by rw [Nat.add_comm a c]
    _ = (b + c) + a := by rw [← Nat.add_assoc]
    _ = (c + b) + a := by rw [Nat.add_comm b c]
    _ = c + b + a := by rw [Nat.add_assoc]

-- Have and suffices
example (h : p → q) : ¬q → ¬p := by
  intro hnq hp
  have hq : q := h hp
  exact hnq hq

example (h : p ↔ q) : q ↔ p := by
  constructor
  · intro hq
    suffices hp : p by exact hp
    exact h.mpr hq
  · intro hp
    exact h.mp hp

-- Match expressions
def listSum : List ℕ → ℕ := fun l =>
  match l with
  | [] => 0
  | x :: xs => x + listSum xs

-- If-then-else expressions
def max (a b : ℕ) : ℕ := if a ≤ b then b else a

def min (a b : ℕ) : ℕ := if a ≤ b then a else b

-- Let expressions
def complexComputation (n : ℕ) : ℕ :=
  let x := n * 2
  let y := x + 1
  let z := y * y
  z + x + y

-- Do notation (for monads)
def optionExample : Option ℕ := do
  let x ← some 5
  let y ← some 3
  return x + y

-- Array and list comprehensions
#eval [1, 2, 3, 4, 5].map (· * 2)
#eval [1, 2, 3, 4, 5].filter (· > 3)

-- Type ascriptions
def typedExample : ℕ → ℕ := (fun x => x + 1 : ℕ → ℕ)

-- Anonymous constructor syntax
def pointExample : Point ℕ := ⟨3, 4⟩

-- Subtype syntax
def positiveNat : Type := {n : ℕ // n > 0}

-- Dependent pair syntax
def dependentPairExample : Σ n : ℕ, Vector ℕ n := ⟨3, sorry⟩

-- Placeholder and sorry
def incompleteProof : ∀ n : ℕ, n + 0 = n := by
  intro n
  sorry

-- Comments and documentation
/-- 
This is a documentation comment for a function.
It can span multiple lines and use markdown.

# Example
```lean
myFunction 5 = 10
```
-/
def myFunction (n : ℕ) : ℕ := n * 2

-- Single line comment

/-
Multi-line comment
that can span
several lines
-/

-- Final example combining multiple features
theorem comprehensive_example {α : Type*} [DecidableEq α] (l : List α) (x : α) :
  x ∈ l ↔ ∃ i, i < l.length ∧ l.get ⟨i, by assumption⟩ = x := by
  constructor
  · intro h
    induction l with
    | nil => 
      simp at h
    | cons y ys ih =>
      simp [List.mem_cons] at h
      cases h with
      | inl h => 
        use 0
        simp [List.get, h]
      | inr h =>
        have ⟨i, hi, hget⟩ := ih h
        use i + 1
        constructor
        · simp [List.length_cons]
          exact Nat.succ_lt_succ hi
        · simp [List.get, hget]
  · intro ⟨i, hi, hget⟩
    induction l generalizing i with
    | nil => 
      simp at hi
    | cons y ys ih =>
      cases i with
      | zero =>
        simp [List.get] at hget
        simp [List.mem_cons, hget]
      | succ j =>
        simp [List.mem_cons]
        right
        apply ih
        use j
        constructor
        · simp [List.length_cons] at hi
          exact Nat.lt_of_succ_lt_succ hi
        · simp [List.get] at hget
          exact hget