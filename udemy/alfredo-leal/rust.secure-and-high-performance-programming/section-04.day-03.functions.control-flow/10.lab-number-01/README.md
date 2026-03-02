[ENGLISH]

# Laboratory Nº1  
## Exercise: "Product Statistics Calculation"

### Context:
You are a developer tasked with creating a small program to manage basic product information in a store. The program must perform simple calculations using different data types.

---

## Instructions:

### Set up your development environment:

Make sure you have Rust installed (use `rustc --version` to verify).

Create a new Rust project:

```bash
cargo new rust_lab_1
cd rust_lab_1
````

---

## Step 1: Basic Variable Declaration

Declare the following variables:

* `product_name`: a `String` that stores the product name.
* `price`: a floating-point number (`f32`) that stores the product price.
* `quantity`: an integer number (`u32`) that stores the available quantity.
* `on_sale`: a boolean value indicating whether the product is on sale or not.

---

## Step 2: Perform Basic Calculations

* Create a new variable `total_value` that stores the total inventory value (that is, `price * quantity`).
* If the product is on sale (`on_sale == true`), reduce the price by 10%. Update `total_value` accordingly.

---

## Step 3: Convert and Format Data

* Convert `price` to a 64-bit floating-point number (`f64`) and display it.
* Convert `quantity` to a `String` and concatenate it with the message:
  `"Available quantity: X units"`.

---

## Step 4: Display the Result

Print all the formatted product information, including:

* Product name.
* Unit price.
* Available quantity.
* Total inventory value.
* Whether the product is on sale.

---
---

[SPANISH]

# Laboratorio Nº1  
## Ejercicio: "Cálculo de Estadísticas de un Producto"

### Contexto:
Eres un desarrollador encargado de crear un pequeño programa para gestionar la información básica de un producto en una tienda. El programa deberá realizar cálculos simples utilizando distintos tipos de datos.

---

## Instrucciones:

### Configura tu entorno de desarrollo:

Asegúrate de tener Rust instalado (usa `rustc --version` para verificar).

Crea un nuevo proyecto en Rust:

```bash
cargo new rust_lab_1
cd rust_lab_1
````

---

## Paso 1: Declaración de variables básicas

Declara las siguientes variables:

* `product_name`: un `String` que almacene el nombre del producto.
* `price`: un número de punto flotante (`f32`) que almacene el precio del producto.
* `quantity`: un número entero (`u32`) que almacene la cantidad disponible.
* `on_sale`: un valor booleano que indique si el producto está en oferta o no.

---

## Paso 2: Realizar cálculos básicos

* Crea una nueva variable `total_value` que almacene el valor total del inventario (es decir, `price * quantity`).
* Si el producto está en oferta (`on_sale == true`), reduce el precio en un 10%. Actualiza `total_value` en consecuencia.

---

## Paso 3: Convertir y formatear datos

* Convierte `price` a un número de punto flotante de 64 bits (`f64`) y muéstralo.
* Convierte `quantity` a una cadena de texto (`String`) y concaténala con un mensaje:
  `"Cantidad disponible: X unidades"`.

---

## Paso 4: Mostrar el resultado

Imprime toda la información del producto formateada, incluyendo:

* Nombre del producto.
* Precio unitario.
* Cantidad disponible.
* Valor total del inventario.
* Si el producto está en oferta.