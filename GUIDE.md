# Guide de prise en main — Solimouv'

Ce document explique pas à pas comment utiliser, modifier et maintenir le site Solimouv'.

---

## 1. Prérequis

| Outil | Version minimum | Installation |
|-------|----------------|-------------|
| **Rust** | 1.75+ | [rustup.rs](https://rustup.rs) |
| **Docker** (optionnel) | 20+ | [docker.com](https://www.docker.com) |
| **Fly CLI** (optionnel) | dernière | `curl -L https://fly.io/install.sh \| sh` |

Vérifier l'installation :

```bash
rustc --version    # rust 1.75+
cargo --version    # cargo 1.75+
```

---

## 2. Lancer le site en local

```bash
# Depuis la racine du projet
cargo run
```

Le site démarre sur **http://localhost:3000**. Il recompile automatiquement les templates Askama à chaque `cargo run`.

> **Astuce** : pour recompiler automatiquement à chaque modification, installez `cargo-watch` :
> ```bash
> cargo install cargo-watch
> cargo watch -x run
> ```

---

## 3. Modifier le contenu

### 3.1 Changer les informations du festival

Ouvrir `src/main.rs` et modifier :

| Information | Fonction à modifier | Exemple |
|------------|-------------------|---------|
| Associations | `get_associations()` | Nom, sport, description, emoji, couleur |
| Programme / Ateliers | `get_programme()` | Horaires, activités, lieu, jour |
| Lieu & dates | Template `home.html` | Adresse, dates dans le HTML |
| Lieux sur la carte | Template `carte.html` | Coordonnées GPS, noms des zones |

**Exemple** — Ajouter une association :

```rust
// Dans get_associations(), ajouter :
Association {
    id: 14,
    name: "Nouveau Club",
    sport: "Natation",
    description: "Club de natation inclusive pour tous les âges.",
    emoji: "🏊",
    color: "#2196F3",
},
```

Puis relancer `cargo run`.

### 3.2 Modifier les pages HTML

Les templates sont dans `templates/`. Chaque fichier hérite de `base.html` :

| Fichier | Page | Ce qu'on peut y modifier |
|---------|------|-------------------------|
| `home.html` | Accueil | Textes du hero, statistiques, articles |
| `programme.html` | Programme | En-tête, filtres jour |
| `carte.html` | Carte | Coordonnées GPS, zoom par défaut |
| `associations.html` | Associations | Mise en page des cartes |
| `about.html` | À propos | Valeurs, timeline, textes |
| `club.html` | Club | Questions du quiz, profil |
| `base.html` | Layout global | Navbar, couleurs Tailwind, meta tags |

**Syntaxe Askama** (résumé) :

```html
{{ variable }}              <!-- Afficher une variable -->
{% for item in list %}      <!-- Boucle -->
{% endfor %}
{% if condition %}          <!-- Condition -->
{% endif %}
{% block content %}         <!-- Bloc extensible -->
{% endblock %}
```

### 3.3 Changer les couleurs

Dans `templates/base.html`, section `tailwind.config` :

```javascript
colors: {
    primary: '#0029FF',   // Bleu principal
    action: '#FF0066',    // Rose (boutons CTA)
    success: '#03CEA4',   // Vert (badges)
    saffron: '#FFE600',   // Jaune (accents)
    tomato: '#1A0B2E',    // Violet foncé (fond)
    night: '#000000',     // Noir (textes)
    paper: '#FFFFFF',     // Blanc
    cloud: '#F4F4F5',     // Gris clair (fonds)
}
```

Modifier les valeurs hexadécimales et recharger la page.

### 3.4 Changer le logo

1. Remplacer `static/images/logo.jpeg` par le nouveau fichier (garder le même nom)
2. Si le format change (ex: `.png`), mettre à jour les `<img>` dans `base.html` et `home.html`

### 3.5 Changer les images du carousel

Les images sont dans `static/images/public/`. Pour les remplacer :

1. Placer les nouvelles images dans `static/images/public/`
2. Ouvrir `templates/home.html`
3. Modifier les chemins `src=` dans la section `hero-carousel`

---

## 4. Modifier la carte

Dans `templates/carte.html`, les coordonnées du festival :

```javascript
const festivalLat = 48.8566;   // Latitude
const festivalLng = 2.3522;    // Longitude
```

Les marqueurs des lieux (gymnases, terrains, courts…) sont définis dans le tableau `locations` de `carte.html` :

```javascript
const locations = [
    { name: "Gymnase Principal",   lat: festivalLat + 0.001,  lng: festivalLng - 0.002 },
    { name: "Terrain Synthétique", lat: festivalLat - 0.001,  lng: festivalLng + 0.001 },
    // ...
];
```

Pour ajouter un lieu, ajouter une entrée dans ce tableau et recharger la page.

---

## 5. PWA (Progressive Web App)

### Fichiers concernés

| Fichier | Rôle |
|---------|------|
| `static/manifest.json` | Nom, icônes, couleurs, screenshots |
| `static/sw.js` | Cache et mode hors-ligne |
| `static/js/app.js` | Enregistrement du Service Worker |

### Mettre à jour les icônes PWA

1. Créer deux images PNG carrées : **192×192** et **512×512** pixels
2. Les nommer `icon-192.png` et `icon-512.png`
3. Les placer dans `static/images/`

### Forcer une mise à jour du cache

Ouvrir `static/sw.js` et incrémenter le numéro de version :

```javascript
const CACHE_NAME = 'festsport-v3';  // était v2 → passer à v3
```

Les visiteurs recevront la nouvelle version au prochain chargement.

---

## 6. Déploiement

### Option A — Docker

```bash
# Construire l'image
docker build -t solimouv .

# Lancer le conteneur
docker run -p 3000:3000 solimouv
```

### Option B — Fly.io

```bash
# Première fois (crée l'app)
fly launch

# Mises à jour suivantes
fly deploy
```

Le fichier `fly.toml` est pré-configuré :
- Région : `cdg` (Paris)
- RAM : 256 Mo
- Auto-scaling : s'arrête quand inactif, redémarre automatiquement
- HTTPS forcé

### Option C — Render

Le fichier `render.yaml` permet un déploiement sur [Render](https://render.com) :

```bash
# Pousser sur un dépôt GitHub connecté à Render
git push
```

Render détectera automatiquement la configuration et construira le projet.

### Option D — Serveur classique

```bash
cargo build --release
# Copier sur le serveur :
#   - target/release/festival-sport  (binaire)
#   - templates/                     (templates HTML)
#   - static/                        (CSS, JS, images)

# Sur le serveur :
RUST_LOG=info ./festival-sport
```

Le serveur écoute sur le port **3000**. Utiliser un reverse proxy (Nginx, Caddy) pour HTTPS.

---

## 7. Maintenance courante

### Ajouter un article sur la page d'accueil

Dans `templates/home.html`, section « Derniers articles », dupliquer un bloc `<article>` :

```html
<article class="bg-white rounded-3xl overflow-hidden shadow-lg text-night">
    <div class="h-40 bg-gradient-to-br from-primary to-primary/70 flex items-center justify-center">
        <i data-lucide="nom-icone" class="w-12 h-12 text-white"></i>
    </div>
    <div class="p-6">
        <span class="text-[10px] font-bold text-success uppercase tracking-widest">DATE</span>
        <h3 class="font-bold text-lg mt-1 mb-2">TITRE</h3>
        <p class="text-sm text-gray-500 line-clamp-2">DESCRIPTION</p>
    </div>
</article>
```

Les icônes disponibles sont listées sur [lucide.dev/icons](https://lucide.dev/icons).

### Ajouter une photo au carousel

1. Placer l'image dans `static/images/public/`
2. Dans `home.html`, ajouter dans `#hero-carousel` :

```html
<img src="/static/images/public/nouvelle-photo.jpeg"
     alt="Description"
     class="hero-slide absolute inset-0 w-full h-full object-cover opacity-0 transition-opacity duration-1000">
```

Le JS du carousel gère automatiquement toutes les slides `.hero-slide`.

### Modifier les questions du quiz

Dans `templates/club.html`, chaque question est un bloc HTML dans la section quiz. Modifier le texte des questions et les `data-correct` pour la bonne réponse.

---

## 8. Dépannage

| Problème | Solution |
|----------|---------|
| `cargo run` échoue | Vérifier `rustc --version` ≥ 1.75. Lancer `cargo clean` puis réessayer |
| Port 3000 occupé | Tuer le processus : `Stop-Process -Name festival-sport -Force` (Windows) ou `pkill festival-sport` (Linux) |
| PWA non installable | Ouvrir DevTools → Application → Manifest. Vérifier que les icônes sont accessibles. Incrémenter la version du SW |
| Templates non mis à jour | Askama compile les templates à la compilation Rust. Relancer `cargo run` après toute modification de `.html` |
| Images carousel ne changent pas | Vider le cache navigateur ou incrémenter `CACHE_NAME` dans `sw.js` |
| CSS ne se met pas à jour | Tailwind est chargé via CDN, pas de build. Vérifier la connexion internet. Les classes custom sont dans `static/css/style.css` |

---

## 9. Contacts techniques

- **Serveur** : Rust/Axum — [docs.rs/axum](https://docs.rs/axum)
- **Templates** : Askama — [docs.rs/askama](https://docs.rs/askama)
- **Style** : Tailwind CSS — [tailwindcss.com/docs](https://tailwindcss.com/docs)
- **Icônes** : Lucide — [lucide.dev](https://lucide.dev)
- **Carte** : Leaflet — [leafletjs.com/reference](https://leafletjs.com/reference)
- **PWA** : [web.dev/progressive-web-apps](https://web.dev/progressive-web-apps)
