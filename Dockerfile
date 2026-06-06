# primero: construcción
# imagen oficial de Rust basada en Debian para compilar
FROM rust:bookworm as builder

WORKDIR /app

# copiamos los archivos de configuración como en todos los otros lenguajes, para cachear las dependencias
COPY Cargo.toml Cargo.lock ./
# creamos un src dummy para compilar y cachear solo las dependencias primero
# para acelerar las futuras compilaciones si no se cambia el Cargo.toml
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# copiamos nuestro código y lo compilamos reemplazando el dummy anterior
RUN rm -rf src
COPY src src
# para que se tome en cuenta el index
COPY public public
# se hac un touch para forzar a cargo a recompilar (sino no detacta el cambio)
RUN touch src/main.rs
RUN cargo build --release

# segundo ejecución
# aca usamos la imagen de Debian súper ligera solo para correr el binario
FROM debian:bookworm-slim

WORKDIR /app

# ahora se instalan certificados de seguridad (para peticiones https)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# se copia solo el binario compilado que hicimos arriba como primer paso
COPY --from=builder /app/target/release/api_posts /app/api_posts

# igual que siempre, se expone el puerto
EXPOSE 8080

# y RUN ***** RUUUN
CMD ["./api_posts"]