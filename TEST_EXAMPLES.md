# Tests pour la gestion des requêtes HTTP

## Test de création d'utilisateur avec ID auto-généré

### Exemple de requête POST
```bash
curl -X POST http://127.0.0.1:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Marie Dupont", "email": "marie.dupont@example.com"}'
```

### Réponse attendue
```json
{
  "id": 42,
  "name": "Marie Dupont", 
  "email": "marie.dupont@example.com",
  "message": "Utilisateur créé avec succès"
}
```

## Avantages de cette approche

1. **ID auto-généré** : L'ID est simulé comme s'il venait de la base de données
2. **Structure séparée** : `UserResponse` pour les réponses, `CreateUserRequest` pour les requêtes
3. **Message informatif** : Confirmation de la création
4. **Réutilisation du port** : Configuration similaire à SO_REUSEADDR en C++

## Pour une vraie base de données

Quand vous intégrerez une vraie base de données, remplacez cette ligne :
```rust
let generated_id = 42; // ID qui serait généré automatiquement par la DB
```

Par quelque chose comme :
```rust
let generated_id = database.insert_user(&user_data.name, &user_data.email).await?;
```

## Gestion des erreurs

Pour une application production, ajoutez la gestion d'erreurs :
```rust
#[post("/users")]
async fn create_user(user_data: web::Json<CreateUserRequest>) -> Result<impl Responder> {
    // Validation des données
    if user_data.name.is_empty() || user_data.email.is_empty() {
        return Ok(HttpResponse::BadRequest().json("Nom et email requis"));
    }
    
    // Vérification de l'email
    if !user_data.email.contains('@') {
        return Ok(HttpResponse::BadRequest().json("Email invalide"));
    }
    
    // Insertion en base de données avec gestion d'erreur
    match database.insert_user(&user_data.name, &user_data.email).await {
        Ok(generated_id) => {
            let response = UserResponse {
                id: generated_id,
                name: user_data.name.clone(),
                email: user_data.email.clone(),
                message: Some("Utilisateur créé avec succès".to_string()),
            };
            Ok(HttpResponse::Created().json(response))
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json("Erreur lors de la création"))
    }
}
```
