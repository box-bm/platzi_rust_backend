# STAGE 1: Creamos contenedor para compilar la aplicación
FROM ubuntu:20.04

# Variable para evitar bloqueos en la terminal al construir el contenedor
ENV DEBIAN_FRONTEND=noninteractive

## Dependencias necesarios del Sistema Operativo
RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y

## Instalamos Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

## Preparamos directorio principal del proyecto
WORKDIR /app
COPY ./ /app

## Compilamos la aplicación
RUN cargo clean
RUN cargo build --release


# STAGE 2: Como el primer contenedor es muy pesado y no lo necesitamos, creamos otro solo para exponer la aplicación
FROM ubuntu:20.04

# Variable para evitar bloqueos en la terminal al construir el contenedor
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install curl pkg-config libssl-dev build-essential libpq-dev -y
WORKDIR /app

## Copiamos desde el otro contenedor, los archivos de la aplicación
COPY --from=0 /app/target/release/blog-platzi /app
COPY /templates/ /app/templates

## Corremos la aplicación
CMD ./blog-platzi