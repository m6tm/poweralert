# Plan de Réalisation - Tickets PowerAlert

Ce document liste l'ensemble des tickets de réalisation pour le projet PowerAlert, classés par ordre logique de développement.

## Epic 1 : Initialisation & Socle Technique
*   **[TICKET-1.1] Initialisation du projet Tauri + Astro**
    *   *Description :* Bootstraper le projet avec l'outil de création de Tauri en sélectionnant le template Astro.
    *   *Critères d'acceptation :* Le projet compile, `npm run tauri dev` lance une fenêtre affichant la page d'accueil par défaut d'Astro.
*   **[TICKET-1.2] Configuration de l'Architecture Hexagonale (Backend Rust)**
    *   *Description :* Mettre en place la structure de dossiers côté Rust (`src-tauri`) pour séparer le domaine (logique), l'infrastructure (système Windows) et les adaptateurs (API Tauri).
    *   *Critères d'acceptation :* Les dossiers `domain`, `infrastructure` et `application` sont créés avec des fichiers `.gitkeep`.

## Epic 2 : Backend (Tauri / Rust) - Cœur du système
*   **[TICKET-2.1] Récupération du niveau de batterie (Infrastructure)**
    *   *Description :* Implémenter le code Rust nécessaire pour interroger l'API Windows et récupérer le pourcentage de batterie ainsi que l'état de charge (branché/sur batterie).
    *   *Critères d'acceptation :* Une fonction Rust retourne une structure de données propre contenant le taux de charge et le statut de connexion secteur.
*   **[TICKET-2.2] Logique de surveillance (Domaine)**
    *   *Description :* Créer la logique métier qui, à partir de l'information de batterie, détermine si une alerte doit être déclenchée en fonction des seuils (par défaut 50% et 100%).
    *   *Critères d'acceptation :* Des tests unitaires valident le déclenchement des alertes (ex: alerte *brancher* si niveau <= 50%, alerte *débrancher* si niveau >= 100%).
*   **[TICKET-2.3] Cycle de vérification asynchrone**
    *   *Description :* Mettre en place une boucle asynchrone gérée par Tauri qui vérifie l'état de la batterie à intervalles réguliers (ex: toutes les minutes).
    *   *Critères d'acceptation :* L'application logge le niveau de batterie toutes les minutes de manière autonome sans bloquer l'UI.

## Epic 3 : Interface Utilisateur (Astro) & IPC
*   **[TICKET-3.1] Design de la fenêtre principale (Astro)**
    *   *Description :* Créer l'interface minimaliste affichant le pourcentage actuel de la batterie et les paramètres. (Utilisation de CSS natif et HTML, design responsive).
    *   *Critères d'acceptation :* L'UI affiche une maquette statique du niveau de batterie et des champs pour modifier les seuils (100% et 50%).
*   **[TICKET-3.2] Communication IPC Frontend-Backend (État)**
    *   *Description :* Lier le frontend Astro au backend Tauri à l'aide de l'API IPC (Inter-Process Communication) pour récupérer et afficher dynamiquement le niveau de batterie réel.
    *   *Critères d'acceptation :* L'UI affiche le vrai pourcentage de batterie, mis à jour périodiquement.
*   **[TICKET-3.3] Communication IPC Frontend-Backend (Configuration)**
    *   *Description :* Permettre à l'UI d'envoyer les nouveaux seuils d'alerte et l'état de l'autorun au backend.
    *   *Critères d'acceptation :* Les changements de seuils dans l'UI sont persistés par le backend.

## Epic 4 : Intégrations Système (Notifications, Tray, Autorun)
*   **[TICKET-4.1] Fenêtres d'Alerte Personnalisées (Custom Toasts)**
    *   *Description :* Développer des petites fenêtres secondaires en Astro pour afficher les alertes de batterie (Charge maximale / Batterie faible), remplaçant les notifications natives de l'OS.
    *   *Critères d'acceptation :* Une fenêtre pop-up Astro stylisée s'affiche en bas à droite de l'écran lorsque les seuils sont franchis.
*   **[TICKET-4.2] System Tray (Zone de notification)**
    *   *Description :* Ajouter une icône dans la zone de notification Windows permettant de masquer l'application en arrière-plan et offrant un menu contextuel (Ouvrir, Quitter).
    *   *Critères d'acceptation :* L'application tourne en tâche de fond, un clic droit sur l'icône permet de la fermer ou de réafficher la fenêtre principale.
*   **[TICKET-4.3] Lancement au démarrage (Autorun)**
    *   *Description :* Implémenter la fonctionnalité purement système (ex: registre ou raccourci) permettant le lancement de l'application via les API de Tauri (`tauri-plugin-autostart`).
    *   *Critères d'acceptation :* Un toggle depuis l'UI permet d'activer ou désactiver l'exécution de PowerAlert au démarrage de Windows.

## Epic 5 : Finalisation & Packaging
*   **[TICKET-5.1] Persistance de la configuration**
    *   *Description :* Sauvegarder les paramètres de l'utilisateur (seuils d'alerte, état de l'autorun) dans un fichier de configuration stocké dans l'AppData de l'utilisateur.
    *   *Critères d'acceptation :* Si l'utilisateur modifie un seuil et relance l'application, le nouveau seuil est conservé.
*   **[TICKET-5.2] Build et Packaging Windows**
    *   *Description :* Configurer `tauri.conf.json` (nom, icônes, identifiants) et compiler l'application de façon à générer un installateur `.msi` via GitHub Actions ou localement.
    *   *Critères d'acceptation :* Un installeur fonctionnel est généré et peut être testé sur la machine locale.
