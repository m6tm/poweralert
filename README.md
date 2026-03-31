# PowerAlert

PowerAlert est une application desktop moderne et performante conçue pour surveiller l'état de la batterie de votre ordinateur et vous alerter en cas de seuils critiques. Développée avec une architecture robuste, elle allie la légèreté de Tauri à la flexibilité d'Astro.

---

## Table des matières

1. [Aperçu](#aperçu)
2. [Fonctionnalités](#fonctionnalités)
3. [Stack Technique](#stack-technique)
4. [Architecture](#architecture)
5. [Installation](#installation)
6. [Développement](#développement)
7. [Standard de Commit](#standard-de-commit)
8. [Licence](#licence)

---

## Aperçu

PowerAlert permet de prolonger la durée de vie de votre batterie en vous informant du moment opportun pour brancher ou débrancher votre chargeur. Grâce à son interface épurée et ses notifications natives, restez toujours au courant de l'autonomie de votre machine sans effort.

---

## Fonctionnalités

- **Surveillance en temps réel** : Affichage dynamique du pourcentage de batterie.
- **Alertes personnalisables** : Définissez vos propres seuils pour les notifications de batterie faible et de charge maximale.
- **Lancement au démarrage** : Option pour lancer l'application automatiquement avec Windows.
- **Mode Réduit** : Choisissez de démarrer l'application directement dans la barre des tâches (systray).
- **Interface Premium** : Design sombre, responsive et moderne utilisant Astro et CSS natif.
- **Menu de zone de notification** : Accès rapide aux commandes principales via une icône dans la barre des tâches.

---

## Stack Technique

- **Frontend** : [Astro](https://astro.build/) - Framework moderne pour des interfaces rapides.
- **Backend Core** : [Tauri](https://tauri.app/) - Environnement sécurisé et léger pour applications desktop.
- **Langage Système** : [Rust](https://www.rust-lang.org/) - Pour la logique métier et l'accès matériel batterie de haute performance.
- **Styling** : CSS natif (Vanilla CSS) avec une approche centrée sur l'UX.
- **Gestionnaire de paquets** : `pnpm`.

---

## Architecture

Le projet suit les principes de l'Architecture Hexagonale (Ports et Adaptateurs) au niveau du code Rust (`src-tauri`) :

- **Domain** : Entités et logique métier pure (Config, Status de batterie).
- **Application** : Cas d'utilisation (Use Cases) orchestrant la logique.
- **Infrastructure** : Implémentations techniques (Accès matériel batterie, Persistance JSON).

Cette structure garantit un code modulaire, testable et indépendant des frameworks.

---

## Installation

### Prérequis
- [Rust & Cargo](https://rustup.rs/)
- [Node.js](https://nodejs.org/) (version >= 22.12.0)
- [pnpm](https://pnpm.io/)

### Étapes
1. Clonez le dépôt :
   ```bash
   git clone https://github.com/votre-repo/poweralert.git
   cd poweralert
   ```
2. Installez les dépendances :
   ```bash
   pnpm install
   ```

---

## Développement

Pour lancer l'application en mode développement :

```bash
pnpm tauri:dev
```

Pour générer l'installateur (Release) :

```bash
pnpm tauri:build
```

---

## Standard de Commit

Ce projet utilise la norme Conventional Commits pour maintenir un historique clair et structuré :

- `feat`: (nouvelle fonctionnalité)
- `fix`: (correction de bug)
- `docs`: (documentation)
- `style`: (formatage, CSS)
- `refactor`: (modification de code sans changement fonctionnel)
- `chore`: (tâches de maintenance)

---

## Licence

Ce projet est sous licence MIT. Voir le fichier [LICENSE](LICENSE) pour plus de détails.
