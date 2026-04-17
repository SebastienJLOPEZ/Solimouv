use askama::Template;
use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

pub struct Association {
    pub id: u32,
    pub name: &'static str,
    pub sport: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
    pub color: &'static str,
}

pub struct Atelier {
    pub heure: &'static str,
    pub titre: &'static str,
    pub association: &'static str,
    pub description: &'static str,
    pub lieu: &'static str,
    pub emoji: &'static str,
    pub image: &'static str,
    pub icon: &'static str,
}

pub struct Jour {
    pub date: &'static str,
    pub jour: &'static str,
    pub ateliers: Vec<Atelier>,
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {
    current_page: &'static str,
    associations: Vec<Association>,
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    current_page: &'static str,
}

#[derive(Template)]
#[template(path = "programme.html")]
struct ProgrammeTemplate {
    current_page: &'static str,
    jours: Vec<Jour>,
}

#[derive(Template)]
#[template(path = "associations.html")]
struct AssociationsTemplate {
    current_page: &'static str,
    associations: Vec<Association>,
}

#[derive(Template)]
#[template(path = "carte.html")]
struct CarteTemplate {
    current_page: &'static str,
    jours: Vec<Jour>,
}

#[derive(Template)]
#[template(path = "club.html")]
struct ClubTemplate {
    current_page: &'static str,
}

// ---------------------------------------------------------------------------
// Template → Response helper
// ---------------------------------------------------------------------------

fn render<T: Template>(tmpl: T) -> Response {
    match tmpl.render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            tracing::error!("Template render error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de rendu").into_response()
        }
    }
}

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

fn get_associations() -> Vec<Association> {
    vec![
        Association { id: 1,  name: "Les Foulées Dynamiques", sport: "Athlétisme",    description: "Découvrez la course à pied et l'athlétisme avec des entraîneurs passionnés. Initiations, relais et courses pour tous les niveaux.", emoji: "🏃", color: "#E74C3C" },
        Association { id: 2,  name: "FC Étoile",              sport: "Football",       description: "Le football au cœur de la ville ! Matchs amicaux, tournois et ateliers techniques encadrés par nos éducateurs diplômés.", emoji: "⚽", color: "#3498DB" },
        Association { id: 3,  name: "Basket Énergie",          sport: "Basketball",     description: "Venez dribler, shooter et vous amuser sur nos terrains. Ateliers pour enfants et adultes, du débutant au confirmé.", emoji: "🏀", color: "#E67E22" },
        Association { id: 4,  name: "Vélo Club Horizon",       sport: "Cyclisme",       description: "Balades découverte, parcours VTT et initiations au cyclisme sur route. Apportez votre vélo ou empruntez le nôtre !", emoji: "🚴", color: "#27AE60" },
        Association { id: 5,  name: "Aqua Vitesse",            sport: "Natation",       description: "Plongez dans le monde de la natation : démonstrations, jeux aquatiques et conseils de nos nageurs expérimentés.", emoji: "🏊", color: "#2980B9" },
        Association { id: 6,  name: "Judo Club Harmonie",      sport: "Judo",           description: "Découvrez les arts martiaux dans un esprit de respect et d'harmonie. Initiations judo et self-défense pour tous.", emoji: "🥋", color: "#8E44AD" },
        Association { id: 7,  name: "Tennis Club du Parc",      sport: "Tennis",         description: "Raquettes en main ! Initiations, mini-tournois et défis sur nos courts. Matériel fourni pour les débutants.", emoji: "🎾", color: "#16A085" },
        Association { id: 8,  name: "Volley Passion",           sport: "Volleyball",     description: "Beach-volley et volley en salle : ambiance conviviale garantie. Matchs ouverts à tous et animations.", emoji: "🏐", color: "#F39C12" },
        Association { id: 9,  name: "Gym'Flex",                 sport: "Gymnastique",    description: "Souplesse, force et grâce : initiations à la gymnastique artistique et rythmique pour petits et grands.", emoji: "🤸", color: "#E91E63" },
        Association { id: 10, name: "Lames d'Argent",          sport: "Escrime",        description: "En garde ! Découvrez l'escrime, sport olympique élégant. Équipement fourni et moniteurs certifiés.", emoji: "🤺", color: "#607D8B" },
        Association { id: 11, name: "Rugby Tonnerre",           sport: "Rugby",          description: "Esprit d'équipe et fair-play : ateliers de rugby adaptés à tous les âges. Plaquages, passes et essais !", emoji: "🏉", color: "#795548" },
        Association { id: 12, name: "Handball Solidaire",       sport: "Handball",       description: "Tirs, passes et stratégie : le handball est un sport complet et accessible. Venez le découvrir avec nous.", emoji: "🤾", color: "#FF5722" },
        Association { id: 13, name: "Zen & Mouvement",          sport: "Yoga & Bien-être", description: "Retrouvez calme et énergie avec nos séances de yoga, stretching et méditation en plein air.", emoji: "🧘", color: "#009688" },
    ]
}

fn get_programme() -> Vec<Jour> {
    vec![
        Jour {
            date: "13 juin 2026",
            jour: "Samedi",
            ateliers: vec![
                Atelier { heure: "09:00", titre: "Échauffement collectif", association: "Zen & Mouvement", description: "Réveil musculaire et stretching pour bien démarrer la journée.", lieu: "Scène principale", emoji: "🧘", image: "https://images.unsplash.com/photo-1544367567-0f2fcb009e0b?w=150&fit=crop", icon: "flower-2" },
                Atelier { heure: "09:30", titre: "Course découverte", association: "Les Foulées Dynamiques", description: "Parcours de 3 km ouvert à tous, encadré par des coachs.", lieu: "Stade municipal", emoji: "🏃", image: "https://images.unsplash.com/photo-1552674605-db6ffd4facb5?w=150&fit=crop", icon: "footprints" },
                Atelier { heure: "10:00", titre: "Tournoi de football 5v5", association: "FC Étoile", description: "Matchs de 15 minutes, inscriptions sur place.", lieu: "Terrain A", emoji: "⚽", image: "https://images.unsplash.com/photo-1574629810360-7efbbe195018?w=150&fit=crop", icon: "circle" },
                Atelier { heure: "10:30", titre: "Initiation basketball", association: "Basket Énergie", description: "Dribbles, passes et tirs : atelier encadré pour tous les niveaux.", lieu: "Gymnase", emoji: "🏀", image: "https://images.unsplash.com/photo-1533758602862-23f46f483842?w=150&fit=crop", icon: "accessibility" },
                Atelier { heure: "11:00", titre: "Balade VTT", association: "Vélo Club Horizon", description: "Randonnée découverte de 10 km autour du site du festival.", lieu: "Départ parking", emoji: "🚴", image: "https://images.unsplash.com/photo-1541625602330-2277a4c46182?w=150&fit=crop", icon: "bike" },
                Atelier { heure: "13:30", titre: "Jeux aquatiques", association: "Aqua Vitesse", description: "Relais, water-polo et jeux dans la piscine en plein air.", lieu: "Piscine", emoji: "🏊", image: "https://images.unsplash.com/photo-1530549387789-4c1017266635?w=150&fit=crop", icon: "waves" },
                Atelier { heure: "14:00", titre: "Initiation judo", association: "Judo Club Harmonie", description: "Découverte des bases du judo : chutes, prises et combat souple.", lieu: "Dojo extérieur", emoji: "🥋", image: "https://images.unsplash.com/photo-1555597673-b21d5c935865?w=150&fit=crop", icon: "shield" },
                Atelier { heure: "14:00", titre: "Mini-tournoi tennis", association: "Tennis Club du Parc", description: "Matchs en simple et double, raquettes prêtées.", lieu: "Courts de tennis", emoji: "🎾", image: "https://images.unsplash.com/photo-1599474924187-334a4ae5bd3c?w=150&fit=crop", icon: "target" },
                Atelier { heure: "15:30", titre: "Beach-volley", association: "Volley Passion", description: "Tournoi de beach-volley sur sable, équipes mixtes.", lieu: "Terrain sable", emoji: "🏐", image: "https://images.unsplash.com/photo-1612872087720-bb876e2e67d1?w=150&fit=crop", icon: "circle-dot" },
                Atelier { heure: "16:00", titre: "Démonstration gymnastique", association: "Gym'Flex", description: "Spectacle et initiations : sol, poutre et barres.", lieu: "Scène principale", emoji: "🤸", image: "", icon: "sparkles" },
                Atelier { heure: "17:30", titre: "Session yoga sunset", association: "Zen & Mouvement", description: "Séance de yoga en plein air pour clôturer la journée en douceur.", lieu: "Pelouse centrale", emoji: "🧘", image: "https://images.unsplash.com/photo-1544367567-0f2fcb009e0b?w=150&fit=crop", icon: "flower-2" },
            ],
        },
        Jour {
            date: "14 juin 2026",
            jour: "Dimanche",
            ateliers: vec![
                Atelier { heure: "09:00", titre: "Méditation matinale", association: "Zen & Mouvement", description: "Commencez la journée avec une séance de méditation guidée.", lieu: "Pelouse centrale", emoji: "🧘", image: "https://images.unsplash.com/photo-1544367567-0f2fcb009e0b?w=150&fit=crop", icon: "flower-2" },
                Atelier { heure: "09:30", titre: "Escrime pour tous", association: "Lames d'Argent", description: "Initiations épée et fleuret, équipement fourni.", lieu: "Salle couverte", emoji: "🤺", image: "", icon: "swords" },
                Atelier { heure: "10:00", titre: "Tournoi rugby touch", association: "Rugby Tonnerre", description: "Rugby sans plaquage, ouvert à tous et toutes.", lieu: "Terrain B", emoji: "🏉", image: "https://images.unsplash.com/photo-1531415074968-036ba1b575da?w=150&fit=crop", icon: "trophy" },
                Atelier { heure: "10:30", titre: "Atelier handball", association: "Handball Solidaire", description: "Exercices techniques et matchs en équipes mixtes.", lieu: "Gymnase", emoji: "🤾", image: "", icon: "hand" },
                Atelier { heure: "11:00", titre: "Atelier cyclisme sur route", association: "Vélo Club Horizon", description: "Sortie route encadrée de 15 km, casques obligatoires.", lieu: "Départ parking", emoji: "🚴", image: "https://images.unsplash.com/photo-1541625602330-2277a4c46182?w=150&fit=crop", icon: "bike" },
                Atelier { heure: "13:30", titre: "Relais natation", association: "Aqua Vitesse", description: "Course de relais en équipes dans la piscine.", lieu: "Piscine", emoji: "🏊", image: "https://images.unsplash.com/photo-1530549387789-4c1017266635?w=150&fit=crop", icon: "waves" },
                Atelier { heure: "14:00", titre: "Initiation self-défense", association: "Judo Club Harmonie", description: "Techniques de base en self-défense, accessible à tous.", lieu: "Dojo extérieur", emoji: "🥋", image: "https://images.unsplash.com/photo-1555597673-b21d5c935865?w=150&fit=crop", icon: "shield" },
                Atelier { heure: "14:00", titre: "Challenge multi-sports", association: "Les Foulées Dynamiques", description: "Parcours combinant course, saut et lancer. Classement individuel.", lieu: "Stade municipal", emoji: "🏃", image: "https://images.unsplash.com/photo-1552674605-db6ffd4facb5?w=150&fit=crop", icon: "activity" },
                Atelier { heure: "15:00", titre: "Volley en salle", association: "Volley Passion", description: "Matchs de volley-ball en salle, 6 contre 6.", lieu: "Gymnase", emoji: "🏐", image: "https://images.unsplash.com/photo-1612872087720-bb876e2e67d1?w=150&fit=crop", icon: "circle-dot" },
                Atelier { heure: "16:00", titre: "Spectacle de clôture", association: "Gym'Flex", description: "Grande démonstration de gymnastique et remise des prix.", lieu: "Scène principale", emoji: "🤸", image: "", icon: "sparkles" },
                Atelier { heure: "17:00", titre: "Cérémonie de clôture", association: "Toutes les associations", description: "Discours, remerciements et rendez-vous pour l'année prochaine !", lieu: "Scène principale", emoji: "🎉", image: "https://images.unsplash.com/photo-1492684223066-81342ee5ff30?w=150&fit=crop", icon: "party-popper" },
            ],
        },
    ]
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn home() -> Response {
    render(HomeTemplate {
        current_page: "home",
        associations: get_associations(),
    })
}

async fn about() -> Response {
    render(AboutTemplate {
        current_page: "about",
    })
}

async fn programme() -> Response {
    render(ProgrammeTemplate {
        current_page: "programme",
        jours: get_programme(),
    })
}

async fn associations() -> Response {
    render(AssociationsTemplate {
        current_page: "associations",
        associations: get_associations(),
    })
}

async fn carte() -> Response {
    render(CarteTemplate {
        current_page: "carte",
        jours: get_programme(),
    })
}

async fn club() -> Response {
    render(ClubTemplate {
        current_page: "club",
    })
}

async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, Html("<h1>404 – Page non trouvée</h1>".to_string())).into_response()
}

async fn sitemap() -> Response {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url><loc>https://solimouv.fr/</loc><priority>1.0</priority><changefreq>weekly</changefreq></url>
  <url><loc>https://solimouv.fr/a-propos</loc><priority>0.8</priority><changefreq>monthly</changefreq></url>
  <url><loc>https://solimouv.fr/programme</loc><priority>0.9</priority><changefreq>weekly</changefreq></url>
  <url><loc>https://solimouv.fr/associations</loc><priority>0.8</priority><changefreq>monthly</changefreq></url>
  <url><loc>https://solimouv.fr/carte</loc><priority>0.7</priority><changefreq>monthly</changefreq></url>
</urlset>"#;
    ([(header::CONTENT_TYPE, "application/xml")], xml).into_response()
}

async fn serve_manifest() -> Response {
    match tokio::fs::read_to_string("static/manifest.json").await {
        Ok(json) => (
            [(header::CONTENT_TYPE, "application/manifest+json")],
            json,
        )
            .into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .route("/a-propos", get(about))
        .route("/programme", get(programme))
        .route("/associations", get(associations))
        .route("/carte", get(carte))
        .route("/club", get(club))
        .route("/sitemap.xml", get(sitemap))
        // PWA files served from root
        .route_service("/robots.txt", ServeFile::new("static/robots.txt"))
        .route_service("/sw.js", ServeFile::new("static/sw.js"))
        .route("/manifest.json", get(serve_manifest))
        // Static assets
        .nest_service("/static", ServeDir::new("static"))
        .fallback(get(not_found));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Impossible de démarrer le serveur sur le port {port}"));

    tracing::info!("Solimouv' – http://localhost:{port}");
    println!("🏅 Solimouv' – http://localhost:{port}");

    axum::serve(listener, app)
        .await
        .expect("Erreur du serveur");
}
