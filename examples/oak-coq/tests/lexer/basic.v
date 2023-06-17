(* Coq test file for lexer testing *)

(* Basic theorem and proof *)
Theorem plus_O_n : forall n : nat, 0 + n = n.
Proof.
  intros n. simpl. reflexivity.
Qed.

(* Function definition *)
Fixpoint factorial (n : nat) : nat :=
  match n with
  | 0 => 1
  | S p => n * factorial p
  end.

(* Inductive type definition *)
Inductive tree (A : Type) : Type :=
  | leaf : A -> tree A
  | node : tree A -> tree A -> tree A.

(* Record definition *)
Record point : Type := make_point
  { x : nat
  ; y : nat
  }.

(* Module definition *)
Module Type ORDERED_TYPE.
  Parameter t : Set.
  Parameter le : t -> t -> Prop.
  Axiom le_refl : forall x : t, le x x.
  Axiom le_trans : forall x y z : t, le x y -> le y z -> le x z.
End ORDERED_TYPE.

(* Notation definition *)
Notation "x <= y" := (le x y) (at level 70).

(* Class definition *)
Class Eq (A : Type) :=
  { eq : A -> A -> bool
  ; eq_refl : forall x, eq x x = true
  }.

(* Instance declaration *)
Instance eq_nat : Eq nat :=
  { eq := Nat.eqb
  ; eq_refl := Nat.eqb_refl
  }.

(* Lemma with tactics *)
Lemma factorial_pos : forall n, factorial n > 0.
Proof.
  induction n.
  - simpl. auto.
  - simpl. apply Nat.lt_0_mul. auto.
Qed.

(* Section for local definitions *)
Section LocalDefinitions.
  Variable A : Type.
  Definition identity (x : A) := x.
End LocalDefinitions.