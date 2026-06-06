# API Posts

Proyecto de aprendizaje desarrollado en **Rust** con el objetivo de construir APIs robustas, seguras y con un manejo de memoria eficiente aprovechando las garantías que ofrece el lenguaje.

## Objetivo

Explorar el desarrollo de APIs REST en Rust aplicando conceptos como:

- Manejo de memoria seguro sin garbage collector
- Concurrencia asíncrona con Tokio
- Autenticación con JWT
- Persistencia con PostgreSQL

## Stack tecnológico

| Componente | Tecnología |
|---|---|
| Framework web | [Actix-Web 4](https://actix.rs/) |
| Base de datos | PostgreSQL 15 |
| ORM / Query builder | SQLx |
| Autenticación | JWT (`jsonwebtoken`) |
| Hash de contraseñas | bcrypt |
| Serialización | Serde / serde_json |
| Variables de entorno | dotenvy |
| Contenedores | Docker + Docker Compose |

## 📚 Documentación de la API

Toda la documentación interactiva de los endpoints, los esquemas de autenticación JWT y los ejemplos de respuestas (JSON) está alojada públicamente en Postman.

[![Ejecutar en Postman](https://run.pstmn.io/button.svg)](https://documenter.getpostman.com/view/3761165/2sBXwqqAUJ)

> **Nota:** Para probar las rutas protegidas del CRUD de Posts, asegúrate de registrar un usuario primero e inyectar el token en el Header de Autorización como `Bearer {token}`.

## Endpoints

### Autenticación

| Método | Ruta | Descripción |
|---|---|---|
| `POST` | `/auth/register` | Registrar nuevo usuario |
| `POST` | `/auth/login` | Iniciar sesión, retorna JWT |

### Posts

| Método | Ruta | Descripción | Auth |
|---|---|---|---|
| `GET` | `/posts` | Listar todos los posts | No |
| `GET` | `/posts/{id}` | Obtener post por ID | No |
| `POST` | `/posts` | Crear nuevo post | Si |
| `PUT` | `/posts/{id}` | Actualizar post | Si |
| `DELETE` | `/posts/{id}` | Eliminar post | Si |

## Configuración

### Variables de entorno

Crea un archivo `.env` en la raíz del proyecto:

```env
DATABASE_URL=postgres://admin:supersecretpassword@localhost:5434/rust_db
JWT_SECRET=tu_secreto_jwt
```

### Con Docker Compose

```bash
docker-compose up --build
```

La API queda disponible en `http://localhost:8080`.

### Sin Docker

Requiere una instancia de PostgreSQL corriendo. Luego:

```bash
cargo run
```

## Estructura del proyecto

```
src/
├── main.rs              # Punto de entrada, configuración de rutas y pool de DB
├── models.rs            # Structs de datos (User, Post, claims JWT)
├── extractors.rs        # Extractor de autenticación JWT (middlewares)
└── controllers/
    ├── mod.rs
    ├── auth.rs          # Registro y login
    └── posts.rs         # CRUD de posts
```
