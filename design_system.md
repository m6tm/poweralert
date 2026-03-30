# Design System & Charte Graphique - PowerAlert

Ce document définit l'identité visuelle de l'application PowerAlert. L'objectif est de proposer une interface premium, minimaliste et discrète, en accord avec les standards modernes des tableaux de bord de surveillance système.

## 1. Philosophie visuelle (Le Thème)
*   **Minimalisme & Compacité :** L'interface doit aller à l'essentiel. L'information principale (le pourcentage de batterie) doit être lisible instantanément au sein d'une **fenêtre de taille fixe et non redimensionnable**, optimisée pour contenir uniquement le nécessaire.
*   **Mode Sombre (Dark Mode) par défaut :** Pour une application résidant souvent en arrière-plan ou ouverte occasionnellement, le mode sombre offre un confort visuel optimal (et accessoirement, s'aligne bien sur le thème de l'économie d'énergie). Un mode clair (Light Mode) pourra être prévu en alternative.
*   **Glassmorphism subtil :** Utilisation de légers effets de flou (backdrop-blur) et de transparence pour les fonds afin de donner un aspect "premium" intégré au système d'exploitation Windows.

## 2. Palette de Couleurs (Couleurs HSL / HEX)

### Mode Sombre (Principal)
*   **Arrière-plan (Background) :**
    *   Fond principal : `hsl(220, 10%, 12%)` ou `#1c1e24` (Gris très foncé, presque noir).
    *   Fond des conteneurs/cartes : `hsla(220, 10%, 18%, 0.8)` avec un filtre de flou.
*   **Couleur Primaire (Accentuation - Énergie) :**
    *   Vert fluo / Électrique (pour la batterie en charge) : `hsl(145, 80%, 45%)` ou `#16cd63`.
*   **Couleurs d'état (Alertes et Notifications) :**
    *   Alerte "Batterie pleine" (100%) : `hsl(210, 100%, 65%)` ou `#4da8ff` (Bleu lumineux).
    *   Alerte "Batterie faible" (<= 50%) : `hsl(15, 90%, 60%)` ou `#f56c42` (Orange/Rouge dynamique).
    *   État inactif / Grisé : `hsl(220, 10%, 45%)`.
*   **Typographie (Texte) :**
    *   Texte principal (Titres, Pourcentages) : `hsl(0, 0%, 95%)` (Blanc cassé).
    *   Texte secondaire (Labels, Mentions légales) : `hsl(220, 10%, 65%)` (Gris clair).

## 3. Typographie

*   **Famille de Polices Principale :** `Inter`, `Outfit` ou `Roboto` (sans-serif géométrique, propre et très lisible).
*   **Échelle typographique :**
    *   **Titres de l'application :** 24px, Font-weight: 600 (Semi-bold).
    *   **Indicateur Principal (Le pourcentage) :** 64px à 80px, Font-weight: 700 (Bold) - *Doit être l'élément le plus massif de l'interface*.
    *   **Labels (ex: "Seuil d'alerte maximum") :** 14px, Font-weight: 500 (Medium).
    *   **Textes descriptifs / Pied de page :** 12px, Font-weight: 400 (Regular).

## 4. Formes et Structure (Shapes)

*   **Bordures (Border-radius) :**
    *   L'ensemble des éléments interactifs (boutons, champs, cartes) doit utiliser des coins arrondis pour adoucir l'aspect technique du monitoring.
    *   Boutons et conteneurs : `12px`.
    *   Fenêtre principale (si customisée) : `16px`.
*   **Espacements (Padding / Margin) :**
    *   Utiliser une échelle basée sur des multiples de 8 (8px, 16px, 24px, 32px) pour garantir un alignement parfait.

## 5. Animations et Transitions (Micro-interactions)

Pour rendre l'interface "vivante" sans l'alourdir, nous utiliserons les animations suivantes :
*   **Survol (Hover) :** Légère diminution de l'opacité ou éclaircissement subtil de la couleur d'arrière-plan sur les boutons et les switchs (`transition: all 0.2s ease-in-out`).
*   **Apparition de la fenêtre :** Si possible, la fenêtre principale apparaît avec un très léger fondu (`opacity: 0` vers `1` sur 0.3s) ou un glissement vers le haut.
*   **Jauge de Batterie :** Lors de la mise à jour du pourcentage, la barre ou le cercle de chargement doit se remplir de manière fluide et non de façon abrupte (`transition: width/stroke-dashoffset 0.5s ease-out`).
