# Lab2-ConwaysGame
# Conway's game of life

Este proyecto es una implementación del famoso autómata celular **Conway's Game of Life** utilizando el lenguaje de programación **Rust** y la biblioteca gráfica **Raylib**. La simulación muestra la evolución de un tablero de células vivas y muertas siguiendo las reglas clásicas del juego, renderizando la animación en una ventana gráfica.

---

## 🚀 Instrucciones para ejecutar

> ⚠️ **Importante**: Este repositorio ignora los archivos `Cargo.toml` y `Cargo.lock`, por lo que deberás crearlos manualmente.

---

### 1. Inicializa el proyecto de Rust

Primeor debes clonar el repositorio.

```bash
cargo init

#en Cargo.toml agrega la siguiente dependencia:
[dependencies]
raylib = "5.5.1"

#ejecuta el programa:
cargo run

magick convert -delay 10 -loop 0 frames/frame_*.png conway.gif
```

### 2. Requisitos

Rust

libclang y dependencias de raylib (instalación depende del sistema operativo)

Para convertir de los frames generados a un gif se utilizó imagemagick instalado desde homebrew con mac

### 3. GIF resultado

![Conway GIF](conway.gif)