# 🦀 Rust desde cero: ejercicios prácticos

Repositorio personal para aprender **Rust desde cero**, practicar la resolución de problemas y avanzar desde los fundamentos del lenguaje hasta pequeños proyectos de consola.

## 🎯 Objetivos

- Practicar variables, tipos de datos, condiciones, bucles y funciones.
- Aprender a trabajar con vectores, cadenas de texto y colecciones.
- Comprender conceptos propios de Rust como **ownership**, **borrowing** y referencias.
- Manejar valores opcionales y errores con `Option<T>` y `Result<T, E>`.
- Construir programas pequeños que integren varios conceptos.

## 🧰 Requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado.
- Cargo, el gestor de paquetes y herramienta de compilación que se instala con Rust.
- Un editor de código, por ejemplo [Visual Studio Code](https://code.visualstudio.com/).

Comprueba la instalación desde la terminal:

```bash
rustc --version
cargo --version
```

## 🚀 Cómo ejecutar un ejercicio

Crea un proyecto para practicar:

```bash
cargo new ejercicio-01
cd ejercicio-01
cargo run
```

Escribe el programa en `src/main.rs` y ejecútalo con `cargo run`. Para mantener los ejercicios organizados, puedes crear un proyecto independiente para cada uno.

## 📚 Ruta de aprendizaje

### Nivel 1: Fundamentos

- [✅] **01. Hola, Rust:** imprimir información básica en la terminal.
- [✅] **02. Calculadora básica:** solicitar dos números y mostrar suma, resta, multiplicación y división.
- [✅] **03. Número par o impar:** identificar si un entero es par, impar, positivo, negativo o cero.
- [✅] **04. Conversor de temperaturas:** convertir grados Celsius a Fahrenheit y viceversa.
- [✅] **05. Tabla de multiplicar:** mostrar la tabla de un número del 1 al 10.

**Conceptos:** `println!`, variables, tipos numéricos, entrada por terminal, `if`, `else`, operadores, bucles y funciones.

### Nivel 2: Colecciones y lógica

- [✅] **06. Adivina el número:** indicar si el número introducido es mayor o menor que el número secreto.
- [✅] **07. Analizador de números:** calcular la suma, el promedio, el valor mínimo y el máximo de un vector.
- [✅] **08. Contador de vocales:** contar las vocales de una frase sin distinguir mayúsculas y minúsculas.
- [✅] **09. Palíndromos:** comprobar si una palabra se lee igual en ambos sentidos.
- [✅] **10. Lista de tareas:** agregar, listar y eliminar tareas desde un menú de consola.

**Conceptos:** `Vec<T>`, `String`, iteradores, rangos, índices, recorridos y lógica de programas.

### Nivel 3: Características esenciales de Rust

- [✅] **11. Ownership en la práctica:** experimentar con el movimiento de valores y `clone()`.
- [✅] **12. Préstamos y referencias:** contar los caracteres de un `String` sin tomar posesión de él.
- [✅] **13. Referencias mutables:** modificar un vector mediante una referencia `&mut`.
- [✅] **14. Búsqueda opcional:** devolver `Some(posición)` si se encuentra un número o `None` si no existe.
- [✅] **15. Calculadora con errores:** devolver un error al intentar dividir entre cero, en lugar de provocar un fallo inesperado.

**Conceptos:** ownership, borrowing, `&T`, `&mut T`, `Option<T>`, `Some`, `None`, `Result<T, E>` y manejo de errores.

### Nivel 4: Proyectos pequeños

- [✅] **16. Gestor de contactos:** crear, buscar, editar y eliminar contactos en memoria.
- [✅] **17. Sistema de inventario:** registrar productos, precios y cantidades, y calcular el valor del inventario.
- [✅] **18. Analizador CSV:** leer registros numéricos de un archivo CSV, calcular estadísticas y guardar resultados.
- [✅] **19. Cliente de una API:** consultar una API pública, procesar una respuesta JSON y mostrar campos seleccionados.
- [ ] **20. Buscador de archivos:** buscar archivos por nombre o extensión dentro de una carpeta y sus subcarpetas.

**Conceptos:** `struct`, métodos, archivos, crates, JSON, solicitudes HTTP, `std::fs`, rutas y recursividad.

## 📁 Organización sugerida

Una estructura sencilla para mantener el repositorio ordenado:

```text
rust-desde-cero/
├── README.md
├── ejercicio-01/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── ejercicio-02/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── ...
```

Cada ejercicio puede incluir su propio `Cargo.toml` y su directorio `src/`.

## 📈 Seguimiento

Marcaré cada ejercicio como completado cuando funcione y pueda explicar cómo lo resolví. También intentaré contemplar entradas inválidas y casos límite, no solamente el caso ideal.

## 📖 Recursos

- [The Rust Programming Language — The Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Documentación oficial de Rust](https://doc.rust-lang.org/)
- [Cargo: The Rust Build Tool and Package Manager](https://doc.rust-lang.org/cargo/)

---

*Repositorio de práctica y aprendizaje. Los ejercicios se irán completando progresivamente.*