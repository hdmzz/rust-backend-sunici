# Documentation API - Gestion des Requêtes HTTP en Rust

## Vue d'ensemble

Ce projet démontre comment gérer différents types de requêtes HTTP en Rust avec Actix Web.

## Routes disponibles

### 1. GET - Route simple
```
GET /
```
Retourne un message de bienvenue simple.

**Exemple de réponse :**
```
Hello world!
```

### 2. GET - Route avec paramètres d'URL
```
GET /users/{id}
```
Récupère un utilisateur par son ID.

**Exemple :**
```bash
curl http://127.0.0.1:8080/users/123
```

**Réponse JSON :**
```json
{
  "id": 123,
  "name": "John Doe",
  "email": "john@example.com"
}
```

### 3. GET - Route avec query parameters
```
GET /users?page=1&limit=10
```
Liste les utilisateurs avec pagination.

**Exemple :**
```bash
curl "http://127.0.0.1:8080/users?page=2&limit=5"
```

### 4. POST - Création avec JSON body
```
POST /users
Content-Type: application/json
```

**Body JSON :**
```json
{
  "name": "Alice Smith",
  "email": "alice@example.com"
}
```

**Exemple :**
```bash
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Smith", "email": "alice@example.com"}'
```

### 5. PUT - Mise à jour
```
PUT /users/{id}
Content-Type: application/json
```

**Exemple :**
```bash
curl -X PUT http://127.0.0.1:8080/users/123 \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated", "email": "alice.new@example.com"}'
```

### 6. DELETE - Suppression
```
DELETE /users/{id}
```

**Exemple :**
```bash
curl -X DELETE http://127.0.0.1:8080/users/123
```

### 7. GET - Route avec paramètres multiples
```
GET /users/{user_id}/posts/{post_id}
```

**Exemple :**
```bash
curl http://127.0.0.1:8080/users/123/posts/456
```

### 8. GET - Route de santé
```
GET /health
```
Vérifie que le serveur fonctionne.

## Concepts clés

### 1. Macros de routes
```rust
#[get("/")]           // Route GET
#[post("/users")]     // Route POST
#[put("/users/{id}")] // Route PUT avec paramètre
#[delete("/users/{id}")] // Route DELETE
```

### 2. Extraction de paramètres
```rust
// Paramètre d'URL
async fn get_user(path: web::Path<u32>) -> Result<impl Responder>

// Paramètres multiples
async fn get_user_post(path: web::Path<(u32, u32)>) -> Result<impl Responder>

// Query parameters
async fn list_users(query: web::Query<HashMap<String, String>>) -> Result<impl Responder>
```

### 3. Body JSON
```rust
// Extraction du JSON body
async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<impl Responder>
```

### 4. Réponses JSON
```rust
// Retourner du JSON
Ok(HttpResponse::Ok().json(user))

// Différents codes de statut
Ok(HttpResponse::Created().json(new_user))
Ok(HttpResponse::NotFound().json("User not found"))
```

### 5. Route manuelle (sans macro)
```rust
.route("/health", web::get().to(health_check))
```

## Structure des données

### User
```rust
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}
```

### CreateUserRequest
```rust
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}
```

## Démarrage du serveur

```bash
cargo run
```

Le serveur démarre sur `http://127.0.0.1:8080`

## Tests avec curl

Voici quelques exemples de tests que vous pouvez faire :

```bash
# Test de base
curl http://127.0.0.1:8080/

# Récupérer un utilisateur
curl http://127.0.0.1:8080/users/1

# Lister avec pagination
curl "http://127.0.0.1:8080/users?page=1&limit=5"

# Créer un utilisateur
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Test User", "email": "test@example.com"}'

# Mettre à jour un utilisateur
curl -X PUT http://127.0.0.1:8080/users/1 \
  -H "Content-Type: application/json" \
  -d '{"name": "Updated User", "email": "updated@example.com"}'

# Supprimer un utilisateur
curl -X DELETE http://127.0.0.1:8080/users/1

# Santé du serveur
curl http://127.0.0.1:8080/health
```
