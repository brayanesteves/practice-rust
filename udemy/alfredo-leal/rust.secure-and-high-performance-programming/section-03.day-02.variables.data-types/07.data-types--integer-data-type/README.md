# [ENGLISH]

## Data types in Rust

1. **Scalars:** Represent single and simple values.
2. **Compound:** Store more than one value.

---

## Integer data type

Represents numbers without decimals.
They store values depending on the number of bits.
They can be signed or unsigned.

### Integer Types Table

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

---

## Examples

```rust
let x: u32 = 10;  // <= Unsigned.

let x: i32 = -10; // <= Signed.
```

---

## Number literals

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

---

## Overflow

It occurs when a numeric value exceeds the allowed range for its data type, either by being too large or too small.

---

---

# [SPANISH]

## Tipos de datos en Rust

1. **Escalares:** Representan valores únicos y simples.
2. **Compuestos:** Almacenan más de un valor.

---

## Tipo de dato entero

Representa números sin decimales.
Almacenan valores dependiendo de la cantidad de bits del número.
Pueden ser con o sin signo.

### Tabla de tipos enteros

| Length  | Signed | Unsigned |
| ------- | ------ | -------- |
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

---

## Ejemplos

```rust
let x: u32 = 10;  // <= Sin signo.

let x: i32 = -10; // <= Con signo.
```

---

## Number literals

| Number literals | Example     |
| --------------- | ----------- |
| Decimal         | 98_222      |
| Hex             | 0xff        |
| Octal           | 0o77        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

---

## Desbordamiento

Ocurre cuando un valor numérico es excede el rango permitido para su tipo de dtos, ya sea por ser demasiado grande o demasiado pequeño.
