(* Comprehensive Coq Lexer Test *)

(* Imports *)
Require Import Coq.Init.Datatypes.
Require Import Coq.Lists.List.
Import ListNotations.

(* Section and Variables *)
Section Basics.
    Variable A : Type.
    
    (* Inductive Types *)
    Inductive day : Type :=
    | monday
    | tuesday
    | wednesday
    | thursday
    | friday
    | saturday
    | sunday.

    (* Definition and Pattern Matching *)
    Definition next_weekday (d:day) : day :=
        match d with
        | monday => tuesday
        | tuesday => wednesday
        | wednesday => thursday
        | thursday => friday
        | friday => saturday
        | saturday => sunday
        | sunday => monday
        end.

    (* Computation *)
    Compute (next_weekday monday).

    (* Theorem and Proof *)
    Theorem simple_theorem : forall (P Q : Prop), P -> (P -> Q) -> Q.
    Proof.
        intros P Q HP HPQ.
        apply HPQ.
        exact HP.
    Qed.

    (* Fixpoint (Recursion) *)
    Fixpoint plus (n : nat) (m : nat) : nat :=
        match n with
        | O => m
        | S n' => S (plus n' m)
        end.
