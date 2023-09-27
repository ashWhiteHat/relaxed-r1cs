# Relaxed R1CS

[A Folding Scheme for NP](https://eprint.iacr.org/2021/370.pdf#page=12&zoom=100,100,780)

## [R1CS](https://eprint.iacr.org/2021/370.pdf#page=13&zoom=100,100,250)

The R1CS structure consits of sparse matrices $A, B, C \in \mathbb F^l$.

$$
(A · Z) ◦ (B · Z) = C · Z
$$

- $l$: instance length
- $m - l - 1$: witness length
- $x$: instance
- $W$: witness
- $Z$: $(W, x, 1)$

## [Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,250)

$$
(A · Z) ◦ (B · Z) = u · (C · Z) + E
$$

- $E$: error vector
- $u$: scalar
- $x$: public inputs and outputs
- $Z$: $(W, x, u)$

## [R1CS to Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,370)

The R1CS instance can be expressed as a relaxed R1CS instance by $u = 1$ and $E = 0$.

$$
(A · Z) ◦ (B · Z) = 1 · (C · Z) + 0
$$

## [Commitment](https://eprint.iacr.org/2021/370.pdf#page=14&zoom=100,100,850)

- $pp_W$: commitment vectors for $W$ size $m$
- $pp_E$: commitment vectors for $E$ size $m - l - 1$
- $(\overline E, u, \overline W, x)$: committed relaxed R1CS instance
- $u$: scalar
- $x$: public inputs and outputs
- $\overline E$: $Com(pp_E, E, r_E)$
- $\overline W$: $Com(pp_W, W, r_W)$

## [Committed Relaxed R1CS](https://eprint.iacr.org/2021/370.pdf#page=15&zoom=100,100,210)

$$
(\overline E, u, \overline W, x)
$$

- $E$: error vector (0 when the R1CS is initalized)
- $u$: scalar (1 when the R1CS is initalized)
- $W$: witness
- $x$: instance

$(\overline E, u, \overline W, x)$ is satisfied by a witness $(E, r_E, W, r_W)$

if
- $\overline E = Com(pp_E, E, r_E)$
- $\overline W = Com(pp_W, W, r_W)$
- $(A · Z) ◦ (B · Z) = u · (C · Z) + E$

## [Folding Scheme](https://eprint.iacr.org/2021/370.pdf#page=15&zoom=100,100,300)

## Todo

- [ ] Transcript Replacement
- [ ] Folded Instance Verification
