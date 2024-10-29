# 🌐 Kronos-Server

Kronos-Server est un serveur backend WebSocket, développé en Rust, destiné à la connexion avec un client Godot WebAssembly. Le serveur prend en charge les connexions WebSocket sécurisées (TLS) en mode production et les connexions non sécurisées en mode développement.

---

## 🚀 Fonctionnalités

- **WebSocket avec support TLS** : Fournit une connexion WebSocket sécurisée avec TLS en production.
- **Logs colorés et détaillés** : Affiche des informations détaillées des connexions et messages échangés avec les clients.
- **Gestion des connexions multiples** : Traite plusieurs connexions clients en parallèle, avec une réponse asynchrone.
- **Arrêt propre** : Gère le signal `Ctrl+C` pour une fermeture en douceur du serveur.

---

## 📋 Prérequis

- [Rust](https://www.rust-lang.org/) installé (1.56 ou version plus récente, version optimale : 1.82.0).
- [Cargo](https://doc.rust-lang.org/cargo/) pour la gestion des dépendances et l'exécution, version optimale : 1.82.0.

---

## 🛠️ Installation

Clonez le dépôt et accédez au dossier du projet :

```bash
git clone https://github.com/votre-utilisateur/kronos-server.git
cd kronos-server
```

Installez les dépendances en compilant le projet :

```bash
cargo build
```

---

## 🏃‍♂️ Usage

### Mode développement

En mode développement, le serveur utilise une connexion WebSocket non sécurisée :

1. **Définir le mode** : Ajoutez la variable d'environnement `MODE=dev` dans un fichier `.env` ou directement dans le terminal.

   Exemple pour le terminal :
   ```bash
   export MODE=dev  # Linux/macOS
   set MODE=dev     # Windows
   ```

2. **Lancer le serveur (en mode dev)** :

   ```bash
   cargo run
   ```

   Le serveur écoute par défaut sur `127.0.0.1:8080` pour les connexions entrantes.

### Mode production (avec TLS)

En mode production, le serveur utilise une connexion TLS. Vous devez fournir un certificat au format `.pfx` pour établir des connexions sécurisées.

1. **Configurer le certificat** : Placez votre certificat `.pfx` dans le dossier du projet et mettez à jour le chemin dans le code si nécessaire.

2. **Définir le mode** : Utilisez `MODE=prod` pour activer le mode production.

3. **Lancer le serveur (en mode prod)** :

   ```bash
   cargo run --features with-ssl
   ```

   Le serveur écoute alors sur `0.0.0.0:443`.

---

### 📦 Commandes utiles

- **Nettoyage** du projet :

   ```bash
   cargo clean
   ```

- **Compilation** sans exécuter :

   ```bash
   cargo build
   ```

- **Exécution** du serveur en mode développement (par défaut) :

   ```bash
   cargo run
   ```

---

## 🏗️ Architecture (en cours d'élaboration)

- **Connexion client** : Chaque client connecté est géré par une tâche asynchrone. Les messages texte reçus sont affichés dans les logs, et une réponse est renvoyée.
- **Gestion des erreurs** : Les erreurs de connexion et de réception de message sont loguées.
- **Arrêt du serveur** : L'appui sur `Ctrl+C` envoie un signal pour arrêter proprement le serveur.

---

## 👾 Exemple de client Godot (WebAssembly)

Ce serveur est conçu pour se connecter à un client développé avec [Godot Engine](https://godotengine.org/) et exporté en WebAssembly pour une compatibilité Web.
Des fonctionnalités viendront s'implémenter au fur et à mesure des commits pour interdire l'usage de clients alternatifs à Godot.

---

## 🤝 Contribuer

Les contributions sont les bienvenues. N'hésitez pas à ouvrir des issues et à proposer des pull requests !

---

## 📄 Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.

---

## 🔗 Liens utiles

- [Documentation Rust](https://doc.rust-lang.org/book/)
- [Guide Godot](https://docs.godotengine.org/en/stable/index.html)
