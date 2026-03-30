# Cahier des Charges - PowerAlert

## 1. Objectif du Projet
Développer une application de bureau légère permettant de préserver la durée de vie de la batterie de l'ordinateur portable en alertant l'utilisateur lorsqu'il doit brancher ou débrancher son chargeur.

## 2. Fonctionnalités Principales
*   **Surveillance du niveau de batterie :** L'application doit lire en temps réel ou à intervalles réguliers (ex: toutes les minutes) le pourcentage de charge de la batterie.
*   **Seuils d'alerte configurables :** L'utilisateur doit pouvoir définir les pourcentages auxquels il souhaite être alerté via l'interface. Les valeurs par défaut seront fixées à :
    *   **Charge maximale (100 % par défaut) :** Déclencher une notification visuelle et/ou sonore pour inciter à débrancher.
    *   **Batterie faible (50 % par défaut) :** Déclencher une notification visuelle et/ou sonore pour inciter à brancher.
*   **Fonctionnement en arrière-plan :** L'application doit pouvoir se réduire dans la zone de notification (system tray) pour ne pas encombrer la barre des tâches.
*   **Autorun configurable :** L'utilisateur doit pouvoir choisir via l'interface s'il souhaite activer ou désactiver le lancement automatique de l'application au démarrage du système (Windows).

## 3. Interface Utilisateur (UI)
*   **Minimaliste et Compacte :** Une fenêtre principale simple, **de taille fixe (non redimensionnable)**, affichant le niveau de batterie actuel et l'état de charge (branché/débranché). La taille doit être idéale pour contenir uniquement le nécessaire sans espace vide superflu.
*   **Zone de notification :** Une icône dans la barre d'état système avec un menu contextuel (Ouvrir, Paramètres, Quitter).
*   **Responsive :** L'interface (basée sur Astro) doit s'adapter correctement à cette taille fixe (et aux différentes résolutions d'écrans en cas de mise à l'échelle DPI).

## 4. Contraintes Techniques
*   **Technologies :**
    *   **Backend / Système :** Tauri (pour la légèreté et l'interaction avec le système).
    *   **Frontend / UI :** Astro (pour une interface utilisateur performante).
*   **Architecture :** Le code sera structuré selon les principes de l'architecture hexagonale (séparation de la logique métier de la récupération des données système et de l'interface graphique).
*   **Système d'exploitation :** Windows/Linux.
*   **Impact limité :** L'application doit consommer un minimum de ressources (CPU et RAM) pour ne pas pénaliser l'autonomie qu'elle est censée protéger.

## 5. Évolutions Possibles (V2)
*   Choix des notifications (sonores, pop-ups système).
*   Historique des cycles de charge.
